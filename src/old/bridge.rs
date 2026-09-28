use std::ptr::copy_nonoverlapping;
use std::sync::atomic::{AtomicI32, AtomicPtr, Ordering};

pub const RUST_WRITE_FLAG: i32 = 0x01;
pub const JAVA_READ_FLAG: i32 = 0x02;

pub mod futex {
    use std::ptr::null as nullptr;
    use std::sync::atomic::AtomicI32;
    use libc::{SYS_futex, FUTEX_WAIT, FUTEX_WAKE};

    pub unsafe fn queue(object: &AtomicI32, target_val: i32) {
        unsafe {
            libc::syscall(
                SYS_futex,
                object,
                FUTEX_WAIT,
                target_val,
                nullptr::<libc::timespec>(),
            );
        }
    }

    pub unsafe fn surrender(object: &AtomicI32) {
        unsafe {
            libc::syscall(
                SYS_futex,
                object,
                FUTEX_WAKE,
                1i32
            );
        }
    }
}

#[allow(non_snake_case)]
pub unsafe fn init_mem(
    LOCK: &'static AtomicPtr<AtomicI32>,
    DATA: &'static AtomicPtr<u8>,
    DATA_SIZE: &'static AtomicI32,
    addr: *mut u8
) -> *mut u8 {
    unsafe {
        LOCK.store(addr as *mut AtomicI32, Ordering::SeqCst);
        DATA.store(addr.add(4), Ordering::SeqCst);
        addr.add(DATA_SIZE.load(Ordering::SeqCst) as usize)
    }
}

#[allow(non_snake_case)]
pub unsafe fn write(
    LOCK: &'static AtomicPtr<AtomicI32>,
    DATA: &'static AtomicPtr<u8>,
    pos: usize,
    bytes: &[u8]
) {
    unsafe {
        let lock_ptr = LOCK.load(Ordering::SeqCst);
        let data_ptr = DATA.load(Ordering::SeqCst);

        if lock_ptr.is_null() || data_ptr.is_null() { return; }

        let lock = &*lock_ptr;

        while lock.compare_exchange_weak(0, RUST_WRITE_FLAG, Ordering::SeqCst, Ordering::Relaxed).is_err() {
            futex::queue(lock, RUST_WRITE_FLAG);
        }

        copy_nonoverlapping(bytes.as_ptr(), data_ptr.add(pos), bytes.len());
        lock.store(0, Ordering::SeqCst);
        futex::surrender(lock);
    }
}

#[macro_export]
macro_rules! jni_futex_handler {
    ($fn_name_1:ident, $fn_name_2:ident, $lock:expr) => {
        use crate::bridge::futex::{queue, surrender};
        use crate::bridge::JAVA_READ_FLAG;
        use jni::EnvUnowned;
        use jni::objects::JClass;
        use std::sync::atomic::Ordering;

        #[unsafe(no_mangle)]
        pub unsafe extern "system" fn $fn_name_1<'local>(
            _env: EnvUnowned<'local>,
            _class: JClass<'local>
        ) {
            unsafe {
                let lock_ptr = $lock.load(Ordering::Relaxed);
                if lock_ptr.is_null() { queue(&*lock_ptr, JAVA_READ_FLAG) }
            }
        }

        #[unsafe(no_mangle)]
        pub unsafe extern "system" fn $fn_name_2<'local>(
            _env: EnvUnowned<'local>,
            _class: JClass<'local>
        ) {
            unsafe {
                let lock_ptr = $lock.load(Ordering::Relaxed);
                if lock_ptr.is_null() { surrender(&*lock_ptr) }
            }
        }
    };
}

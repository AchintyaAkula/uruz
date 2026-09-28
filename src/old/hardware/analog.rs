use std::sync::atomic::{AtomicI32, AtomicPtr};
use crate::jni_futex_handler;

static LOCK: AtomicPtr<AtomicI32> = AtomicPtr::new(core::ptr::null_mut());
static ANALOG_DATA: AtomicPtr<u8> = AtomicPtr::new(core::ptr::null_mut());
static DATA_SIZE: AtomicI32 = AtomicI32::new(0);

jni_futex_handler!(
    Java_dev_achintyaakula_uruz_Analog_applyForAccess,
    Java_dev_achintyaakula_uruz_Analog_surrenderAccess,
    LOCK
);
use std::sync::atomic::{AtomicI32, AtomicPtr};
use crate::lynx::{Packet, Priority};
use crossbeam_channel::Sender;
use std::sync::OnceLock;
use crate::jni_futex_handler;

const I2C_CHANNEL: OnceLock<Sender<(Packet, Priority)>> = OnceLock::new();

static LOCK: AtomicPtr<AtomicI32> = AtomicPtr::new(core::ptr::null_mut());
static I2C_DATA: AtomicPtr<u8> = AtomicPtr::new(core::ptr::null_mut());
static DATA_SIZE: AtomicI32 = AtomicI32::new(0);

jni_futex_handler!(
    Java_dev_achintyaakula_uruz_I2cManager_applyForAccess,
    Java_dev_achintyaakula_uruz_I2cManager_surrenderAccess,
    LOCK
);

#[allow(non_snake_case)]
pub unsafe extern "system" fn Java_dev_achintyaakula_uruz_I2cManager_writeSingleByte(
    _env: EnvUnowned,
    _class: JClass,

) {

}

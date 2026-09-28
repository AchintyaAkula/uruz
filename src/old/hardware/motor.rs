use std::sync::atomic::{AtomicI32, AtomicPtr};
use crate::lynx::{Packet, Priority};
use crossbeam_channel::Sender;
use std::sync::OnceLock;
use crate::jni_futex_handler;

const MOTOR_CHANNEL: OnceLock<Sender<(Packet, Priority)>> = OnceLock::new();

static LOCK: AtomicPtr<AtomicI32> = AtomicPtr::new(core::ptr::null_mut());
static MOTOR_DATA: AtomicPtr<u8> = AtomicPtr::new(core::ptr::null_mut());
static DATA_SIZE: AtomicI32 = AtomicI32::new(0);

jni_futex_handler!(
    Java_dev_achintyaakula_uruz_MotorManager_applyForAccess,
    Java_dev_achintyaakula_uruz_MotorManager_surrenderAccess,
    LOCK
);

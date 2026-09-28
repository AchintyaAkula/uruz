use crate::lynx::{Packet, Priority};
use crossbeam_channel::Sender;
use std::sync::OnceLock;

const SERVO_CHANNEL: OnceLock<Sender<(Packet, Priority)>> = OnceLock::new();

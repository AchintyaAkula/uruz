use crate::lynx::packet::Packet;
use crate::lynx::util::MotorPIDCoeffs;

pub fn bulk_read(t_addr: u8) -> Packet {
    Packet::new(t_addr, 0x00, Vec::new())
}

pub fn set_digital_output(t_addr: u8, pin: u8, value: bool) -> Packet {
    Packet::new(t_addr, 0x01, vec![pin, value as u8])
}

pub fn set_digital_output_all(t_addr: u8, values: u8) -> Packet {
    Packet::new(t_addr, 0x02, vec![values])
}

pub fn set_digital_mode(t_addr: u8, pin: u8, mode: u8) -> Packet {
    Packet::new(t_addr, 0x03, vec![pin, mode])
}

pub fn get_digital_mode(t_addr: u8, pin: u8) -> Packet {
    Packet::new(t_addr, 0x04, vec![pin])
}

pub fn get_digital_input(t_addr: u8, pin: u8) -> Packet {
    Packet::new(t_addr, 0x05, vec![pin])
}

pub fn get_digital_input_all(t_addr: u8) -> Packet {
    Packet::new(t_addr, 0x06, Vec::new())
}

pub fn get_adc_value(t_addr: u8, channel: u8, config: u8) -> Packet {
    Packet::new(t_addr, 0x07, vec![channel, config])
}

pub fn set_motor_config(t_addr: u8, port: u8, mode: u8, zpb: u8) -> Packet {
    Packet::new(t_addr, 0x08, vec![port, mode, zpb])
}

pub fn get_motor_config(t_addr: u8, port: u8) -> Packet {
    Packet::new(t_addr, 0x09, vec![port])
}

pub fn set_motor_activation(t_addr: u8, port: u8, active: bool) -> Packet {
    Packet::new(t_addr, 0x0a, vec![port, active as u8])
}

pub fn get_motor_activation(t_addr: u8, port: u8) -> Packet {
    Packet::new(t_addr, 0x0b, vec![port])
}

pub fn set_motor_current_alert(t_addr: u8, port: u8, current: i16) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(3);
    data.push(port);
    data.extend_from_slice(&current.to_le_bytes());
    Packet::new(t_addr, 0x0c, data)
}

pub fn get_motor_current_alert(t_addr: u8, port: u8) -> Packet {
    Packet::new(t_addr, 0x0d, vec![port])
}

pub fn reset_motor_encoder(t_addr: u8, port: u8) -> Packet {
    Packet::new(t_addr, 0x0e, vec![port])
}

pub fn set_motor_power(t_addr: u8, port: u8, power: i16) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(3);
    data.push(port);
    data.extend_from_slice(&power.to_le_bytes());
    Packet::new(t_addr, 0x0f, data)
}

pub fn get_motor_power(t_addr: u8, port: u8) -> Packet {
    Packet::new(t_addr, 0x10, vec![port])
}

pub fn set_motor_target_vel(t_addr: u8, port: u8, vel: i16) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(3);
    data.push(port);
    data.extend_from_slice(&vel.to_le_bytes());
    Packet::new(t_addr, 0x11, data)
}

pub fn get_motor_target_vel(t_addr: u8, port: u8) -> Packet {
    Packet::new(t_addr, 0x12, vec![port])
}

pub fn set_motor_target_pos(t_addr: u8, port: u8, pos: i32, tolerance: i16) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(7);
    data.push(port);
    data.extend_from_slice(&pos.to_le_bytes());
    data.extend_from_slice(&tolerance.to_le_bytes());
    Packet::new(t_addr, 0x13, data)
}

pub fn get_motor_target_pos(t_addr: u8, port: u8) -> Packet {
    Packet::new(t_addr, 0x14, vec![port])
}

pub fn get_motor_target_status(t_addr: u8, port: u8) -> Packet {
    Packet::new(t_addr, 0x15, vec![port])
}

pub fn get_motor_position(t_addr: u8, port: u8) -> Packet {
    Packet::new(t_addr, 0x16, vec![port])
}

pub fn set_motor_pid_coeffs(t_addr: u8, port: u8, mode: u8, coeffs: MotorPIDCoeffs) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(14);
    data.push(port);
    data.push(mode);
    data.extend_from_slice(&coeffs.export());
    Packet::new(t_addr, 0x17, data)
}

pub fn get_motor_pid_coeffs(t_addr: u8, port: u8, mode: u8) -> Packet {
    Packet::new(t_addr, 0x18, vec![port, mode])
}

pub fn set_pwm_config(t_addr: u8, channel: u8, period: i16) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(3);
    data.push(channel);
    data.extend_from_slice(&period.to_le_bytes());
    Packet::new(t_addr, 0x19, data)
}

pub fn get_pwm_config(t_addr: u8, channel: u8) -> Packet {
    Packet::new(t_addr, 0x1a, vec![channel])
}

pub fn set_pulse_width(t_addr: u8, channel: u8, width_us: i16) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(3);
    data.push(channel);
    data.extend_from_slice(&width_us.to_le_bytes());
    Packet::new(t_addr, 0x1b, data)
}

pub fn get_pulse_width(t_addr: u8, channel: u8) -> Packet {
    Packet::new(t_addr, 0x1c, vec![channel])
}

pub fn set_pwm_activation(t_addr: u8, channel: u8, active: bool) -> Packet {
    Packet::new(t_addr, 0x1d, vec![channel, active as u8])
}

pub fn get_pwm_activation(t_addr: u8, channel: u8) -> Packet {
    Packet::new(t_addr, 0x1e, vec![channel])
}

pub fn set_servo_pwm_config(t_addr: u8, port: u8, period: i16) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(3);
    data.push(port);
    data.extend_from_slice(&period.to_le_bytes());
    Packet::new(t_addr, 0x1f, data)
}

pub fn get_servo_pwm_config(t_addr: u8, port: u8) -> Packet {
    Packet::new(t_addr, 0x20, vec![port])
}

pub fn set_servo_pw(t_addr: u8, port: u8, width_us: i16) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(3);
    data.push(port);
    data.extend_from_slice(&width_us.to_le_bytes());
    Packet::new(t_addr, 0x21, data)
}

pub fn get_servo_pw(t_addr: u8, port: u8) -> Packet {
    Packet::new(t_addr, 0x22, vec![port])
}

pub fn set_servo_activation(t_addr: u8, port: u8, active: bool) -> Packet {
    Packet::new(t_addr, 0x23, vec![port, active as u8])
}

pub fn get_servo_activation(t_addr: u8, port: u8) -> Packet {
    Packet::new(t_addr, 0x24, vec![port])
}

pub fn i2c_write_single(t_addr: u8, bus: u8, addr_7_bit: u8, value: u8) -> Packet {
    Packet::new(t_addr, 0x25, vec![bus, addr_7_bit, value])
}

pub fn i2c_write_many(t_addr: u8, bus: u8, addr_7_bit: u8, values: Vec<u8>) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(3 + values.len());
    data.push(bus);
    data.push(addr_7_bit);
    data.push(values.len() as u8);
    data.extend_from_slice(&values);
    Packet::new(t_addr, 0x26, data)
}

pub fn i2c_read_single(t_addr: u8, bus: u8, addr_7_bit: u8) -> Packet {
    Packet::new(t_addr, 0x27, vec![bus, addr_7_bit])
}

pub fn i2c_read_many(t_addr: u8, bus: u8, addr_7_bit: u8, bytes_to_read: u8) -> Packet {
    Packet::new(t_addr, 0x28, vec![bus, addr_7_bit, bytes_to_read])
}

pub fn i2c_read_followup(t_addr: u8, bus: u8) -> Packet {
    Packet::new(t_addr, 0x29, vec![bus])
}

pub fn i2c_write_followup(t_addr: u8, bus: u8) -> Packet {
    Packet::new(t_addr, 0x2a, vec![bus])
}

pub fn i2c_configure_channel(t_addr: u8, bus: u8, speed: u8) -> Packet {
    Packet::new(t_addr, 0x2b, vec![bus, speed])
}

pub fn phone_charge_control(t_addr: u8, enabled: bool) -> Packet {
    Packet::new(t_addr, 0x2c, vec![enabled as u8])
}

pub fn get_phone_charge(t_addr: u8) -> Packet {
    Packet::new(t_addr, 0x2d, Vec::new())
}

pub fn inject_datalog_hint() -> Packet {
    todo!("Ya, im not figuring out this string stuff")
}

pub fn i2c_get_channel_config(t_addr: u8, bus: u8) -> Packet {
    Packet::new(t_addr, 0x2f, vec![bus])
}

pub fn read_version(t_addr: u8) -> Packet {
    Packet::new(t_addr, 0x30, Vec::new())
}

pub fn ftdi_reset_control(t_addr: u8, enabled: bool) -> Packet {
    Packet::new(t_addr, 0x31, vec![enabled as u8])
}

pub fn get_ftdi_reset(t_addr: u8) -> Packet {
    Packet::new(t_addr, 0x32, Vec::new())
}

pub fn set_motor_pidf_coeffs(
    t_addr: u8,
    port: u8,
    mode: u8,
    pid_coeffs: MotorPIDCoeffs,
    f: i32,
    alg: u8,
) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(19);
    data.push(port);
    data.push(mode);
    data.extend_from_slice(&pid_coeffs.export());
    data.extend_from_slice(&f.to_le_bytes());
    data.push(alg);
    Packet::new(t_addr, 0x33, data)
}

pub fn i2c_write_read_many(
    t_addr: u8,
    bus: u8,
    addr_7_bit: u8,
    start_addr: u8,
    bytes_to_read: u8,
) -> Packet {
    Packet::new(
        t_addr,
        0x34,
        vec![bus, addr_7_bit, bytes_to_read, start_addr],
    )
}

pub fn get_motor_pidf_coeffs(t_addr: u8, port: u8, mode: u8) -> Packet {
    Packet::new(t_addr, 0x35, vec![port, mode])
}

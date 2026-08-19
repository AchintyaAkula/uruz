use super::packet::Packet;

pub fn bulk_read(t_addr: u8) -> Packet {
    Packet::new(t_addr, 0x00, Vec::new())
}

pub fn set_digital_output(t_addr: u8, pin: u8, value: bool) -> Packet {
    Packet::new(t_addr, 0x01, vec![pin, value as u8])
}

pub fn set_digital_output_all(t_addr: u8, values: u8) -> Packet {
    Packet::new(t_addr, 0x02, vec![values])
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum DigitalMode {
    Input = 0x00,
    Output = 0x01,
}

pub fn set_digital_mode(t_addr: u8, pin: u8, mode: DigitalMode) -> Packet {
    Packet::new(t_addr, 0x03, vec![pin, mode as u8])
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCChannel {
    Analog0 = 0x00,
    Analog1 = 0x01,
    Analog2 = 0x02,
    Analog3 = 0x03,
    DigitalBus = 0x04,
    I2cBus = 0x05,
    ServoBus = 0x06,
    BatteryCurrent = 0x07,
    Motor0 = 0x08,
    Motor1 = 0x09,
    Motor2 = 0x0a,
    Motor3 = 0x0b,
    FiveVoltBus = 0x0c,
    BatteryMonitor = 0x0d,
    Temperature = 0x0e,
    Unknown = 0xff,
}

impl ADCChannel {
    pub fn analog(port: u8) -> ADCChannel {
        match port {
            0 => ADCChannel::Analog0,
            1 => ADCChannel::Analog1,
            2 => ADCChannel::Analog2,
            3 => ADCChannel::Analog3,
            _ => ADCChannel::Unknown,
        }
    }

    pub fn motor(port: u8) -> ADCChannel {
        match port {
            0 => ADCChannel::Motor0,
            1 => ADCChannel::Motor1,
            2 => ADCChannel::Motor2,
            3 => ADCChannel::Motor3,
            _ => ADCChannel::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum DataConfig {
    Filtered = 0x00,
    Raw = 0x01,
}

pub fn get_current(t_addr: u8, channel: ADCChannel, config: DataConfig) -> Packet {
    Packet::new(t_addr, 0x07, vec![channel as u8, config as u8])
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum MotorMode {
    RawPower = 0,
    VelocityControl = 1,
    PositionalControl = 2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum MotorZPB {
    Brake = 0,
    Float = 1,
}

pub fn set_motor_config(t_addr: u8, port: u8, mode: MotorMode, zpb: MotorZPB) -> Packet {
    Packet::new(t_addr, 0x08, vec![port, mode as u8, zpb as u8])
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

pub struct MotorPIDCoeffs {
    pub p: i32,
    pub i: i32,
    pub d: i32,
}

impl MotorPIDCoeffs {
    pub fn export(&self) -> Vec<u8> {
        [
            self.p.to_le_bytes().as_slice(),
            self.i.to_le_bytes().as_slice(),
            self.d.to_le_bytes().as_slice(),
        ]
        .concat()
    }
}

pub fn set_motor_pid_coeffs(t_addr: u8, port: u8, mode: MotorMode, coeffs: MotorPIDCoeffs) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(14);
    data.push(port);
    data.push(mode as u8);
    data.extend_from_slice(&coeffs.export());
    Packet::new(t_addr, 0x17, data)
}

pub fn get_motor_pid_coeffs(t_addr: u8, port: u8, mode: MotorMode) -> Packet {
    Packet::new(t_addr, 0x18, vec![port, mode as u8])
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

pub struct I2cAddress {
    pub(crate) seven_bit: u8,
}

impl I2cAddress {
    pub fn new(addr: u8) -> Self {
        Self {
            seven_bit: (addr & 0x7F),
        }
    }

    pub fn from_8b(addr: u8) -> Self {
        Self {
            seven_bit: (addr >> 1),
        }
    }

    pub fn to_8b(&self) -> u8 {
        self.seven_bit << 1
    }
}

pub fn i2c_write_single(t_addr: u8, bus: u8, addr: I2cAddress, value: u8) -> Packet {
    Packet::new(t_addr, 0x25, vec![bus, addr.seven_bit, value])
}

pub fn i2c_write_many(t_addr: u8, bus: u8, addr: I2cAddress, values: Vec<u8>) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(3 + values.len());
    data.push(bus);
    data.push(addr.seven_bit);
    data.push(values.len() as u8);
    data.extend_from_slice(&values);
    Packet::new(t_addr, 0x26, data)
}

pub fn i2c_read_single(t_addr: u8, bus: u8, addr: I2cAddress) -> Packet {
    Packet::new(t_addr, 0x27, vec![bus, addr.seven_bit])
}

pub fn i2c_read_many(t_addr: u8, bus: u8, addr: I2cAddress, bytes_to_read: u8) -> Packet {
    Packet::new(t_addr, 0x28, vec![bus, addr.seven_bit, bytes_to_read])
}

pub fn i2c_read_followup(t_addr: u8, bus: u8) -> Packet {
    Packet::new(t_addr, 0x29, vec![bus])
}

pub fn i2c_write_followup(t_addr: u8, bus: u8) -> Packet {
    Packet::new(t_addr, 0x2a, vec![bus])
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum I2cSpeed {
    Standard = 0x00,  // 100 kHz
    Fast = 0x01,      // 400 kHz
    SuperFast = 0x02, // 1 MHz
    Uruz = 0x03,      // 4 MHz
    Unknown = 0xff,
}

pub fn i2c_configure_channel(t_addr: u8, bus: u8, speed: I2cSpeed) -> Packet {
    Packet::new(t_addr, 0x2b, vec![bus, speed as u8])
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum MotorControlAlgorithm {
    LegacyPID = 0x00,
    PIDF = 0x01,
    Max = 0x02,
    NotSet = 0xff,
}

pub fn set_motor_pidf_coeffs(
    t_addr: u8,
    port: u8,
    mode: MotorMode,
    pid_coeffs: MotorPIDCoeffs,
    f: i32,
    alg: MotorControlAlgorithm,
) -> Packet {
    let mut data: Vec<u8> = Vec::with_capacity(19);
    data.push(port);
    data.push(mode as u8);
    data.extend_from_slice(&pid_coeffs.export());
    data.extend_from_slice(&f.to_le_bytes());
    data.push(alg as u8);
    Packet::new(t_addr, 0x33, data)
}

pub fn i2c_write_read_many(
    t_addr: u8,
    bus: u8,
    addr: I2cAddress,
    start_addr: u8,
    bytes_to_read: u8,
) -> Packet {
    Packet::new(
        t_addr,
        0x34,
        vec![bus, addr.seven_bit, bytes_to_read, start_addr],
    )
}

pub fn get_motor_pidf_coeffs(t_addr: u8, port: u8, mode: MotorMode) -> Packet {
    Packet::new(t_addr, 0x35, vec![port, mode as u8])
}

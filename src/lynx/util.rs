use std::convert::From;

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
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
    #[default]
    Unknown = 0xff,
}

impl ADCChannel {
    pub fn analog(port: u8) -> ADCChannel {
        ADCChannel::from(port)
    }

    pub fn motor(port: u8) -> ADCChannel {
        ADCChannel::from(port + 8)
    }
}

impl From<u8> for ADCChannel {
    fn from(value: u8) -> Self {
        match value {
            0x00 => ADCChannel::Analog0,
            0x01 => ADCChannel::Analog1,
            0x02 => ADCChannel::Analog2,
            0x03 => ADCChannel::Analog3,
            0x04 => ADCChannel::DigitalBus,
            0x05 => ADCChannel::I2cBus,
            0x06 => ADCChannel::ServoBus,
            0x07 => ADCChannel::BatteryCurrent,
            0x08 => ADCChannel::Motor0,
            0x09 => ADCChannel::Motor1,
            0x0a => ADCChannel::Motor2,
            0x0b => ADCChannel::Motor3,
            0x0c => ADCChannel::FiveVoltBus,
            0x0d => ADCChannel::BatteryMonitor,
            0x0e => ADCChannel::Temperature,
            _ => ADCChannel::default(),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCDataConfig {
    #[default]
    Filtered = 0x00,
    Raw = 0x01,
}

impl From<u8> for ADCDataConfig {
    fn from(value: u8) -> Self {
        match value {
            0x00 => ADCDataConfig::Filtered,
            0x01 => ADCDataConfig::Raw,
            _ => ADCDataConfig::default(),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum DigitalMode {
    #[default]
    Input = 0x00,
    Output = 0x01,
}

impl From<u8> for DigitalMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => DigitalMode::Input,
            0x01 => DigitalMode::Output,
            _ => DigitalMode::default(),
        }
    }
}

pub struct I2cAddress {
    pub seven_bit: u8,
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

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum I2cSpeed {
    Standard = 0x00,  // 100 kHz
    Fast = 0x01,      // 400 kHz
    SuperFast = 0x02, // 1 MHz
    Uruz = 0x03,      // 4 MHz
    #[default]
    Unknown = 0xff,
}

impl From<u8> for I2cSpeed {
    fn from(value: u8) -> Self {
        match value {
            0x00 => I2cSpeed::Standard,
            0x01 => I2cSpeed::Fast,
            0x02 => I2cSpeed::SuperFast,
            0x03 => I2cSpeed::Uruz,
            _ => I2cSpeed::default(),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum MotorControlAlgorithm {
    LegacyPID = 0x00,
    PIDF = 0x01,
    Max = 0x02,
    #[default]
    NotSet = 0xff,
}

impl From<u8> for MotorControlAlgorithm {
    fn from(value: u8) -> Self {
        match value {
            0x00 => MotorControlAlgorithm::LegacyPID,
            0x01 => MotorControlAlgorithm::PIDF,
            0x02 => MotorControlAlgorithm::Max,
            _ => MotorControlAlgorithm::default(),
        }
    }
}

pub struct MotorPIDCoeffs {
    pub p: i32,
    pub i: i32,
    pub d: i32,
}

impl MotorPIDCoeffs {
    pub fn new(p: i32, i: i32, d: i32) -> Self {
        MotorPIDCoeffs { p, i, d }
    }

    pub fn empty() -> Self {
        MotorPIDCoeffs::new(0i32, 0i32, 0i32)
    }

    pub fn export(&self) -> Vec<u8> {
        [
            self.p.to_le_bytes().as_slice(),
            self.i.to_le_bytes().as_slice(),
            self.d.to_le_bytes().as_slice(),
        ]
        .concat()
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum MotorMode {
    #[default]
    RawPower = 0x00,
    VelocityControl = 0x01,
    PositionalControl = 0x02,
}

impl From<u8> for MotorMode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => MotorMode::RawPower,
            0x01 => MotorMode::VelocityControl,
            0x02 => MotorMode::PositionalControl,
            _ => MotorMode::default(),
        }
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum MotorZPB {
    Brake = 0x00,
    #[default]
    Float = 0x01,
}

impl From<u8> for MotorZPB {
    fn from(value: u8) -> Self {
        match value {
            0x00 => MotorZPB::Brake,
            _ => MotorZPB::default(),
        }
    }
}

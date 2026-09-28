const RESPONSE_FLAG: u16 = 0x8000;

pub trait Command: Into<u16> {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum Digital {
    SetOutput    = 0x0001,
    SetOutputAll = 0x0002,
    SetMode      = 0x0003,
    GetMode      = 0x0004,
    GetInput     = 0x0005,
    GetInputAll  = 0x0006,
}

impl From<Digital> for u16 {
    fn from(digital: Digital) -> u16 { digital as u16 }
}

impl Command for Digital {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum PWM {
    SetConfig = 0x0019,
    GetConfig = 0x001A,
    SetTarget = 0x001B,
    GetTarget = 0x001C,
    Enable    = 0x001D,
    Disable   = 0x001E,
}

impl From<PWM> for u16 {
    fn from(pwm: PWM) -> u16 { pwm as u16 }
}

impl Command for PWM {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum Servo {
    SetConfig = 0x001F,
    GetConfig = 0x0020,
    SetTarget = 0x0021,
    GetTarget = 0x0022,
    Enable    = 0x0023,
    Disable   = 0x0024,
}

impl From<Servo> for u16 {
    fn from(servo: Servo) -> u16 { servo as u16 }
}

impl Command for Servo {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum Motor {
    SetMode              = 0x0008,
    GetMode              = 0x0009,
    Enable               = 0x000A,
    Disable              = 0x000B,
    SetCurrentAlertLevel = 0x000C,
    GetCurrentAlertLevel = 0x000D,
    ResetEncoder         = 0x000E,
    SetPower             = 0x000F,
    GetPower             = 0x0010,
    SetTargetVelocity    = 0x0011,
    GetTargetVelocity    = 0x0012,
    SetTargetPosition    = 0x0013,
    GetTargetPosition    = 0x0014,
    GetTargetStatus      = 0x0015,
    GetPosition          = 0x0016,
    SetPIDCoeffs         = 0x0017,
    GetPIDCoeffs         = 0x0018,
    SetPIDFCoeffs        = 0x0033,
    GetPIDFCoeffs        = 0x0035,
}

impl From<Motor> for u16 {
    fn from(motor: Motor) -> u16 { motor as u16 }
}

impl Command for Motor {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum I2c {
    WriteSingle         = 0x0025,
    WriteMultiple       = 0x0026,
    ReadSingle          = 0x0027,
    ReadMultiple        = 0x0028,
    RetrieveReadStatus  = 0x0029,
    RetrieveWriteStatus = 0x002A,
    SetSpeedConfig      = 0x002B,
    GetSpeedConfig      = 0x002F,
    ReadWriteMultiple   = 0x0034,
}

impl From<I2c> for u16 {
    fn from(i2c: I2c) -> u16 { i2c as u16 }
}

impl Command for I2c {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum Lynx {
    GetBulkData    = 0x0000,
    GetADCData     = 0x0007,
    SetPhoneCharge = 0x002C,
    GetPhoneCharge = 0x002D,
    DataLogHints   = 0x002E,
    ReadVersion    = 0x0030,
    SetFTDIReset   = 0x0031,
    GetFTDIReset   = 0x0032,
}

impl From<Lynx> for u16 {
    fn from(lynx: Lynx) -> u16 { lynx as u16 }
}

impl Command for Lynx {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum System {
    Ack                 = 0x7f01,
    Nack                = 0x7f02,
    GetModuleStatus     = 0x7f03,
    KeepAlive           = 0x7f04,
    FailSafe            = 0x7f05,
    SetNewModuleAddr    = 0x7f06,
    QueryInterface      = 0x7f07,
    StartDownload       = 0x7f08,
    DownloadChunk       = 0x7f09,
    SetModuleLEDColor   = 0x7f0a,
    GetModuleLEDColor   = 0x7f0b,
    SetModuleLEDPattern = 0x7f0c,
    GetModuleLEDPattern = 0x7f0d,
    DebugLogLevel       = 0x7f0e,
    Discovery           = 0x7f0f,
}

impl From<System> for u16 {
    fn from(system: System) -> u16 { system as u16 }
}

impl Command for System {}
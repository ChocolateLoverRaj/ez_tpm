use num_enum::IntoPrimitive;

#[repr(u32)]
#[derive(Debug, IntoPrimitive)]
pub enum TpmCommandCode {
    PcrRead = 0x0000017E,
}

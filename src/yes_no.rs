use num_enum::TryFromPrimitive;

/// `TPMI_YES_NO`
/// This matches how a Rust `bool` is stored, but in case the value is invalid, we don't want to assume it is a valid `bool`.
#[repr(u8)]
#[derive(Debug, TryFromPrimitive)]
pub enum YesNo {
    No,
    Yes,
}

impl From<YesNo> for bool {
    fn from(value: YesNo) -> Self {
        match value {
            YesNo::No => false,
            YesNo::Yes => true,
        }
    }
}

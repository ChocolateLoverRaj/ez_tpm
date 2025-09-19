use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
pub struct TpmCommand<Inputs> {
    pub header: CommandHeader,
    pub inputs: Inputs,
}

#[repr(C)]
#[derive(Debug, Immutable, IntoBytes, Unaligned, FromBytes, KnownLayout)]
pub struct CommandHeader {
    pub tag: [u8; 2],
    pub command_size: [u8; 4],
    pub command_code: [u8; 4],
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
pub struct ResponseHeader {
    pub tag: [u8; 2],
    pub response_size: [u8; 4],
    pub response_code: [u8; 4],
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
pub struct TpmResponse<Outputs> {
    pub header: ResponseHeader,
    pub outputs: Outputs,
}

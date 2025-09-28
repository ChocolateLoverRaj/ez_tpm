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

#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
#[repr(C)]
pub struct Tpm2bDigest<const BYTE_LEN: usize> {
    pub size: [u8; 2],
    pub buffer: [u8; BYTE_LEN],
}

impl Tpm2bDigest<0> {
    pub const EMPTY: Self = Self {
        size: 0_u16.to_be_bytes(),
        buffer: [],
    };
}

#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
#[repr(C)]
pub struct TpmlPcrSelection<const N_ALGORITHMS: usize, const N_PCR_BITMAP_BYTES: usize> {
    pub count: [u8; 4],
    pub pcr_selections: [TpmsPcrSelection<N_PCR_BITMAP_BYTES>; N_ALGORITHMS],
}

#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
#[repr(C)]
pub struct TpmsPcrSelection<const N: usize> {
    pub hash: [u8; 2],
    /// Size in bytes of the pcr_select bitmap
    pub size_of_select: u8,
    /// Bitmap of selected PCRs
    pub pcr_select: [u8; N],
}

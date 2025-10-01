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

/// There are many TPM structs which can be a variable len, and are encoded with a u16 (big endian) followed by the bytes.
/// This Rust struct can be used for all of them
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
#[repr(C)]
pub struct Tpm2Buffer<const BYTE_LEN: usize> {
    pub size: [u8; 2],
    pub buffer: [u8; BYTE_LEN],
}

impl Tpm2Buffer<0> {
    pub const EMPTY: Self = Self {
        size: 0_u16.to_be_bytes(),
        buffer: [],
    };
}

impl<const BYTE_LEN: usize> Tpm2Buffer<BYTE_LEN> {
    pub fn new(bytes: &[u8; BYTE_LEN]) -> Self {
        Self {
            size: (BYTE_LEN as u16).to_be_bytes(),
            buffer: *bytes,
        }
    }
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
    /// Size in bytes of the pcr_sesesslect bitmap
    pub size_of_select: u8,
    /// Bitmap of selected PCRs
    pub pcr_select: [u8; N],
}

#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
#[repr(C)]
pub struct TpmsAuthCommand<const NONCE_LEN: usize, const HMAC_LEN: usize> {
    pub session_handle: [u8; 4],
    pub nonce: Tpm2Buffer<NONCE_LEN>,
    pub session_attributes: u8,
    /// either an HMAC, a password, or an EmptyAuth
    pub hmac: Tpm2Buffer<HMAC_LEN>,
}

#![no_std]

mod command_code;
mod get_random;
mod pcr_read;
#[cfg(feature = "uefi")]
pub mod uefi;

use command_code::*;
pub use get_random::*;
pub use pcr_read::*;

// Construct command input from Rust input
// Construct output buffer
// Send command
// Decode output buffer's response header
// If successful, decode the command-specific output

use num_enum::{IntoPrimitive, TryFromPrimitive};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

pub trait Command {
    type Output: ?Sized;

    fn input_and_output(&mut self) -> (&[u8], &mut [u8]);
    /// Note that this function can be called if there is a command-specific error.
    fn process_output<'a>(
        response_header: &'a mut ResponseHeader,
        parameters: &'a mut [u8],
    ) -> &'a Self::Output;
}

#[repr(C)]
#[derive(Debug, Immutable, IntoBytes, Unaligned, FromBytes, KnownLayout)]
struct CommandHeader {
    tag: [u8; 2],
    command_size: [u8; 4],
    command_code: [u8; 4],
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
pub struct ResponseHeader {
    tag: [u8; 2],
    response_size: [u8; 4],
    response_code: [u8; 4],
}

const RC_WARN: u32 = 0x900;

/// The response code field is a `u32`. `0` means `Ok`, non-zero means `Err`. This enum only contains some of the `Err` variants.
#[non_exhaustive]
#[repr(u32)]
#[derive(Debug, TryFromPrimitive)]
pub enum ResponseCode {
    TpmRcCanceled = RC_WARN + 0x009,
}

const TPM_ST_NO_SESSIONS: u16 = 0x8001;

#[repr(u16)]
#[derive(Debug, IntoPrimitive)]
pub enum TpmAlgId {
    Sha1 = 0x0004,
    Sha256 = 0x000B,
    Sha384 = 0x000C,
    Sha512 = 0x000D,
}

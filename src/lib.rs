#![no_std]

mod capability;
mod command;
mod command_code;
mod create;
mod create_primary;
mod get_capability;
mod get_random;
mod pcr_read;
mod structs;
mod tpm_ht;
mod tpm_sts;
#[cfg(feature = "uefi")]
pub mod uefi;
mod yes_no;

use capability::*;
pub use command::*;
use command_code::*;
pub use create::*;
use create_primary::*;
pub use get_capability::*;
pub use get_random::*;
pub use pcr_read::*;
pub use structs::*;
use tpm_ht::*;
use tpm_sts::*;
use yes_no::*;

// Construct command input from Rust input
// Construct output buffer
// Send command
// Decode output buffer's response header
// If successful, decode the command-specific output

use num_enum::{IntoPrimitive, TryFromPrimitive};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

pub trait Command<'a> {
    type Output;

    fn input_and_output(&mut self) -> (&[u8], &mut [u8]);
    /// Note that this function can be called if there is a command-specific error.
    fn process_output(
        response_header: &'a mut ResponseHeader,
        parameters: &'a mut [u8],
    ) -> Self::Output;
}

const RC_WARN: u32 = 0x900;

/// The response code field is a `u32`. `0` means `Ok`, non-zero means `Err`. This enum only contains some of the `Err` variants.
#[non_exhaustive]
#[repr(u32)]
#[derive(Debug, TryFromPrimitive)]
pub enum ResponseCode {
    TpmRcCanceled = RC_WARN + 0x009,
}

#[repr(u16)]
#[derive(Debug, IntoPrimitive)]
pub enum TpmAlgId {
    Sha1 = 0x0004,
    Sha256 = 0x000B,
    Sha384 = 0x000C,
    Sha512 = 0x000D,
}

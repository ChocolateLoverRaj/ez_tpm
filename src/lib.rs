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
mod tpm_alg_id;
mod tpm_ht;
mod tpm_rh;
mod tpm_sts;
mod tpma_object;
#[cfg(feature = "uefi")]
pub mod uefi;
mod yes_no;

use capability::*;
pub use command::*;
use command_code::*;
pub use create::*;
pub use create_primary::*;
pub use get_capability::*;
pub use get_random::*;
pub use pcr_read::*;
pub use structs::*;
use tpm_alg_id::*;
use tpm_ht::*;
use tpm_rh::*;
use tpm_sts::*;
use tpma_object::*;
use yes_no::*;

// Construct command input from Rust input
// Construct output buffer
// Send command
// Decode output buffer's response header
// If successful, decode the command-specific output

use num_enum::{IntoPrimitive, TryFromPrimitive};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

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

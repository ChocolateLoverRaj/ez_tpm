use zerocopy::{FromBytes, FromZeros, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::*;

#[derive(Debug, Immutable, Unaligned, IntoBytes)]
#[repr(C)]
struct GetRandomCommand {
    header: CommandHeader,
    parameters: GetRandomParameters,
}

#[derive(Debug, Immutable, Unaligned, IntoBytes)]
#[repr(C)]
struct GetRandomParameters {
    bytes_requested: [u8; 2],
}

#[derive(Debug, Immutable, KnownLayout, FromBytes, IntoBytes, Unaligned)]
#[repr(C)]
pub struct GetRandomResponse<const N: usize> {
    header: ResponseHeader,
    parameters: GetRandomResponseParameters<N>,
}

#[derive(Debug, Immutable, KnownLayout, FromBytes, IntoBytes, Unaligned)]
#[repr(C)]
pub struct GetRandomResponseParameters<const N: usize> {
    random_bytes: Tpm2Buffer<N>,
}

/// Currently this is a `const` and not a generic because Rust doesn't properly support it
const N: usize = 16;
/// This command supports a variable number of random bytes to generate.
/// However, currently it is hard-coded to 16 because Rust doesn't support `size_of` with generics.
pub struct GetRandom {
    input: GetRandomCommand,
    output: GetRandomResponse<N>,
}

impl GetRandom {
    pub fn new() -> Self {
        Self {
            input: GetRandomCommand {
                header: CommandHeader {
                    tag: TPM_ST_NO_SESSIONS.to_be_bytes(),
                    command_size: ((size_of::<CommandHeader>() + size_of::<GetRandomParameters>())
                        as u32)
                        .to_be_bytes(),
                    command_code: TPM_CC_GET_RANDOM.to_be_bytes(),
                },
                parameters: GetRandomParameters {
                    bytes_requested: (N as u16).to_be_bytes(),
                },
            },
            output: FromZeros::new_zeroed(),
        }
    }
}

const TPM_CC_GET_RANDOM: u32 = 0x0000017B;

impl<'a> Command<'a> for GetRandom {
    type Output = &'a [u8];

    fn input_and_output(&mut self) -> (&[u8], &mut [u8]) {
        (self.input.as_bytes(), self.output.as_mut_bytes())
    }

    fn process_output(
        response_header: &'a mut ResponseHeader,
        parameters: &'a mut [u8],
    ) -> Self::Output {
        let _ = response_header;
        let parameters = GetRandomResponseParameters::<N>::ref_from_bytes(parameters).unwrap();
        let len = u16::from_be_bytes(parameters.random_bytes.size) as usize;
        &parameters.random_bytes.buffer[..len]
    }
}

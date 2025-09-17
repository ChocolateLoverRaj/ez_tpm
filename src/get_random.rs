use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::{Command, CommandHeader, ResponseHeader, TPM_ST_NO_SESSIONS};

#[derive(Debug, Immutable, Unaligned, IntoBytes)]
#[repr(C)]
struct GetRandomParameters {
    bytes_requested: [u8; 2],
}

#[derive(Debug, Immutable, KnownLayout, FromBytes)]
#[repr(C)]
pub struct GetRandomResponse<const N: usize> {
    random_bytes: Tpm2bDigest<N>,
}

#[derive(Debug, Immutable, KnownLayout, FromBytes)]
#[repr(C)]
pub struct Tpm2bDigest<const N: usize> {
    size: [u8; 2],
    bytes: [u8; N],
}

/// Currently this is a `const` and not a generic because Rust doesn't properly support it
const N: usize = 16;
/// This command supports a variable number of random bytes to generate.
/// However, currently it is hard-coded to 16 because Rust doesn't support `size_of` with generics.
pub struct GetRandom {
    input: [u8; size_of::<CommandHeader>() + size_of::<GetRandomParameters>()],
    output: [u8; size_of::<ResponseHeader>() + size_of::<GetRandomResponse<N>>()],
}

impl GetRandom {
    pub fn new() -> Self {
        Self {
            input: {
                let mut input = [Default::default();
                    size_of::<CommandHeader>() + size_of::<GetRandomParameters>()];
                (input[..size_of::<CommandHeader>()]).copy_from_slice(
                    CommandHeader {
                        tag: TPM_ST_NO_SESSIONS.to_be_bytes(),
                        command_size: ((size_of::<CommandHeader>()
                            + size_of::<GetRandomParameters>())
                            as u32)
                            .to_be_bytes(),
                        command_code: TPM_CC_GET_RANDOM.to_be_bytes(),
                    }
                    .as_bytes(),
                );
                (input[size_of::<CommandHeader>()..]).copy_from_slice(
                    GetRandomParameters {
                        bytes_requested: (N as u16).to_be_bytes(),
                    }
                    .as_bytes(),
                );
                input
            },
            output: [0; size_of::<ResponseHeader>() + size_of::<GetRandomResponse<N>>()],
        }
    }
}

const TPM_CC_GET_RANDOM: u32 = 0x0000017B;

impl Command for GetRandom {
    type Output = [u8];

    fn input_and_output(&mut self) -> (&[u8], &mut [u8]) {
        (&self.input, &mut self.output)
    }

    fn process_output<'a>(
        response_header: &'a mut ResponseHeader,
        parameters: &'a mut [u8],
    ) -> &'a Self::Output {
        let _ = response_header;
        let parameters = GetRandomResponse::<N>::ref_from_bytes(parameters).unwrap();
        let len = u16::from_be_bytes(parameters.random_bytes.size) as usize;
        &parameters.random_bytes.bytes[..len]
    }
}

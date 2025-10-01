use zerocopy::{FromBytes, FromZeros, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::*;

#[derive(Debug, Immutable, Unaligned, FromBytes, IntoBytes)]
#[repr(C)]
struct PcrReadCommand {
    header: CommandHeader,
    parameters: PcrReadParameters<N_ALGORITHMS, N_PCR_BITMAP_BYTES>,
}

#[derive(Debug, Immutable, Unaligned, IntoBytes, FromBytes)]
#[repr(C)]
struct PcrReadParameters<const N_ALGORITHMS: usize, const N_PCR_BITMAP_BYTES: usize> {
    pcr_selection_in: TpmlPcrSelection<N_ALGORITHMS, N_PCR_BITMAP_BYTES>,
}

#[derive(Debug, Immutable, Unaligned, FromBytes, IntoBytes)]
#[repr(C)]
struct PcrReadResponse {
    header: ResponseHeader,
    parameters:
        PcrReadResponseParameters<N_ALGORITHMS, N_PCR_BITMAP_BYTES, DIGEST_BYTE_LEN, N_DIGESTS>,
}

#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
#[repr(C)]
struct PcrReadResponseParameters<
    const N_ALGORITHMS: usize,
    const N_PCR_BITMAP_BYTES: usize,
    const DIGEST_BYTE_LEN: usize,
    const N_DIGESTS: usize,
> {
    pcr_update_counter: [u8; 4],
    pcr_selection_out: TpmlPcrSelection<N_ALGORITHMS, N_PCR_BITMAP_BYTES>,
    pcr_values: TpmlDigest<DIGEST_BYTE_LEN, N_DIGESTS>,
}

#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
#[repr(C)]
struct TpmlDigest<const DIGEST_BYTE_LEN: usize, const N_DIGESTS: usize> {
    count: [u8; 4],
    digests: [Tpm2Buffer<DIGEST_BYTE_LEN>; N_DIGESTS],
}

/// We're hardcoding this to 1 for now, so just read multiple times for multiple algorithms.
const N_ALGORITHMS: usize = 1;
/// This will be enough for all 24 PCRs
const N_PCR_BITMAP_BYTES: usize = 3;

/// Currently hard-coded to sha1 len
const DIGEST_BYTE_LEN: usize = 20;
/// Currently hard-coded to 1 digest
const N_DIGESTS: usize = 1;

/// Currently only reads a single SHA1 PCR
pub struct PcrRead {
    input: PcrReadCommand,
    output: PcrReadResponse,
}

impl PcrRead {
    pub fn new(pcr: usize) -> Self {
        Self {
            input: PcrReadCommand {
                header: CommandHeader {
                    tag: TPM_ST_NO_SESSIONS.to_be_bytes(),
                    command_size: (size_of::<PcrReadCommand>() as u32).to_be_bytes(),
                    command_code: TPM_CC_PCR_READ.to_be_bytes(),
                },
                parameters: PcrReadParameters {
                    pcr_selection_in: TpmlPcrSelection {
                        count: 1_u32.to_be_bytes(),
                        pcr_selections: [TpmsPcrSelection {
                            hash: u16::from(TpmAlgId::Sha1).to_be_bytes(),
                            size_of_select: 3,
                            pcr_select: {
                                let mut bitmap = [0; 3];
                                bitmap[pcr / 8] |= 1 << (pcr % 8);
                                bitmap
                            },
                        }],
                    },
                },
            },
            output: FromZeros::new_zeroed(),
        }
    }
}

impl<'a> Command<'a> for PcrRead {
    type Output = &'a [u8; 20];

    fn input_and_output(&mut self) -> (&[u8], &mut [u8]) {
        (self.input.as_bytes(), self.output.as_mut_bytes())
    }

    fn process_output(
        response_header: &'a mut ResponseHeader,
        parameters: &'a mut [u8],
    ) -> Self::Output {
        let _ = response_header;
        let parameters = PcrReadResponseParameters::<
            N_ALGORITHMS,
            N_PCR_BITMAP_BYTES,
            DIGEST_BYTE_LEN,
            N_DIGESTS,
        >::ref_from_bytes(parameters)
        .unwrap();
        &parameters.pcr_values.digests.first().unwrap().buffer
    }
}

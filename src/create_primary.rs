use zerocopy::FromZeros;

use crate::{command::Command, *};

type CreatePrimaryCommand =
    TpmCommand<CreatePrimaryInputs<0, 0, 0, { size_of::<TpmsRsaParms>() }, 0, 0, 0, 0>>;

type CreatePrimaryResponse = TpmResponse<CreatePrimaryOutputs<0, 0, 0, 0, 0, 0, 0>>;

/// `TPM2_CreatePrimary`
pub struct CreatePrimary {
    command: CreatePrimaryCommand,
    response: CreatePrimaryResponse,
}

impl CreatePrimary {
    pub fn new() -> Self {
        Self {
            command: TpmCommand {
                header: CommandHeader {
                    command_code: TPM_CC_CREATE_PRIMARY.to_be_bytes(),
                    tag: TPM_ST_SESSIONS.to_be_bytes(),
                    command_size: (size_of::<CreatePrimaryCommand>() as u32).to_be_bytes(),
                },
                inputs: CreatePrimaryInputs {
                    primary_handle: TPM_RH_ENDORSEMENT.to_be_bytes(),
                    in_sensitive: Tpm2bSensitiveCreate {
                        size: 0_u16.to_be_bytes(),
                        sensitive: TpmsSensitiveCreate {
                            user_auth: Tpm2bDigest {
                                size: 0_u16.to_be_bytes(),
                                buffer: [],
                            },
                            data: Tpm2bDigest {
                                size: 0_u16.to_be_bytes(),
                                buffer: [],
                            },
                        },
                    },
                    in_public: TpmtPublic {
                        _type: TPM_ALG_RSA.to_be_bytes(),
                        name_alg: TPM_ALG_SHA256.to_be_bytes(),
                        object_attributes: (TPMA_OBJECT_FIXED_TPM
                            | TPMA_OBJECT_FIXED_PARENT
                            | TPMA_OBJECT_SENSITIVE_DATA_ORIGIN
                            | TPMA_OBJECT_USER_WITH_AUTH
                            | TPMA_OBJECT_RESTRICTED
                            | TPMA_OBJECT_DECRYPT)
                            .to_be_bytes(),
                        auth_policy: Tpm2bDigest::EMPTY,
                        parameters: {
                            let mut bytes = [Default::default(); size_of::<TpmsRsaParms>()];
                            bytes.copy_from_slice(
                                TpmsRsaParms {
                                    symmetric: TPM_ALG_AES.to_be_bytes(),
                                    scheme: TPM_ALG_NULL.to_be_bytes(),
                                    key_bits: 2048_u16.to_be_bytes(),
                                    exponent: 65537_u32.to_be_bytes(),
                                }
                                .as_bytes(),
                            );
                            bytes
                        },
                        unique: {
                            let bytes = Default::default();
                            bytes
                        },
                    },
                    outside_info: Tpm2bDigest {
                        size: 0_u16.to_be_bytes(),
                        buffer: [],
                    },
                    creation_pcr: TpmlPcrSelection {
                        count: 0_u32.to_be_bytes(),
                        pcr_selections: [],
                    },
                },
            },
            response: FromZeros::new_zeroed(),
        }
    }
}

impl<'a> Command<'a> for CreatePrimary {
    type Output = ();
    fn input_and_output(&mut self) -> (&[u8], &mut [u8]) {
        (self.command.as_bytes(), self.response.as_mut_bytes())
    }
    fn process_output(
        response_header: &'a mut ResponseHeader,
        parameters: &'a mut [u8],
    ) -> Self::Output {
        todo!()
    }
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
struct CreatePrimaryInputs<
    const USER_AUTH_BYTE_LEN: usize,
    const DATA_BYTE_LEN: usize,
    const AUTH_POLICY_BYTE_LEN: usize,
    const PARAMETERS_BYTE_LEN: usize,
    const PUBLIC_ID_BYTE_LEN: usize,
    const OUTSIDE_INFO_LEN: usize,
    const N_ALGORITHMS: usize,
    const N_PCR_BITMAP_BYTES: usize,
> {
    primary_handle: [u8; 4],
    in_sensitive: Tpm2bSensitiveCreate<USER_AUTH_BYTE_LEN, DATA_BYTE_LEN>,
    in_public: TpmtPublic<AUTH_POLICY_BYTE_LEN, PARAMETERS_BYTE_LEN, PUBLIC_ID_BYTE_LEN>,
    outside_info: Tpm2bDigest<OUTSIDE_INFO_LEN>,
    creation_pcr: TpmlPcrSelection<N_ALGORITHMS, N_PCR_BITMAP_BYTES>,
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
struct TpmtPublic<
    const AUTH_POLICY_BYTE_LEN: usize,
    const PARAMETERS_BYTE_LEN: usize,
    const PUBLIC_ID_BYTE_LEN: usize,
> {
    _type: [u8; 2],
    name_alg: [u8; 2],
    object_attributes: [u8; 4],
    auth_policy: Tpm2bDigest<AUTH_POLICY_BYTE_LEN>,
    parameters: [u8; PARAMETERS_BYTE_LEN],
    /// The unique identifier of the structure.
    /// For an asymmetric key, this would be the public key.
    unique: [u8; PUBLIC_ID_BYTE_LEN],
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
struct TpmsRsaParms {
    ///  For a restricted decryption key, shall be set to a supported symmetric algorithm, key size, and mode.
    /// If the key is not a restricted decryption key, this field shall be set to TPM_ALG_NULL.
    symmetric: [u8; 2],
    scheme: [u8; 2],
    key_bits: [u8; 2],
    exponent: [u8; 4],
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
struct Tpm2bSensitiveCreate<const USER_AUTH_BYTE_LEN: usize, const DATA_BYTE_LEN: usize> {
    size: [u8; 2],
    sensitive: TpmsSensitiveCreate<USER_AUTH_BYTE_LEN, DATA_BYTE_LEN>,
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
struct TpmsSensitiveCreate<const USER_AUTH_BYTE_LEN: usize, const DATA_BYTE_LEN: usize> {
    user_auth: Tpm2bDigest<USER_AUTH_BYTE_LEN>,
    data: Tpm2bDigest<DATA_BYTE_LEN>,
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
struct CreatePrimaryOutputs<
    const PUBLIC_BYTE_LEN: usize,
    const N_ALGORITHMS: usize,
    const N_PCR_BITMAP_BYTES: usize,
    const DIGEST_BYTE_LEN: usize,
    const PARENT_NAME_BYTE_LEN: usize,
    const PARENT_QUALIFIED_NAME_BYTE_LEN: usize,
    const OUTSIDE_INFO_BYTE_LEN: usize,
> {
    handle: [u8; 4],
    out_public: Tpm2bDigest<PUBLIC_BYTE_LEN>,
    creation_data: Tpm2bCreationData<
        N_ALGORITHMS,
        N_PCR_BITMAP_BYTES,
        DIGEST_BYTE_LEN,
        PARENT_NAME_BYTE_LEN,
        PARENT_QUALIFIED_NAME_BYTE_LEN,
        OUTSIDE_INFO_BYTE_LEN,
    >,
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
struct Tpm2bCreationData<
    const N_ALGORITHMS: usize,
    const N_PCR_BITMAP_BYTES: usize,
    const DIGEST_BYTE_LEN: usize,
    const PARENT_NAME_BYTE_LEN: usize,
    const PARENT_QUALIFIED_NAME_BYTE_LEN: usize,
    const OUTSIDE_INFO_BYTE_LEN: usize,
> {
    size: [u8; 2],
    creation_data: TpmsCreationData<
        N_ALGORITHMS,
        N_PCR_BITMAP_BYTES,
        DIGEST_BYTE_LEN,
        PARENT_NAME_BYTE_LEN,
        PARENT_QUALIFIED_NAME_BYTE_LEN,
        OUTSIDE_INFO_BYTE_LEN,
    >,
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
struct TpmsCreationData<
    const N_ALGORITHMS: usize,
    const N_PCR_BITMAP_BYTES: usize,
    const DIGEST_BYTE_LEN: usize,
    const PARENT_NAME_BYTE_LEN: usize,
    const PARENT_QUALIFIED_NAME_BYTE_LEN: usize,
    const OUTSIDE_INFO_BYTE_LEN: usize,
> {
    pcr_select: TpmlPcrSelection<N_ALGORITHMS, N_PCR_BITMAP_BYTES>,
    pcr_digest: Tpm2bDigest<DIGEST_BYTE_LEN>,
    locality: u8,
    parent_name_alg: [u8; 2],
    parent_name: Tpm2bDigest<PARENT_NAME_BYTE_LEN>,
    parent_qualified_name: Tpm2bDigest<PARENT_QUALIFIED_NAME_BYTE_LEN>,
    outside_info: Tpm2bDigest<OUTSIDE_INFO_BYTE_LEN>,
}

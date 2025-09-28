// TPM_ALG_ID constants, as defined in
// TCG TPM 2.0 Library Specification Part 2: Structures v184 (2025/03/20)

pub type TpmAlgId = u16;

// Core algorithm identifiers
pub const TPM_ALG_ERROR: TpmAlgId = 0x0000;
pub const TPM_ALG_RSA: TpmAlgId = 0x0001;
pub const TPM_ALG_TDES: TpmAlgId = 0x0003;
pub const TPM_ALG_SHA: TpmAlgId = 0x0004;
pub const TPM_ALG_SHA1: TpmAlgId = 0x0004;
pub const TPM_ALG_HMAC: TpmAlgId = 0x0005;
pub const TPM_ALG_AES: TpmAlgId = 0x0006;
pub const TPM_ALG_MGF1: TpmAlgId = 0x0007;
pub const TPM_ALG_KEYEDHASH: TpmAlgId = 0x0008;
pub const TPM_ALG_XOR: TpmAlgId = 0x000A;
pub const TPM_ALG_SHA256: TpmAlgId = 0x000B;
pub const TPM_ALG_SHA384: TpmAlgId = 0x000C;
pub const TPM_ALG_SHA512: TpmAlgId = 0x000D;
pub const TPM_ALG_SHA256_192: TpmAlgId = 0x000E;
pub const TPM_ALG_NULL: TpmAlgId = 0x0010;
pub const TPM_ALG_SM3_256: TpmAlgId = 0x0012;
pub const TPM_ALG_SM4: TpmAlgId = 0x0013;

// Asymmetric & ECC
pub const TPM_ALG_RSASSA: TpmAlgId = 0x0014;
pub const TPM_ALG_RSAES: TpmAlgId = 0x0015;
pub const TPM_ALG_RSAPSS: TpmAlgId = 0x0016;
pub const TPM_ALG_OAEP: TpmAlgId = 0x0017;
pub const TPM_ALG_ECDSA: TpmAlgId = 0x0018;
pub const TPM_ALG_ECDH: TpmAlgId = 0x0019;
pub const TPM_ALG_ECDAA: TpmAlgId = 0x001A;
pub const TPM_ALG_SM2: TpmAlgId = 0x001B;
pub const TPM_ALG_ECSCHNORR: TpmAlgId = 0x001C;
pub const TPM_ALG_ECMQV: TpmAlgId = 0x001D;

// Key Derivation & ECC object
pub const TPM_ALG_KDF1_SP800_56A: TpmAlgId = 0x0020;
pub const TPM_ALG_KDF2: TpmAlgId = 0x0021;
pub const TPM_ALG_KDF1_SP800_108: TpmAlgId = 0x0022;
pub const TPM_ALG_ECC: TpmAlgId = 0x0023;

// Symmetric cipher object and Camellia
pub const TPM_ALG_SYMCIPHER: TpmAlgId = 0x0025;
pub const TPM_ALG_CAMELLIA: TpmAlgId = 0x0026;

// SHA-3 and SHAKE family
pub const TPM_ALG_SHA3_256: TpmAlgId = 0x0027;
pub const TPM_ALG_SHA3_384: TpmAlgId = 0x0028;
pub const TPM_ALG_SHA3_512: TpmAlgId = 0x0029;
pub const TPM_ALG_SHAKE128: TpmAlgId = 0x002A;
pub const TPM_ALG_SHAKE256: TpmAlgId = 0x002B;
pub const TPM_ALG_SHAKE256_192: TpmAlgId = 0x002C;
pub const TPM_ALG_SHAKE256_256: TpmAlgId = 0x002D;
pub const TPM_ALG_SHAKE256_512: TpmAlgId = 0x002E;

// Block cipher MACs and modes
pub const TPM_ALG_CMAC: TpmAlgId = 0x003F;
pub const TPM_ALG_CTR: TpmAlgId = 0x0040;
pub const TPM_ALG_OFB: TpmAlgId = 0x0041;
pub const TPM_ALG_CBC: TpmAlgId = 0x0042;
pub const TPM_ALG_CFB: TpmAlgId = 0x0043;
pub const TPM_ALG_ECB: TpmAlgId = 0x0044;
pub const TPM_ALG_CCM: TpmAlgId = 0x0050;
pub const TPM_ALG_GCM: TpmAlgId = 0x0051;
pub const TPM_ALG_KW: TpmAlgId = 0x0052;
pub const TPM_ALG_KWP: TpmAlgId = 0x0053;
pub const TPM_ALG_EAX: TpmAlgId = 0x0054;

// EdDSA
pub const TPM_ALG_EDDSA: TpmAlgId = 0x0060;
pub const TPM_ALG_EDDSA_PH: TpmAlgId = 0x0061;

// Hash-based and XMSS/LMS
pub const TPM_ALG_LMS: TpmAlgId = 0x0070;
pub const TPM_ALG_XMSS: TpmAlgId = 0x0071;

// Keyed XOF and KMAC
pub const TPM_ALG_KEYEDXOF: TpmAlgId = 0x0080;
pub const TPM_ALG_KMACXOF128: TpmAlgId = 0x0081;
pub const TPM_ALG_KMACXOF256: TpmAlgId = 0x0082;
pub const TPM_ALG_KMAC128: TpmAlgId = 0x0090;
pub const TPM_ALG_KMAC256: TpmAlgId = 0x0091;

// Reserved ranges
// 0x00C1..=0x00C6 and 0x8000..=0xFFFF are reserved

// Convenience aliases (sometimes found in codebases)
pub const TPM_ALG_ID_FIRST: TpmAlgId = TPM_ALG_ERROR;
pub const TPM_ALG_ID_LAST: TpmAlgId = 0xFFFF;

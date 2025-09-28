/// TPMA_OBJECT: Object attribute bitfields (see TPM 2.0 Library Part 2: Structures v184 Table 39)
/// Each constant represents a bit position for the object attribute.

pub const TPMA_OBJECT_FIXED_TPM: u32 = 1 << 1;
pub const TPMA_OBJECT_ST_CLEAR: u32 = 1 << 2;
pub const TPMA_OBJECT_FIXED_PARENT: u32 = 1 << 4;
pub const TPMA_OBJECT_SENSITIVE_DATA_ORIGIN: u32 = 1 << 5;
pub const TPMA_OBJECT_USER_WITH_AUTH: u32 = 1 << 6;
pub const TPMA_OBJECT_ADMIN_WITH_POLICY: u32 = 1 << 7;
pub const TPMA_OBJECT_FIRMWARE_LIMITED: u32 = 1 << 8;
pub const TPMA_OBJECT_SVN_LIMITED: u32 = 1 << 9;
pub const TPMA_OBJECT_NO_DA: u32 = 1 << 10;
pub const TPMA_OBJECT_ENCRYPTED_DUPLICATION: u32 = 1 << 11;
pub const TPMA_OBJECT_RESTRICTED: u32 = 1 << 16;
pub const TPMA_OBJECT_DECRYPT: u32 = 1 << 17;
pub const TPMA_OBJECT_SIGN_ENCRYPT: u32 = 1 << 18;
pub const TPMA_OBJECT_X509_SIGN: u32 = 1 << 19;

// Reserved bits: 0, 3, 12-15, 20-31 (must be zero)

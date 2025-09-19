/// TPM_HT represents the handle type field of a TPM handle.
/// Each handle in TPM 2.0 has a high byte that encodes its type.
/// See "TPM 2.0 Library Specification Part 2: Structures" (Table 35).

/// 0x00 - PCR - consecutive numbers, starting at 0, that reference the PCR registers.
/// The minimum number of PCRs is platform specific; implementations may have more.
pub const TPM_HT_PCR: u8 = 0x00;
/// 0x01 - NV Index - assigned by the caller.
pub const TPM_HT_NV_INDEX: u8 = 0x01;
/// 0x02 - HMAC Authorization Session - assigned by the TPM when the session is created.
pub const TPM_HT_HMAC_SESSION: u8 = 0x02;
/// 0x02 - Loaded Authorization Session - used only in the context of TPM2_GetCapability().
/// This type references both loaded HMAC and loaded policy authorization sessions.
pub const TPM_HT_LOADED_SESSION: u8 = 0x02;
/// 0x03 - Policy Authorization Session - assigned by the TPM when the session is created.
pub const TPM_HT_POLICY_SESSION: u8 = 0x03;
/// 0x03 - Saved Authorization Session - used only in the context of TPM2_GetCapability().
/// This type references saved authorization session contexts for which the TPM is maintaining tracking information.
pub const TPM_HT_SAVED_SESSION: u8 = 0x03;
/// 0x11 - External NV Index - assigned by the caller (added in version 1.83).
pub const TPM_HT_EXTERNAL_NV: u8 = 0x11;
/// 0x12 - Permanent NV Index - assigned by a platform specific specification (added in version 1.83).
pub const TPM_HT_PERMANENT_NV: u8 = 0x12;
/// 0x40 - Permanent Values - assigned by this specification in Table 36.
pub const TPM_HT_PERMANENT: u8 = 0x40;
/// 0x80 - Transient Objects - assigned by the TPM when an object is loaded into transient-object memory or when a persistent object is converted to a transient object.
pub const TPM_HT_TRANSIENT: u8 = 0x80;
/// 0x81 - Persistent Objects - assigned by the TPM when a loaded transient object is made persistent.
pub const TPM_HT_PERSISTENT: u8 = 0x81;
/// 0x90 - Attached Component - handle for an Attached Component.
pub const TPM_HT_AC: u8 = 0x90;

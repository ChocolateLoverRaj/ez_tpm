/// TPM_CAP values (capability codes) for TPM2_GetCapability().
/// See "TPM 2.0 Library Specification Part 2: Structures" Table 26 (v184, 2025/03/20).

/// First valid capability value.
pub const TPM_CAP_FIRST: u32 = 0x00000000;
/// Algorithm properties (returns TPML_ALG_PROPERTY).
pub const TPM_CAP_ALGS: u32 = 0x00000000;
/// Handle enumeration (returns TPML_HANDLE).
pub const TPM_CAP_HANDLES: u32 = 0x00000001;
/// Command properties (returns TPML_CCA).
pub const TPM_CAP_COMMANDS: u32 = 0x00000002;
/// Physical presence commands (returns TPML_CC).
pub const TPM_CAP_PP_COMMANDS: u32 = 0x00000003;
/// Audit commands (returns TPML_CC).
pub const TPM_CAP_AUDIT_COMMANDS: u32 = 0x00000004;
/// PCR selection (returns TPML_PCR_SELECTION).
pub const TPM_CAP_PCRS: u32 = 0x00000005;
/// TPM properties (returns TPML_TAGGED_TPM_PROPERTY).
pub const TPM_CAP_TPM_PROPERTIES: u32 = 0x00000006;
/// PCR properties (returns TPML_TAGGED_PCR_PROPERTY).
pub const TPM_CAP_PCR_PROPERTIES: u32 = 0x00000007;
/// ECC curves (returns TPML_ECC_CURVE).
pub const TPM_CAP_ECC_CURVES: u32 = 0x00000008;
/// Authorization policies (returns TPML_TAGGED_POLICY).
pub const TPM_CAP_AUTH_POLICIES: u32 = 0x00000009;
/// ACT (returns TPML_ACT_DATA).
pub const TPM_CAP_ACT: u32 = 0x0000000A;
/// Public keys (returns TPML_PUB_KEY).
pub const TPM_CAP_PUB_KEYS: u32 = 0x0000000B;
/// SPDM session info (returns TPML_SPDM_SESSION_INFO).
pub const TPM_CAP_SPDM_SESSION_INFO: u32 = 0x0000000C;
/// Last valid capability value.
pub const TPM_CAP_LAST: u32 = 0x0000000C;
/// Vendor-specific capability values.
pub const TPM_CAP_VENDOR_PROPERTY: u32 = 0x00000100;

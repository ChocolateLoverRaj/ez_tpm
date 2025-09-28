// Architecturally defined permanent TPM handles (TPM_RH)
// Source: TPM 2.0 Library Part 2: Structures, Table 36, v184 (2025/03/20)

pub const TPM_RH_FIRST: u32 = 0x4000_0000;
pub const TPM_RH_SRK: u32 = 0x4000_0000; // Not used
pub const TPM_RH_OWNER: u32 = 0x4000_0001;
pub const TPM_RH_REVOKE: u32 = 0x4000_0002; // Not used
pub const TPM_RH_TRANSPORT: u32 = 0x4000_0003; // Not used
pub const TPM_RH_OPERATOR: u32 = 0x4000_0004; // Not used
pub const TPM_RH_ADMIN: u32 = 0x4000_0005; // Not used
pub const TPM_RH_EK: u32 = 0x4000_0006; // Not used
pub const TPM_RH_NULL: u32 = 0x4000_0007; // NULL hierarchy
pub const TPM_RH_UNASSIGNED: u32 = 0x4000_0008; // Value reserved for unassigned
pub const TPM_RS_PW: u32 = 0x4000_0009; // Password authorization session
pub const TPM_RH_LOCKOUT: u32 = 0x4000_000A; // Dictionary attack lockout
pub const TPM_RH_ENDORSEMENT: u32 = 0x4000_000B; // Endorsement hierarchy
pub const TPM_RH_PLATFORM: u32 = 0x4000_000C; // Platform hierarchy
pub const TPM_RH_PLATFORM_NV: u32 = 0x4000_000D; // phEnableNV
pub const TPM_RH_AUTH_00: u32 = 0x4000_0010; // Start of vendor-specific range
pub const TPM_RH_AUTH_FF: u32 = 0x4000_010F; // End of vendor-specific range
pub const TPM_RH_ACT_0: u32 = 0x4000_0110; // Start of authenticated timers
pub const TPM_RH_ACT_F: u32 = 0x4000_011F; // End of authenticated timers

// Firmware-limited hierarchies
pub const TPM_RH_FW_OWNER: u32 = 0x4000_0140;
pub const TPM_RH_FW_ENDORSEMENT: u32 = 0x4000_0141;
pub const TPM_RH_FW_PLATFORM: u32 = 0x4000_0142;
pub const TPM_RH_FW_NULL: u32 = 0x4000_0143;

// SVN-limited hierarchies (low 2 bytes are the minimum SVN)
pub const TPM_RH_SVN_OWNER_BASE: u32 = 0x4001_0000;
pub const TPM_RH_SVN_ENDORSEMENT_BASE: u32 = 0x4002_0000;
pub const TPM_RH_SVN_PLATFORM_BASE: u32 = 0x4003_0000;
pub const TPM_RH_SVN_NULL_BASE: u32 = 0x4004_0000;

pub const TPM_RH_LAST: u32 = 0x4004_FFFF;

/// TPM_ST values (TPM structure tags) for TPM command/response headers.
/// See "TPM 2.0 Library Specification Part 2: Structures" Table 9.

/// 0x8001 - Indicates that the command or response does not use a session header.
pub const TPM_ST_NO_SESSIONS: u16 = 0x8001;
/// 0x8002 - Indicates that the command or response uses a session header.
pub const TPM_ST_SESSIONS: u16 = 0x8002;

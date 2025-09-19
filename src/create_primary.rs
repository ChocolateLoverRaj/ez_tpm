use crate::*;

/// `TPM2_CreatePrimary`
pub struct CreatePrimary {
    command: TpmCommand<CreatePrimaryInputs>,
    response: TpmResponse<CreatePrimaryOutputs>,
}

// impl CreatePrimary {
//     pub fn new() -> Self {
//         Self {
//             command: TpmCommand {
//                 header: CommandHeader { command_code },
//             },
//         }
//     }
// }

#[repr(C)]
struct CreatePrimaryInputs {
    primary_handle: (),
}

#[repr(C)]
struct CreatePrimaryOutputs {}

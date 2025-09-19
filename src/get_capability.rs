use zerocopy::FromZeros;

use crate::*;

const N_HANDLES: usize = 1;
const DATA_SIZE: usize = size_of::<TpmlHandle<N_HANDLES>>();

pub struct GetCapability {
    command: TpmCommand<GetCapabilityInputs>,
    response: TpmResponse<GetCapabilityOutputs<DATA_SIZE>>,
}

impl GetCapability {
    pub fn new(index: u32) -> Self {
        Self {
            command: TpmCommand {
                header: CommandHeader {
                    command_code: TPM_CC_GET_CAPABILITY.to_be_bytes(),
                    command_size: (size_of::<TpmCommand<GetCapabilityInputs>>() as u32)
                        .to_be_bytes(),
                    tag: TPM_ST_NO_SESSIONS.to_be_bytes(),
                },
                inputs: GetCapabilityInputs {
                    capability: TPM_CAP_HANDLES.to_be_bytes(),
                    property: (TPM_HT_PERSISTENT as u32 * 0x1000000 + index).to_be_bytes(),
                    property_count: (N_HANDLES as u32).to_be_bytes(),
                },
            },
            response: FromZeros::new_zeroed(),
        }
    }
}

pub struct GetHandleCapabilityOutput<'a> {
    outputs: &'a GetCapabilityOutputs<DATA_SIZE>,
}

impl GetHandleCapabilityOutput<'_> {
    pub fn more_data(&self) -> bool {
        YesNo::try_from(self.outputs.more_data).unwrap().into()
    }

    pub fn handle(&self) -> Option<u32> {
        let tpml_handle =
            TpmlHandle::<1>::ref_from_bytes(self.outputs.capability_data.data.as_bytes()).unwrap();
        if u32::from_be_bytes(tpml_handle.count) == 1 {
            Some(u32::from_be_bytes(tpml_handle.handles[0]))
        } else {
            None
        }
    }
}

impl<'a> Command<'a> for GetCapability {
    type Output = GetHandleCapabilityOutput<'a>;

    fn input_and_output(&mut self) -> (&[u8], &mut [u8]) {
        (self.command.as_bytes(), self.response.as_mut_bytes())
    }

    fn process_output(
        _response_header: &'a mut ResponseHeader,
        parameters: &'a mut [u8],
    ) -> Self::Output {
        GetHandleCapabilityOutput {
            outputs: GetCapabilityOutputs::<DATA_SIZE>::ref_from_bytes(parameters).unwrap(),
        }
    }
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
struct GetCapabilityInputs {
    capability: [u8; 4],
    property: [u8; 4],
    property_count: [u8; 4],
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
struct GetCapabilityOutputs<const DATA_SIZE: usize> {
    more_data: u8,
    capability_data: TpmsCapabilityData<DATA_SIZE>,
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
struct TpmsCapabilityData<const DATA_SIZE: usize> {
    capability: [u8; 4],
    data: [u8; DATA_SIZE],
}

#[repr(C)]
#[derive(Debug, KnownLayout, Immutable, Unaligned, FromBytes, IntoBytes)]
struct TpmlHandle<const N: usize> {
    count: [u8; 4],
    handles: [[u8; 4]; N],
}

use uefi::proto::tcg::v2::Tcg;
use zerocopy::FromBytes;

use crate::{Command, ResponseCode, ResponseHeader};

#[derive(Debug)]
pub enum SubmitCommandError {
    UefiError(uefi::Error),
    TpmError(ResponseCode),
}

pub fn submit_command<'a, C: Command<'a>>(
    tcg: &mut Tcg,
    command: &'a mut C,
) -> Result<C::Output, SubmitCommandError> {
    let (input, output) = command.input_and_output();
    tcg.submit_command(input, output)
        .map_err(SubmitCommandError::UefiError)?;
    let (response_header, parameters) = output.split_at_mut(size_of::<ResponseHeader>());
    let response_header = ResponseHeader::mut_from_bytes(response_header).unwrap();
    let response_code = u32::from_be_bytes(response_header.response_code);
    if let Ok(response_code) = ResponseCode::try_from(response_code) {
        Err(SubmitCommandError::TpmError(response_code))
    } else {
        Ok(C::process_output(response_header, parameters))
    }
}

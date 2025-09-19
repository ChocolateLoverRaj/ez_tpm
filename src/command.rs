use crate::*;

pub trait Command {
    type Output: ?Sized;

    fn input_and_output(&mut self) -> (&[u8], &mut [u8]);
    /// Note that this function can be called if there is a command-specific error.
    fn process_output<'a>(
        response_header: &'a mut ResponseHeader,
        parameters: &'a mut [u8],
    ) -> &'a Self::Output;
}

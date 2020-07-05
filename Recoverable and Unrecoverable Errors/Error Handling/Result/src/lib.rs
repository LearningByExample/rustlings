use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
pub struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
pub enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    pub fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        /*here could be the expression to compare if the value is appropriate*/
        if value > 0 {
            Ok(PositiveNonzeroInteger(value as u64))
        } else {
            if value == 0 {
                Err(CreationError::Zero)
            } else {
                Err(CreationError::Negative)
            }
        }
        /*here could be the othere arms of the comparison*/
    }
}

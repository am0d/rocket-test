use std::fmt::{Debug, Display, Formatter, Result};

pub enum ValidateResult {
    Valid,
    Invalid(Vec<String>),
}

impl From<ValidateResult> for String {
    fn from(result: ValidateResult) -> String {
        match result {
            ValidateResult::Valid => String::default(),
            ValidateResult::Invalid(errors) => {
                errors.iter().fold(String::from(""), |mut acc, e| {
                    if !acc.is_empty() {
                        acc.push_str("\n");
                    }
                    acc.push_str(e);
                    acc
                })
            }
        }
    }
}

impl<'a> From<&'a ValidateResult> for String {
    fn from(result: &'a ValidateResult) -> String {
        match *result {
            ValidateResult::Valid => String::default(),
            ValidateResult::Invalid(ref errors) => {
                errors.iter().fold(String::from(""), |mut acc, e| {
                    if !acc.is_empty() {
                        acc.push_str("\n");
                    }
                    acc.push_str(e);
                    acc
                })
            }
        }
    }
}

pub trait Validate {
    fn is_valid(&self) -> ValidateResult;
}

impl Display for ValidateResult {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", String::from(self))
    }
}

impl Debug for ValidateResult {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", String::from(self))
    }
}

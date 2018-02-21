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

pub trait Validate {
    fn is_valid(&self) -> ValidateResult;
}

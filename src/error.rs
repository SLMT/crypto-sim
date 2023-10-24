use thiserror::Error;

pub type SimResult<T> = Result<T, SimError>;

#[derive(Error, Debug, PartialEq)]
pub enum SimError {
    #[error("the field {0} for building option is not set")]
    OptionFieldNotSet(String),
}

impl SimError {
    pub fn option_field_not_set(field_name: &str) -> SimError {
        SimError::OptionFieldNotSet(field_name.to_owned())
    }
}

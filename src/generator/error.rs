use std::fmt;

pub struct GeneratorError {
    pub error_msg: String,
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_msg)
    }
}

impl fmt::Debug for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}: {}", file!(), line!(), self.error_msg)
    }
}

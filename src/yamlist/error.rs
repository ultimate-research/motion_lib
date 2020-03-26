pub struct ErrorMessage {
    msg: String,
}

pub struct ErrorString(pub &'static str);

impl<E> From<E> for ErrorMessage
where
    E: std::error::Error,
{
    fn from(error: E) -> Self {
        Self {
            msg: format!("{}", error),
        }
    }
}

impl From<ErrorString> for ErrorMessage {
    fn from(error: ErrorString) -> Self {
        Self {
            msg: error.0.to_string(),
        }
    }
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.msg)
    }
}

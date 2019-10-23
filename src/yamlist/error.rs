pub struct ErrorMessage {
    msg: String,
}

impl<E> From<E> for ErrorMessage
    where E: std::error::Error
{
    fn from(error: E) -> Self {
        Self {
            msg: format!("{}", error)
        }
    }
}

impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.msg)
    }
}

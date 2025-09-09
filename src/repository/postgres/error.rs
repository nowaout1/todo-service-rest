use crate::repository::Error;

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        // TODO: better error handling
        match value {
            _ => Self::Database,
        }
    }
}

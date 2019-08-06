use failure::{Error, Fail};
use sql_migration_connector::error::SqlError;
use crate::commands::CommandError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Error in connector: {}", _0)]
    ConnectorError(Error),
    #[fail(display = "Failure during a migration command: {}", _0)]
    CommandError(CommandError),
}

impl From<SqlError> for Error {
    fn from(e: SqlError) -> Self {
        Error::ConnectorError(e.into())
    }
}

impl From<CommandError> for Error {
    fn from(e: CommandError) -> Self {
        Error::CommandError(e)
    }
}

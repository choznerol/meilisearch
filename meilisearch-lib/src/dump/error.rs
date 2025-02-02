use meilisearch_auth::error::AuthControllerError;
use meilisearch_error::{internal_error, Code, ErrorCode};

use crate::{index_resolver::error::IndexResolverError, tasks::error::TaskError};

pub type Result<T> = std::result::Result<T, DumpError>;

#[derive(thiserror::Error, Debug)]
pub enum DumpError {
    #[error("An internal error has occurred. `{0}`.")]
    Internal(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("{0}")]
    IndexResolver(#[from] IndexResolverError),
}

internal_error!(
    DumpError: milli::heed::Error,
    std::io::Error,
    tokio::task::JoinError,
    tokio::sync::oneshot::error::RecvError,
    serde_json::error::Error,
    tempfile::PersistError,
    fs_extra::error::Error,
    AuthControllerError,
    TaskError
);

impl ErrorCode for DumpError {
    fn error_code(&self) -> Code {
        match self {
            DumpError::Internal(_) => Code::Internal,
            DumpError::IndexResolver(e) => e.error_code(),
        }
    }
}

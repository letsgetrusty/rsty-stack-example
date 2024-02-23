// from: https://github.com/rust-awesome-app/template-app-base/blob/main/src-tauri/src/error.rs

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Surrealdb(#[from] surrealdb::error::Db),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

impl From<Error> for std::io::Error {
    fn from(e: Error) -> std::io::Error {
        match e {
            Error::IO(e) => e,
            _ => std::io::Error::new(std::io::ErrorKind::Other, e.to_string()),
        }
    }
}

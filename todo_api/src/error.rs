// from: https://github.com/rust-awesome-app/template-app-base/blob/main/src-tauri/src/error.rs

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Value not of type '{0}'")]
    XValueNotOfType(&'static str),

    #[error(transparent)]
    Surreal(#[from] surrealdb::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}

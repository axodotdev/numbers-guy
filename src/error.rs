use thiserror::Error;

#[derive(Error, Debug)]
pub enum NumbersGuyError {
    #[error("octocrab errored")]
    Octocrab(#[from] octocrab::Error),
}

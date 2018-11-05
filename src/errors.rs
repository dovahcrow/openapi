use std::result::Result as StdResult;

use failure::Error;
use semver::Version;

pub type Result<T> = StdResult<T, Error>;

#[derive(Fail, Debug, Clone)]
pub enum OpenApiError {
    #[fail(
        display = "Unsupported spec file version ({}). Expected >= 3.0",
        _0
    )]
    UnsupportedSpecFileVersion(Version),
}

#![allow(unused_imports)]
#[cfg(feature = "std")]
use crate::cue::show::Show;

#[cfg(feature = "serde")]
extern crate serde;
#[cfg(feature = "json")]
extern crate serde_json;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "std")]
#[allow(dead_code)]
/// Builder type for [Show]
pub struct ShowBuilder {}

#[cfg(feature = "std")]
#[allow(dead_code)]
impl ShowBuilder {
    /// Load a [Show] from a binary file representation of a show object
    /// Returns an error if the file can't be read
    #[cfg(feature = "postcard")]
    pub fn from_bin_file(path: std::path::PathBuf) -> Result<Show, postcard::Error> {
        use postcard::from_bytes;

        let file = match std::fs::read(path) {
            Ok(val) => val,
            Err(_err) => return Err(postcard::Error::SerdeDeCustom),
        };

        from_bytes(&file)
    }

    /// Load a [Show] from a show.json file
    /// Returns an error if the file can't be read or if the json is invalid utf8
    #[cfg(feature = "json")]
    pub fn from_json_file(
        path: std::path::PathBuf,
    ) -> Result<(Show, usize), serde_json::error::Error> {
        use serde::de::Error;

        let file = match std::fs::read(path) {
            Ok(val) => val,
            Err(_err) => return Err(serde_json::error::Error::custom("file read error")),
        };

        serde_json::from_slice(&file)
    }
}

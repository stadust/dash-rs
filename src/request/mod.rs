//! Module containing all structs modelling requests to the Boomlings APIs.
//!
//! These directly implement (de)serialization into RobTop's data format, unlike models where
//! RobTop's eccentricities are hidden. This is since directly re-using these structs outside of
//! making/proxying requests for the boomlings servers seems rather useless to me, as they already
//! contain a lot of boomlings-specific fields.
//! This can also be edited for a specific GDPS, e.g 1.9 GDPS. (hi absowute :3)

use crate::{model::GameVersion, serde::RequestSerializer};
use serde::{Deserialize, Serialize};
use once_cell::sync::OnceCell;

static REQUEST_BASE: OnceLock<&'static str> = OnceLock::new();

pub fn get_gdps_url() -> String {
    GDPS_URL.get_or_init(|| BOOMLINGS_REQUEST_BASE)
}

pub const GD_21: BaseRequest = BaseRequest::new(
    GameVersion::Version { major: 2, minor: 1 },
    GameVersion::Version { major: 3, minor: 3 },
    "Wmfd2893gb7",
);

pub const GD_22: BaseRequest = BaseRequest::new(
    GameVersion::Version { major: 2, minor: 2 },
    GameVersion::Version { major: 3, minor: 8 },
    "Wmfd2893gb7",
);

#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BaseRequest<'a> {
    pub game_version: GameVersion,
    pub binary_version: GameVersion,
    pub secret: &'a str,
}

impl BaseRequest<'_> {
    pub const fn new(game_version: GameVersion, binary_version: GameVersion, secret: &'static str) -> BaseRequest<'static> {
        BaseRequest {
            game_version,
            binary_version,
            secret,
        }
    }
}

impl Default for BaseRequest<'static> {
    fn default() -> Self {
        GD_22
    }
}

pub(crate) fn to_string<S: Serialize>(request: S) -> String {
    let mut output = Vec::new();
    let mut serializer = RequestSerializer::new(&mut output);

    request.serialize(&mut serializer).unwrap();

    String::from_utf8(output).unwrap()
}

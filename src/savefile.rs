//! Module containing utilities for interacting with Geometry Dash's local save files

use std::{path::Path, io::Read};

use base64::{DecodeError, engine::general_purpose::URL_SAFE, Engine};
use flate2::read::GzDecoder;

use crate::util;

pub enum SavefileError {
    Io(std::io::Error),
    Base64(DecodeError)
}

/// Reads the given path to a CCGameManager.dat file and decodes it.
pub fn load_cc_game_manager(p: impl AsRef<Path>) -> Result<String, SavefileError> {
    let mut cc_game_manager_bytes = std::fs::read_to_string(p)
        .map_err(SavefileError::Io)?
        .into_bytes();

    util::cyclic_xor(&mut cc_game_manager_bytes, &[0xB]);

    // Spurious nul-terminators at end of string, kinda scary
    while cc_game_manager_bytes.last() == Some(&0) {
        cc_game_manager_bytes.pop();
    }

    let decoded = URL_SAFE.decode(&cc_game_manager_bytes)
        .map_err(SavefileError::Base64)?;
    let mut decoder = GzDecoder::new(&decoded[..]);

    let mut result = String::new();

    decoder.read_to_string(&mut result)
        .map_err(SavefileError::Io)?;

    Ok(result)
}

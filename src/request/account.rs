use std::borrow::Cow;
use std::fmt;
use std::fmt::Formatter;
use serde::{Serialize};

pub const XOR_KEY: &str = "37526";
pub const CONTENT_TYPE: &str = "Content-Type";
pub const URL_FORM_ENCODED: &str = "application/x-www-form-urlencoded";

#[derive(Debug, Serialize, Default, PartialEq, Eq, Clone, Hash)]
pub struct AuthenticatedUser<'a> {
    /// The username of the authenticated user
    ///
    /// ## GD Internals:
    /// This field is called `userName` in the Boomlings API
    #[serde(rename = "userName")]
    pub user_name: &'a str,

    /// The account ID of the authenticated user
    ///
    /// ## GD Internals:
    /// This field is called `accountID` in the Boomlings API
    #[serde(rename = "accountID")]
    pub account_id: u64,

    /// The encrypted password of the authenticated user, this is sensitive data as it can be used to act as a user on endpoints requiring `gjp`
    ///
    /// ## GD Internals:
    /// This field is called `gjp` in the Boomlings API
    #[serde(rename = "gjp")]
    password_hash: Cow<'a, str>
}

impl<'a> AuthenticatedUser<'a> {
    pub fn new(user_name: &'a str, account_id: u64, password_hash: Cow<'a, str>) -> Self {
        AuthenticatedUser{
            user_name,
            account_id,
            password_hash,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthenticationError(String);

impl std::error::Error for AuthenticationError {}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

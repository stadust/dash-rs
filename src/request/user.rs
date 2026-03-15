//! Module containing request definitions for retrieving users

use crate::{
    model::creator::Creator,
    request::{endpoint_base_url, BaseRequest, GD_22},
};
use serde::Serialize;
use std::borrow::Cow;
use std::fmt::Display;
use crate::request::account::AuthenticatedUser;

pub const GET_USER_ENDPOINT: &str = "getGJUserInfo20.php";
pub const SEARCH_USER_ENDPOINT: &str = "getGJUsers20.php";

/// Struct modelled after a request to `getGJUserInfo20.php`.
///
/// In the geometry Dash API, this endpoint is used to download player profiles from the servers by
/// their account IDs
#[derive(Debug, Default, Clone, Serialize, Hash)]
pub struct UserRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// The authenticated user data
    authenticated_user: Option<AuthenticatedUser<'a>>,

    /// The **account ID** (_not_ user ID) of the users whose data to retrieve.
    ///
    /// ## GD Internals:
    /// This field is called `targetAccountID` in the Boomlings API
    #[serde(rename = "targetAccountID")]
    pub user: u64,
}

impl<'a> UserRequest<'a> {
    pub const fn new(user_id: u64) -> UserRequest<'a> {
        UserRequest {
            base: GD_22,
            authenticated_user: None,
            user: user_id,
        }
    }

    pub const fn with_authenticated_user(authenticated_user: AuthenticatedUser<'a>, user_id: u64) -> UserRequest<'a> {
        UserRequest {
            authenticated_user: Some(authenticated_user),
            base: GD_22,
            user: user_id,
        }
    }

    pub fn to_url(&self) -> String {
        format!("{}{}", endpoint_base_url(), GET_USER_ENDPOINT)
    }

    pub fn to_string(&self) -> String {
        super::to_string(&self)
    }
}

impl From<u64> for UserRequest<'_> {
    fn from(user_id: u64) -> Self {
        UserRequest::new(user_id)
    }
}

impl From<Creator<'_>> for UserRequest<'_> {
    fn from(creator: Creator<'_>) -> Self {
        UserRequest::from(creator.user_id)
    }
}

impl Display for UserRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", super::to_string(self))
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct UserSearchRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// Unknown, probably related to pagination
    ///
    /// ## GD Internals:
    /// This field is called `total` in the Boomlings API
    pub total: u32,

    /// The page of users to retrieve
    ///
    /// Since the behavior of the search function was changed to return only the user whose name
    /// matches the search string exactly (previous behavior was a prefix search), it is not
    /// possible to retrieve more than 1 user via this endpoint anymore, rendering the pagination
    /// parameters useless.
    ///
    /// ## GD Internals:
    /// This field is called `page` in the Boomlings API
    pub page: u32,

    /// The name of the user being searched for
    ///
    /// ## GD Internals:
    /// This field is called `str` in the Boomlings API
    #[serde(rename = "str")]
    pub search_string: Cow<'a, str>,
}

impl Display for UserSearchRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", super::to_string(self))
    }
}

impl<'a> UserSearchRequest<'a> {
    pub fn new(search_string: impl Into<Cow<'a, str>>) -> Self {
        UserSearchRequest {
            base: GD_22,
            total: 0,
            page: 0,
            search_string: search_string.into(),
        }
    }

    pub fn to_url(&self) -> String {
        format!("{}{}", endpoint_base_url(), SEARCH_USER_ENDPOINT)
    }

    pub fn to_string(&self) -> String {
        super::to_string(&self)
    }

}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use crate::request::account::AuthenticatedUser;
    use crate::request::user::{UserRequest, UserSearchRequest};

    #[test]
    fn serialize_user_request() {
        let test_authenticated_user: AuthenticatedUser = AuthenticatedUser::new(
            "Ryder",
            57903,
            Cow::Borrowed("VGhpc0lzQUZha2VQYXNzd29yZA==")
        );

        let request = UserRequest::with_authenticated_user(test_authenticated_user, 57903);

        assert_eq!(
            request.to_string(),
            "gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&userName=TestUser&accountID=472634&gjp=VGhpc0lzQUZha2VQYXNzd29yZA==&targetAccountID=57903"
        );
    }

    #[test]
    fn serialize_user_search_request() {
        let request = UserSearchRequest::new("Ryder");

        assert_eq!(
            request.to_string(),
            "gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&total=0&page=0&str=Ryder"
        );
    }
}

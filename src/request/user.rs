//! Module containing request definitions for retrieving users

use reqwest::Error;
use crate::{
    request::{BaseRequest, GD_21, REQUEST_BASE_URL},
};
use serde::Serialize;
use crate::model::user::profile::Profile;
use crate::model::user::searched::SearchedUser;
use crate::request::AuthenticatedUser;
use crate::response::{parse_get_gj_user_info_response, parse_get_gj_users_response, ResponseError};

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
    pub authenticated_user: Option<AuthenticatedUser<'a>>,

    /// The **account ID** (_not_ user ID) of the users whose data to retrieve.
    ///
    /// ## GD Internals:
    /// This field is called `targetAccountID` in the boomlings API
    #[serde(rename = "targetAccountID")]
    pub user: u64,
}

impl<'a> UserRequest<'a> {

    fn to_url(&self) -> String {
        format!("{}{}", REQUEST_BASE_URL, GET_USER_ENDPOINT)
    }

    pub const fn new(user_id: u64) -> UserRequest<'a> {
        UserRequest {
            base: GD_21,
            authenticated_user: None,
            user: user_id,
        }
    }

    pub const fn with_authenticated_user(authenticated_user: AuthenticatedUser<'a>, user_id: u64) -> UserRequest<'a> {
        UserRequest {
            base: GD_21,
            authenticated_user: Some(authenticated_user),
            user: user_id,
        }
    }

    pub async fn get_response_body(&self) -> Result<String, Error> {
        super::execute(&self, &self.to_url()).await
    }

    pub async fn into_robtop(self, response_body: &str) -> Result<Profile, ResponseError> {
        parse_get_gj_user_info_response(response_body)
    }
}

#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq)]
pub struct UserSearchRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// Unknown, probably related to pagination
    ///
    /// ## GD Internals:
    /// This field is called `total` in the boomlings API
    pub total: u32,

    /// The page of users to retrieve
    ///
    /// Since the behavior of the search function was changed to return only the user whose name
    /// matches the search string exactly (previous behavior was a prefix search), it is not
    /// possible to retrieve more than 1 user via this endpoint anymore, rendering the pagination
    /// parameters useless.
    ///
    /// ## GD Internals:
    /// This field is called `page` in the boomlings API
    pub page: u32,

    /// The name of the user being searched for
    ///
    /// ## GD Internals:
    /// This field is called `str` in the boomlings API
    #[serde(rename = "str")]
    pub search_string: &'a str,
}

impl<'a> UserSearchRequest<'a> {

    fn to_url(&self) -> String {
        format!("{}{}", REQUEST_BASE_URL, SEARCH_USER_ENDPOINT)
    }

    pub const fn new(search_string: &'a str) -> Self {
        UserSearchRequest {
            base: GD_21,
            total: 0,
            page: 0,
            search_string,
        }
    }

    pub async fn get_response_body(&self) -> Result<String, Error> {
        super::execute(&self, &self.to_url()).await
    }

    pub async fn into_robtop(self, response_body: &str) -> Result<SearchedUser, ResponseError> {
        parse_get_gj_users_response(response_body)
    }
}

#[cfg(test)]
mod tests {
    use crate::request::user::{UserRequest, UserSearchRequest};

    #[test]
    fn serialize_user_request() {
        let request = UserRequest::new(57903);

        assert_eq!(
            super::super::to_string(request),
            "gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&targetAccountID=57903"
        );
    }

    #[test]
    fn serialize_user_search_request() {
        let request = UserSearchRequest::new("Ryder");

        assert_eq!(
            super::super::to_string(request),
            "gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&total=0&page=0&str=Ryder"
        );
    }
}

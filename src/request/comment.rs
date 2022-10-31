//! Module containing request structs for retrieving profile/level comments

use std::borrow::Cow;
use reqwest::{Error, Response};
use crate::{model::level::Level, request::{BaseRequest, GD_21, REQUEST_BASE_URL, AuthenticatedUser}, util};
use serde::Serialize;
use crate::request::Executable;
use async_trait::async_trait;

pub const LEVEL_COMMENTS_ENDPOINT: &str = "getGJComments21.php";
pub const PROFILE_COMMENT_ENDPOINT: &str = "getGJAccountComments20.php";
pub const COMMENT_HISTORY_ENDPOINT: &str = "getGJCommentHistory.php";
pub const UPLOAD_COMMENT_ENDPOINT: &str = "uploadGJComment21.php";
pub const DELETE_COMMENT_ENDPOINT: &str = "deleteGJComment20.php";

pub const COMMENT_CHK_SALT: &str = "xPT6iUrtws0J";
pub const COMMENT_XOR_CHK_KEY: &str = "29481";

/// The different orderings that can be requested for level comments
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(into = "u8")]
pub enum SortMode {
    /// Sort the comments by likes, in descending order
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `1` in the boomlings API
    Liked,

    /// Sort the comments from newest to oldest
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `0` in the boomlings APII
    Recent,
}

impl From<SortMode> for u8 {
    fn from(mode: SortMode) -> Self {
        match mode {
            SortMode::Liked => 1,
            SortMode::Recent => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Hash)]
pub struct LevelCommentsRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// Unknown, probably related to pagination
    ///
    /// ## GD Internals:
    /// This field is called `total` in the boomlings API
    pub total: u32,

    /// The page of users to retrieve. The first page is page `0`
    ///
    /// ## GD Internals:
    /// This field is called `page` in the boomlings API
    pub page: u32,

    /// What to sort by comments by
    ///
    /// ## GD Internals:
    /// This field is called `mode` in the boomlings API.
    #[serde(rename = "mode")]
    pub sort_mode: SortMode,

    /// The id of the level to retrieve the comments of
    ///
    /// ## GD Internals:
    /// This field is called `levelID` in the boomlings API
    #[serde(rename = "levelID")]
    pub level_id: u64,

    /// The amount of comments to retrieve. Note that while in-game this can only be set to 20 or 40
    /// (via the "load more comments option), the API accepts any value. So you can set it to
    /// something ridiculously high (like u32::MAX_VALUE) and retrieve all comments at once.
    ///
    /// ## GD Internals:
    /// This field is called `count` in the boomlings API
    #[serde(rename = "count")]
    pub limit: u32,
}

impl<'a> LevelCommentsRequest<'a> {
    const_setter!(total: u32);

    const_setter!(limit: u32);

    const_setter!(page: u32);

    pub fn to_url(&self) -> String {
        format!("{}{}", REQUEST_BASE_URL, LEVEL_COMMENTS_ENDPOINT)
    }

    pub const fn new(level: u64) -> Self {
        Self::with_base(GD_21, level)
    }

    pub const fn with_base(base: BaseRequest<'a>, level: u64) -> Self {
        LevelCommentsRequest {
            level_id: level,
            base,
            page: 0,
            total: 0,
            sort_mode: SortMode::Recent,
            limit: 20,
        }
    }

    pub const fn most_liked(mut self) -> Self {
        self.sort_mode = SortMode::Liked;
        self
    }

    pub const fn most_recent(mut self) -> Self {
        self.sort_mode = SortMode::Recent;
        self
    }
}

impl ToString for LevelCommentsRequest<'_> {
    fn to_string(&self) -> String {
        super::to_string(self)
    }
}

impl From<u64> for LevelCommentsRequest<'_> {
    fn from(level_id: u64) -> Self {
        LevelCommentsRequest::new(level_id)
    }
}

impl<S, U> From<Level<'_, S, U>> for LevelCommentsRequest<'_> {
    fn from(level: Level<'_, S, U>) -> Self {
        LevelCommentsRequest::from(level.level_id)
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
pub struct ProfileCommentsRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// Unknown, probably related to pagination
    ///
    /// ## GD Internals:
    /// This field is called `total` in the boomlings API
    pub total: u32,

    /// The page of users to retrieve. The first page is page `0`
    ///
    /// ## GD Internals:
    /// This field is called `page` in the boomlings API
    pub page: u32,

    /// The account id of the user to retrieve the comments of
    ///
    /// ## GD Internals:
    /// This field is called `accountID` in the boomlings API
    #[serde(rename = "accountID")]
    pub account_id: u64,
}

impl<'a> ProfileCommentsRequest<'a> {
    const_setter!(total: u32);

    const_setter!(page: u32);

    const_setter!(account_id: u64);

    pub fn to_url(&self) -> String {
        format!("{}{}", REQUEST_BASE_URL, PROFILE_COMMENT_ENDPOINT)
    }

    pub const fn new(account: u64) -> Self {
        Self::with_base(GD_21, account)
    }

    pub const fn with_base(base: BaseRequest<'a>, account: u64) -> Self {
        ProfileCommentsRequest {
            account_id: account,
            base,
            page: 0,
            total: 0,
        }
    }
}

impl ToString for ProfileCommentsRequest<'_> {
    fn to_string(&self) -> String {
        super::to_string(self)
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
pub struct CommentHistoryRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// Unknown, probably related to pagination
    ///
    /// ## GD Internals:
    /// This field is called `total` in the boomlings API
    pub total: u32,

    /// The page of comments to retrieve. The first page is page `0` and pages will contain `total` number of comments
    ///
    /// ## GD Internals:
    /// This field is called `page` in the boomlings API
    pub page: u32,

    /// What to sort by comments by
    ///
    /// ## GD Internals:
    /// This field is called `mode` in the boomlings API.
    #[serde(rename = "mode")]
    pub sort_mode: SortMode,

    /// The id of the player to retrieve comments, this is `not` the account ID
    ///
    /// ## GD Internals:
    /// This field is called `userID` in the boomlings API.
    #[serde(rename = "userID")]
    pub player_id: u64,

    /// The amount of comments to retrieve. Note that while in-game this can only be set to 20 or 40 however, a max of 100 comments can be returned
    /// ## GD Internals:
    /// This field is called `count` in the boomlings API
    #[serde(rename = "count")]
    pub limit: u32,
}

impl<'a> CommentHistoryRequest<'a> {
    const_setter!(limit: u32);
    const_setter!(page: u32);

    pub fn to_url(&self) -> String {
        format!("{}{}", REQUEST_BASE_URL, COMMENT_HISTORY_ENDPOINT)
    }

    pub const fn with_base(base: BaseRequest<'a>, player: u64) -> Self {
        CommentHistoryRequest {
            player_id: player,
            base,
            page: 0,
            total: 0,
            sort_mode: SortMode::Recent,
            limit: 20,
        }
    }

    pub const fn new(player: u64) -> Self {
        Self::with_base(GD_21, player)
    }

    pub const fn sort_mode(mut self, sort_mode: SortMode) -> Self {
        self.sort_mode = sort_mode;
        self
    }
}

impl<'a> ToString for CommentHistoryRequest<'a> {
    fn to_string(&self) -> String {
        super::to_string(self)
    }
}

#[async_trait]
impl<'a> Executable for CommentHistoryRequest<'a> {
    async fn execute(&self) -> Result<Response, Error> {
        let reqwest_client = reqwest::Client::new();
        println!("{:?}", self.to_string());
        println!("{:?}", self.to_url());
        reqwest_client
            .post(self.to_url())
            .body(self.to_string())
            .header(super::CONTENT_TYPE, super::URL_FORM_ENCODED)
            .send()
            .await
    }
}

#[derive(Debug, Clone, Serialize, Hash)]
pub struct UploadCommentRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// The authenticated user data
    pub authenticated_user: AuthenticatedUser<'a>,

    /// The content of the comment, this value will be base64 url encoded
    pub comment: Cow<'a, str>,

    /// The id of the level the comment to upload is posted to
    /// ## GD Internals:
    /// This field is called `levelID` in the boomlings API
    #[serde(rename = "levelID")]
    pub level_id: u64,

    /// The percent completed to display on the comment, this should be a number between 0 or 100 if present
    pub percent: u8,

    /// The CHK for /uploadGJComment21.php
    pub chk: Cow<'a, str>
}

impl<'a> UploadCommentRequest<'a> {
    const_setter!(level_id: u64);
    const_setter!(percent: u8);

    pub fn new(authenticated_user: AuthenticatedUser<'a>, level_id: u64) -> Self {
        Self::with_base(GD_21, authenticated_user, level_id)
    }

    pub const fn with_base(base: BaseRequest<'a>, authenticated_user: AuthenticatedUser<'a>, level_id: u64) -> Self {
        UploadCommentRequest{
            base,
            authenticated_user,
            comment: Cow::Borrowed(""),
            level_id,
            percent: 0,
            chk: Cow::Borrowed("")
        }
    }

    pub fn comment(mut self, comment_content: String) -> Self {
        self.comment = base64::encode_config(comment_content.as_bytes(), base64::URL_SAFE).into();
        self
    }

    pub fn to_url(&self) -> String {
        format!("{}{}", REQUEST_BASE_URL, UPLOAD_COMMENT_ENDPOINT)
    }

    fn generate_chk(mut self) -> Self {
        self.chk = format!("{}{}{}{}{}{}", self.authenticated_user.user_name, self.comment, self.level_id, self.percent, 0, COMMENT_CHK_SALT)
            .into();

        let xor_chk = util::xor(util::sha_encrypt(self.chk).as_bytes().to_vec(), COMMENT_XOR_CHK_KEY.as_bytes());
        self.chk = base64::encode_config(xor_chk.as_slice(), base64::URL_SAFE).into();
        self
    }
}

impl ToString for UploadCommentRequest<'_> {
    fn to_string(&self) -> String {
        super::to_string(self)
    }
}

#[async_trait]
impl<'a> Executable for UploadCommentRequest<'a> {
    async fn execute(&self) -> Result<Response, Error> {

        let reqwest_client = reqwest::Client::new();
        println!("{:?}", self.to_string());
        println!("{:?}", self.to_url());
        reqwest_client
            .post(self.to_url())
            .body(self.to_string())
            .header(super::CONTENT_TYPE, super::URL_FORM_ENCODED)
            .send()
            .await
    }
}

#[derive(Debug, Clone, Serialize, Hash)]
pub struct DeleteCommentRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// The authenticated user data
    pub authenticated_user: AuthenticatedUser<'a>,

    /// The id of the level comment to delete
    /// ## GD Internals:
    /// This field is called `commentID` in the boomlings API
    #[serde(rename = "commentID")]
    pub comment_id: u64,

    /// The id of the level the comment to delete is posted to
    /// ## GD Internals:
    /// This field is called `levelID` in the boomlings API
    #[serde(rename = "levelID")]
    pub level_id: u64,
}

impl<'a> DeleteCommentRequest<'a> {
    const_setter!(comment_id: u64);
    const_setter!(level_id: u64);

    pub fn new(authenticated_user: AuthenticatedUser<'a>, level_id: u64) -> Self {
        Self::with_base(GD_21, authenticated_user, level_id)
    }

    pub const fn with_base(base: BaseRequest<'a>, authenticated_user: AuthenticatedUser<'a>, level_id: u64) -> Self {
        DeleteCommentRequest {
            base,
            authenticated_user,
            comment_id: 0,
            level_id
        }
    }
    
    pub fn to_url(&self) -> String {
        format!("{}{}", REQUEST_BASE_URL, DELETE_COMMENT_ENDPOINT)
    }
}

#[async_trait]
impl<'a> Executable for DeleteCommentRequest<'a> {
    async fn execute(&self) -> Result<Response, Error> {
        let reqwest_client = reqwest::Client::new();
        println!("{:?}", self.to_string());
        println!("{:?}", self.to_url());
        reqwest_client
            .post(self.to_url())
            .body(self.to_string())
            .header(super::CONTENT_TYPE, super::URL_FORM_ENCODED)
            .send()
            .await
    }
}

impl ToString for DeleteCommentRequest<'_> {
    fn to_string(&self) -> String {
        super::to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::request::comment::{LevelCommentsRequest, ProfileCommentsRequest, CommentHistoryRequest, SortMode};
    use crate::request::Executable;
    use crate::response::parse_get_gj_comments_response;

    #[test]
    fn serialize_level_comments() {
        if let Err(err) = env_logger::builder().is_test(true).try_init() {
            // nothing to make the tests fail over
            eprintln!("Error setting up env_logger: {:?}", err)
        }

        let request = LevelCommentsRequest::new(1234).most_liked().page(2).limit(15);

        assert_eq!(
            super::super::to_string(request),
            "gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&total=0&page=2&mode=1&levelID=1234&count=15"
        );
    }

    #[test]
    fn serialize_profile_comments() {
        if let Err(err) = env_logger::builder().is_test(true).try_init() {
            // nothing to make the tests fail over
            eprintln!("Error setting up env_logger: {:?}", err)
        }

        let request = ProfileCommentsRequest::new(1710032).page(2);

        assert_eq!(
            super::super::to_string(request),
            "gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&total=0&page=2&accountID=1710032"
        );
    }

    #[test]
    fn serialize_comment_history() {
        if let Err(err) = env_logger::builder().is_test(true).try_init() {
            // nothing to make the tests fail over
            eprintln!("Error setting up env_logger: {:?}", err)
        }

        let request = CommentHistoryRequest::new(159782398)
            .limit(2);

        assert_eq!(super::super::to_string(request), "gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&total=0&page=0&mode=0&userID=159782398&count=2")
    }

    #[tokio::test]
    async fn upload_comment() {
        let request = crate::request::account::LoginRequest::default()
            .user_name("Ryder")
            .password("PASSHERE");

        let login_response = request.to_authenticated_user()
            .await
            .unwrap();

        let comment_upload_request = crate::request::comment::UploadCommentRequest::new(login_response, 85179632)
            .comment(String::from("More tests still ignore me"))
            .percent(69)
            .generate_chk()
            .execute()
            .await
            .unwrap()
            .text().await.unwrap();

        println!("{:?}", comment_upload_request)
    }

    #[tokio::test]
    async fn delete_comment() {
        let request = crate::request::account::LoginRequest::default()
            .user_name("Ryder")
            .password("PASSHERE");

        let login_response = request.to_authenticated_user()
            .await
            .unwrap();


        let comment_history_request = CommentHistoryRequest::new(3713125)
            .sort_mode(SortMode::Recent)
            .limit(1)
            .page(0);

        let comment_history_response = comment_history_request.execute()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        let comment = parse_get_gj_comments_response(comment_history_response.as_str())
            .unwrap();

        let comment_id = comment.get(0).unwrap().comment_id;

        println!("{}", &comment_id);

        let comment_delete_request = crate::request::comment::DeleteCommentRequest::new(login_response, 85179632)
            .comment_id(comment_id);

        let comment_delete_response = comment_delete_request.execute()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        println!("{}", comment_delete_response)
    }
}

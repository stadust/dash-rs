//! Module containing request structs for retrieving profile/level comments

use std::borrow::Cow;
use crate::{request::{endpoint_base_url, BaseRequest, GD_22}, util};
use serde::Serialize;
use std::fmt::Display;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use crate::request::account::AuthenticatedUser;

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
    /// This variant is represented by the numeric value `1` in the Boomlings API
    Liked,

    /// Sort the comments from newest to oldest
    ///
    /// ## GD Internals:
    /// This variant is represented by the numeric value `0` in the Boomlings API
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
    /// This field is called `total` in the Boomlings API
    pub total: u32,

    /// The page of users to retrieve. The first page is page `0`
    ///
    /// ## GD Internals:
    /// This field is called `page` in the Boomlings API
    pub page: u32,

    /// What to sort by comments by
    ///
    /// ## GD Internals:
    /// This field is called `mode` in the Boomlings API.
    #[serde(rename = "mode")]
    pub sort_mode: SortMode,

    /// The id of the level to retrieve the comments of
    ///
    /// ## GD Internals:
    /// This field is called `levelID` in the Boomlings API
    #[serde(rename = "levelID")]
    pub level_id: u64,

    /// The amount of comments to retrieve. Note that while in-game this can only be set to 20 or 40
    /// (via the "load more comments option), the API accepts any value. The max value for this is 100
    ///
    /// ## GD Internals:
    /// This field is called `count` in the Boomlings API
    #[serde(rename = "count")]
    pub limit: u32,
}

impl<'a> LevelCommentsRequest<'a> {
    const_setter!(total: u32);

    const_setter!(limit: u32);

    const_setter!(page: u32);

    pub fn to_url(&self) -> String {
        format!("{}{}", endpoint_base_url(), LEVEL_COMMENTS_ENDPOINT)
    }

    pub const fn new(level: u64) -> Self {
        Self::with_base(GD_22, level)
    }

    const fn with_base(base: BaseRequest<'a>, level: u64) -> Self {
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

impl Display for LevelCommentsRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", super::to_string(self))
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
pub struct ProfileCommentsRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// Unknown, probably related to pagination
    ///
    /// ## GD Internals:
    /// This field is called `total` in the bBomlings API
    pub total: u32,

    /// The page of users to retrieve. The first page is page `0`
    ///
    /// ## GD Internals:
    /// This field is called `page` in the Boomlings API
    pub page: u32,

    /// The account id of the user to retrieve the comments of
    ///
    /// ## GD Internals:
    /// This field is called `accountID` in the Boomlings API
    #[serde(rename = "accountID")]
    pub account_id: u64,
}

impl<'a> ProfileCommentsRequest<'a> {
    const_setter!(total: u32);

    const_setter!(page: u32);

    const_setter!(account_id: u64);

    pub fn to_url(&self) -> String {
        format!("{}{}", endpoint_base_url(), PROFILE_COMMENT_ENDPOINT)
    }

    pub const fn new(account: u64) -> Self {
        Self::with_base(GD_22, account)
    }

    const fn with_base(base: BaseRequest<'a>, account: u64) -> Self {
        ProfileCommentsRequest {
            account_id: account,
            base,
            page: 0,
            total: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Hash)]
pub struct CommentHistoryRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// Unknown, probably related to pagination
    ///
    /// ## GD Internals:
    /// This field is called `total` in the Boomlings API
    pub total: u32,

    /// The page of comments to retrieve. The first page is page `0` and pages will contain `total` number of comments
    ///
    /// ## GD Internals:
    /// This field is called `page` in the Boomlings API
    pub page: u32,

    /// The amount of comments to retrieve. Note that while in-game this can only be set to 20 or 40 however, a max of 100 comments can be returned
    /// ## GD Internals:
    /// This field is called `count` in the Boomlings API
    pub count: u32,

    /// What to sort by comments by
    ///
    /// ## GD Internals:
    /// This field is called `mode` in the Boomlings API.
    #[serde(rename = "mode")]
    pub sort_mode: SortMode,

    /// The id of the player to retrieve comments, this is `not` the account ID
    ///
    /// ## GD Internals:
    /// This field is called `userID` in the Boomlings API.
    #[serde(rename = "userID")]
    pub player_id: u64,
}

impl<'a> CommentHistoryRequest<'a> {
    const_setter!(page: u32);

    const_setter!(count: u32);

    pub const fn with_base(base: BaseRequest<'a>, player: u64) -> Self {
        CommentHistoryRequest {
            player_id: player,
            base,
            page: 0,
            count: 10,
            total: 0,
            sort_mode: SortMode::Recent,
        }
    }

    pub const fn new(player: u64) -> Self {
        Self::with_base(GD_22, player)
    }

    pub fn to_url(&self) -> String {
        format!("{}{}", endpoint_base_url(), COMMENT_HISTORY_ENDPOINT)
    }

    pub const fn sort_mode(mut self, sort_mode: SortMode) -> Self {
        self.sort_mode = sort_mode;
        self
    }
}

#[derive(Debug, Clone, Serialize, Hash)]
pub struct UploadCommentRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// The authenticated user data
    authenticated_user: AuthenticatedUser<'a>,

    /// The content of the comment, this value will be base64 url encoded
    pub comment: Cow<'a, str>,

    /// The id of the level the comment to upload is posted to
    /// ## GD Internals:
    /// This field is called `levelID` in the Boomlings API
    #[serde(rename = "levelID")]
    pub level_id: u64,

    /// The percent completed to display on the comment, this should be a number between 0 or 100 if present
    pub percent: u8,
}

impl<'a> UploadCommentRequest<'a> {
    const_setter!(level_id: u64);

    const_setter!(percent: u8);

    pub fn to_url(&self) -> String {
        format!("{}{}", endpoint_base_url(), UPLOAD_COMMENT_ENDPOINT)
    }

    pub fn new(authenticated_user: AuthenticatedUser<'a>, level_id: u64) -> Self {
        Self::with_base(GD_22, authenticated_user, level_id)
    }

    const fn with_base(base: BaseRequest<'a>, authenticated_user: AuthenticatedUser<'a>, level_id: u64) -> Self {
        UploadCommentRequest{
            base,
            authenticated_user,
            comment: Cow::Borrowed(""),
            level_id,
            percent: 0,
        }
    }

    pub fn comment(mut self, comment_content: &str) -> Self {
        self.comment = URL_SAFE.encode(comment_content.as_bytes()).into();
        self
    }

    fn generate_chk(&self) -> Cow<'a, str> {
        let chk: Cow<'a, str> = format!(
            "{}{}{}{}{}{}",
            self.authenticated_user.user_name,
            self.comment,
            self.level_id,
            self.percent,
            0,
            COMMENT_CHK_SALT
        )
            .into();

        let xor_chk = util::xor(util::sha_encrypt(&chk).as_bytes().to_vec(), COMMENT_XOR_CHK_KEY.as_bytes());
        URL_SAFE.encode(xor_chk.as_slice()).into()
    }

    pub fn to_string(&self) -> String {
        format!("{}&chk={}", super::to_string(&self), self.generate_chk())
    }
}

#[derive(Debug, Clone, Serialize, Hash)]
pub struct DeleteCommentRequest<'a> {
    /// The base request data
    pub base: BaseRequest<'a>,

    /// The authenticated user data
    authenticated_user: AuthenticatedUser<'a>,

    /// The id of the level comment to delete
    /// ## GD Internals:
    /// This field is called `commentID` in the Boomlings API
    #[serde(rename = "commentID")]
    pub comment_id: u64,

    /// The id of the level the comment to delete is posted to
    /// ## GD Internals:
    /// This field is called `levelID` in the Boomlings API
    #[serde(rename = "levelID")]
    pub level_id: u64,
}

impl<'a> DeleteCommentRequest<'a> {
    const_setter!(comment_id: u64);

    const_setter!(level_id: u64);

    pub fn new(authenticated_user: AuthenticatedUser<'a>, level_id: u64, comment_id: u64) -> Self {
        Self::with_base(GD_22, authenticated_user, level_id, comment_id)
    }

    const fn with_base(base: BaseRequest<'a>, authenticated_user: AuthenticatedUser<'a>, level_id: u64, comment_id: u64) -> Self {
        DeleteCommentRequest {
            base,
            authenticated_user,
            comment_id,
            level_id
        }
    }

    pub fn to_url(&self) -> String {
        format!("{}{}", endpoint_base_url(), DELETE_COMMENT_ENDPOINT)
    }
}

impl Display for ProfileCommentsRequest<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", super::to_string(self))
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;
    use crate::request::comment::{LevelCommentsRequest, ProfileCommentsRequest, CommentHistoryRequest, UploadCommentRequest, DeleteCommentRequest, SortMode};
    use crate::request::account::AuthenticatedUser;

    const TEST_AUTHENTICATED_USER: AuthenticatedUser = AuthenticatedUser::new(
        "Ryder",
        57903,
        Cow::Borrowed("UmVkaXNuZU1FQXJFREdlTnRJQw==")
    );

    #[test]
    fn serialize_level_comments() {
        if let Err(err) = env_logger::builder().is_test(true).try_init() {
            // nothing to make the tests fail over
            eprintln!("Error setting up env_logger: {:?}", err)
        }

        let request = LevelCommentsRequest::new(1234).most_liked().page(2).limit(15);

        assert_eq!(
            super::super::to_string(request),
            "gameVersion=22&binaryVersion=38&secret=Wmfd2893gb7&total=0&page=2&mode=1&levelID=1234&count=15"
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
            "gameVersion=22&binaryVersion=38&secret=Wmfd2893gb7&total=0&page=2&accountID=1710032"
        );
    }

    #[test]
    fn serialize_comment_history() {
        if let Err(err) = env_logger::builder().is_test(true).try_init() {
            // nothing to make the tests fail over
            eprintln!("Error setting up env_logger: {:?}", err)
        }

        let request = CommentHistoryRequest::new(159782398)
            .sort_mode(SortMode::Recent)
            .page(0)
            .count(2);

        assert_eq!(
            request.to_string(),
            "gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&total=0&page=0&count=2&mode=0&userID=159782398"
        );
    }

    #[test]
    fn serialize_upload_comment() {
        if let Err(err) = env_logger::builder().is_test(true).try_init() {
            // nothing to make the tests fail over
            eprintln!("Error setting up env_logger: {:?}", err)
        }

        let request = UploadCommentRequest::new(TEST_AUTHENTICATED_USER, 85179632)
            .comment("This is a test comment")
            .percent(56);

        assert_eq!(
            request.to_string(),
            "gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&userName=TestUser&accountID=472634&gjp=VGhpc0lzQUZha2VQYXNzd29yZA==&comment=VGhpcyBpcyBhIHRlc3QgY29tbWVudA==&levelID=85179632&percent=56&chk=UQsGAAEACgQBVQBaAwoGVwtSDQIEWAYOUFEAVQoIBVtWDwEHDQEJVA=="
        );
    }

    #[test]
    fn serialize_delete_comment() {
        if let Err(err) = env_logger::builder().is_test(true).try_init() {
            // nothing to make the tests fail over
            eprintln!("Error setting up env_logger: {:?}", err)
        }

        let request = DeleteCommentRequest::new(TEST_AUTHENTICATED_USER, 85179632, 7000000);

        assert_eq!(
            request.to_string(),
            "gameVersion=21&binaryVersion=33&secret=Wmfd2893gb7&userName=TestUser&accountID=472634&gjp=VGhpc0lzQUZha2VQYXNzd29yZA==&commentID=7000000&levelID=85179632"
        );
    }
}


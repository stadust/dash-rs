use serde::{Deserialize, Serialize};

pub mod profile;
pub mod searched;

/// Enum representing the different types of moderator a user can be
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum ModLevel {
    /// User isn't a moderator
    None,

    /// User is a normal moderator
    Normal,

    /// User is an elder moderator
    Elder,

    /// Unknown or invalid value. This variant will be constructed if robtop ever adds more
    /// moderator levels and will hold the internal game value associated with the new moderator
    /// level
    Unknown(u8),
}

impl From<ModLevel> for u8 {
    fn from(level: ModLevel) -> u8 {
        match level {
            ModLevel::None => 0,
            ModLevel::Normal => 1,
            ModLevel::Elder => 2,
            ModLevel::Unknown(inner) => inner,
        }
    }
}

impl From<u8> for ModLevel {
    fn from(i: u8) -> Self {
        match i {
            0 => ModLevel::None,
            1 => ModLevel::Normal,
            2 => ModLevel::Elder,
            _ => ModLevel::Unknown(i),
        }
    }
}

/// The type of icon displayed next a user's comment or next to their search result
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IconType {
    Cube,
    Ship,
    Ball,
    Ufo,
    Wave,
    Robot,
    Spider,
    Unknown(u8),
}

impl From<u8> for IconType {
    fn from(i: u8) -> Self {
        match i {
            0 => IconType::Cube,
            1 => IconType::Ship,
            2 => IconType::Ball,
            3 => IconType::Ufo,
            4 => IconType::Wave,
            5 => IconType::Robot,
            6 => IconType::Spider,
            i => IconType::Unknown(i),
        }
    }
}

impl From<IconType> for u8 {
    fn from(mode: IconType) -> u8 {
        match mode {
            IconType::Cube => 0,
            IconType::Ship => 1,
            IconType::Ball => 2,
            IconType::Ufo => 3,
            IconType::Wave => 4,
            IconType::Robot => 5,
            IconType::Spider => 6,
            IconType::Unknown(idx) => idx,
        }
    }
}

/// The user's in game message privacy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageState {

    /// Messages are open to all users
    /// This is provided by value 0 by boomlings response for messageState
    Anyone,

    /// Messages are only open to friends
    /// This is provided by value 1 by boomlings response for messageState
    FriendsOnly,

    /// Messages cannot be sent to this user
    /// This is provided by value 2 by boomlings response for messageState
    NoOne,

    /// Unknown or invalid value
    Unknown(u8)
}

impl From<MessageState> for u8 {
    fn from(state: MessageState) -> u8 {
        match state {
            MessageState::Anyone => 0,
            MessageState::FriendsOnly => 1,
            MessageState::NoOne => 2,
            MessageState::Unknown(inner) => inner,
        }
    }
}

impl From<u8> for MessageState {
    fn from(i: u8) -> Self {
        match i {
            0 => MessageState::Anyone,
            1 => MessageState::FriendsOnly,
            2 => MessageState::NoOne,
            _ => MessageState::Unknown(i)
        }
    }
}

/// The user's in game message privacy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FriendsState {

    /// Friend requests are open to all users, this is provided by value 0 by boomlings response for friendsState
    Anyone,

    /// Friend requests cannot be sent to this user, this is provided by value 1 by boomlings response for friendsState
    NoOne,

    /// Unknown or invalid value
    Unknown(u8)
}

impl From<FriendsState> for u8 {
    fn from(state: FriendsState) -> u8 {
        match state {
            FriendsState::Anyone => 0,
            FriendsState::NoOne => 1,
            FriendsState::Unknown(inner) => inner,
        }
    }
}

impl From<u8> for FriendsState {
    fn from(i: u8) -> Self {
        match i {
            0 => FriendsState::Anyone,
            1 => FriendsState::NoOne,
            _ => FriendsState::Unknown(i)
        }
    }
}

/// The user's status of a friend request for a given [`AuthenticatedUser`]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FriendState {

    /// The [`AuthenticatedUser`] has not sent a friend request to this profile
    /// This is provided by value 0 by boomlings response for friendsState
    NoRequest,

    /// The [`AuthenticatedUser`] is already friends with this profile
    /// This is provided by value 1 by boomlings response for friendsState
    AlreadyFriends,

    /// The [`AuthenticatedUser`] has sent a request to this profile but the profile has not responded
    /// This is provided by value 3 by boomlings response for friendsState
    RequestPendingByProfile,

    /// The profile has sent a request to the [`AuthenticatedUser`] but the [`AuthenticatedUser`] has not responded
    /// This is provided by value 4 by boomlings response for friendsState
    RequestPendingByUser,

    /// Unknown or invalid value
    Unknown(u8)
}

impl From<FriendState> for u8 {
    fn from(state: FriendState) -> u8 {
        match state {
            FriendState::NoRequest => 0,
            FriendState::AlreadyFriends => 1,
            FriendState::RequestPendingByProfile => 3,
            FriendState::RequestPendingByUser => 4,
            FriendState::Unknown(inner) => inner,
        }
    }
}

impl From<u8> for FriendState {
    fn from(i: u8) -> Self {
        match i {
            0 => FriendState::NoRequest,
            1 => FriendState::AlreadyFriends,
            3 => FriendState::RequestPendingByProfile,
            4 => FriendState::RequestPendingByUser,
            _ => FriendState::Unknown(i),
        }
    }
}

/// The user's comment history privacy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommentHistoryState {

    /// Messages are open to all users
    /// This is provided by value 0 by boomlings response for messageState
    Anyone,

    /// Messages are only open to friends
    /// This is provided by value 1 by boomlings response for messageState
    FriendsOnly,

    /// Messages cannot be sent to this user
    /// This is provided by value 2 by boomlings response for messageState
    NoOne,

    /// Unknown or invalid value
    Unknown(u8)
}

impl From<CommentHistoryState> for u8 {
    fn from(state: CommentHistoryState) -> u8 {
        match state {
            CommentHistoryState::Anyone => 0,
            CommentHistoryState::FriendsOnly => 1,
            CommentHistoryState::NoOne => 2,
            CommentHistoryState::Unknown(inner) => inner,
        }
    }
}

impl From<u8> for CommentHistoryState {
    fn from(i: u8) -> Self {
        match i {
            0 => CommentHistoryState::Anyone,
            1 => CommentHistoryState::FriendsOnly,
            2 => CommentHistoryState::NoOne,
            _ => CommentHistoryState::Unknown(i),
        }
    }
}

// Enum representing an in-game icon color
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Serialize, Deserialize)]
pub enum Color {
    /// A color whose index was known to dash-rs which could be converted to RGB values
    Known(u8, u8, u8),

    /// The index of some unknown colors. This variant will be constructed if robtop ever adds more
    /// colors and while dash-rs hasn't updated yet
    Unknown(u8),
}

impl From<u8> for Color {
    fn from(idx: u8) -> Self {
        // This match expression is listing the colors in order of the in-game selection menu!
        match idx {
            0 => Color::Known(125, 255, 0),
            1 => Color::Known(0, 255, 0),
            2 => Color::Known(0, 255, 125),
            3 => Color::Known(0, 255, 255),
            16 => Color::Known(0, 200, 255),
            4 => Color::Known(0, 125, 255),
            5 => Color::Known(0, 0, 255),
            6 => Color::Known(125, 0, 255),
            13 => Color::Known(185, 0, 255),
            7 => Color::Known(255, 0, 255),
            8 => Color::Known(255, 0, 125),
            9 => Color::Known(255, 0, 0),
            29 => Color::Known(255, 75, 0),
            10 => Color::Known(255, 125, 0),
            14 => Color::Known(255, 185, 0),
            11 => Color::Known(255, 255, 0),
            12 => Color::Known(255, 255, 255),
            17 => Color::Known(175, 175, 175),
            18 => Color::Known(80, 80, 80),
            15 => Color::Known(0, 0, 0),
            27 => Color::Known(125, 125, 0),
            32 => Color::Known(100, 150, 0),
            28 => Color::Known(75, 175, 0),
            38 => Color::Known(0, 150, 0),
            20 => Color::Known(0, 175, 75),
            33 => Color::Known(0, 150, 100),
            21 => Color::Known(0, 125, 125),
            34 => Color::Known(0, 100, 150),
            22 => Color::Known(0, 75, 175),
            39 => Color::Known(0, 0, 150),
            23 => Color::Known(75, 0, 175),
            35 => Color::Known(100, 0, 150),
            24 => Color::Known(125, 0, 125),
            36 => Color::Known(150, 0, 100),
            25 => Color::Known(175, 0, 75),
            37 => Color::Known(150, 0, 0),
            30 => Color::Known(150, 50, 0),
            26 => Color::Known(175, 75, 0),
            31 => Color::Known(150, 100, 0),
            19 => Color::Known(255, 255, 125),
            40 => Color::Known(125, 255, 175),
            41 => Color::Known(125, 125, 255),
            idx => Color::Unknown(idx),
        }
    }
}

impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        // in this house we are thankful for regular expressions
        match color {
            Color::Known(125, 255, 0) => 0,
            Color::Known(0, 255, 0) => 1,
            Color::Known(0, 255, 125) => 2,
            Color::Known(0, 255, 255) => 3,
            Color::Known(0, 200, 255) => 16,
            Color::Known(0, 125, 255) => 4,
            Color::Known(0, 0, 255) => 5,
            Color::Known(125, 0, 255) => 6,
            Color::Known(185, 0, 255) => 13,
            Color::Known(255, 0, 255) => 7,
            Color::Known(255, 0, 125) => 8,
            Color::Known(255, 0, 0) => 9,
            Color::Known(255, 75, 0) => 29,
            Color::Known(255, 125, 0) => 10,
            Color::Known(255, 185, 0) => 14,
            Color::Known(255, 255, 0) => 11,
            Color::Known(255, 255, 255) => 12,
            Color::Known(175, 175, 175) => 17,
            Color::Known(80, 80, 80) => 18,
            Color::Known(0, 0, 0) => 15,
            Color::Known(125, 125, 0) => 27,
            Color::Known(100, 150, 0) => 32,
            Color::Known(75, 175, 0) => 28,
            Color::Known(0, 150, 0) => 38,
            Color::Known(0, 175, 75) => 20,
            Color::Known(0, 150, 100) => 33,
            Color::Known(0, 125, 125) => 21,
            Color::Known(0, 100, 150) => 34,
            Color::Known(0, 75, 175) => 22,
            Color::Known(0, 0, 150) => 39,
            Color::Known(75, 0, 175) => 23,
            Color::Known(100, 0, 150) => 35,
            Color::Known(125, 0, 125) => 24,
            Color::Known(150, 0, 100) => 36,
            Color::Known(175, 0, 75) => 25,
            Color::Known(150, 0, 0) => 37,
            Color::Known(150, 50, 0) => 30,
            Color::Known(175, 75, 0) => 26,
            Color::Known(150, 100, 0) => 31,
            Color::Known(255, 255, 125) => 19,
            Color::Known(125, 255, 175) => 40,
            Color::Known(125, 125, 255) => 41,
            Color::Unknown(idx) => idx,
            _ => 0, // default color
        }
    }
}

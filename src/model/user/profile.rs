use crate::model::user::{Color, CommentHistoryState, FriendsState, FriendState, IconType, MessageState, ModLevel};
use serde::{__private::Formatter, Deserialize, Serialize};
use std::{borrow::Cow, fmt::Display};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Youtube<'a>(pub Cow<'a, str>);

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Twitch<'a>(pub Cow<'a, str>);

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Twitter<'a>(pub Cow<'a, str>);

impl Display for Youtube<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "https://www.youtube.com/channel/{}", self.0)
    }
}

impl Display for Twitch<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "https://www.twitch.tv/{}", self.0)
    }
}

impl Display for Twitter<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "https://www.twitter.com/{}", self.0)
    }
}

/// Struct representing a Geometry Dash User's profile, as seen after clicking their name in the
/// official client
///
/// ## GD Internals:
/// The Geometry Dash servers provide user data in a `getGJUserInfo` response
///
/// ### Unused Indices
/// The following indices aren't used by the Geometry Dash servers: `5`, `6`, `7`, `9`, `12`, `14`,
/// `15`, `27`, `32`, `33`, `34`, `35`, `36`, `37`, `38`, `39`, `40`, `41`, `42`, `47`
#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Profile<'a> {
    /// The [`Profile`]'s name
    ///
    /// ## GD Internals:
    /// This value is provided at index `1`.
    pub name: Cow<'a, str>,

    /// The [`Profile`]'s unique user ID
    ///
    /// ## GD Internals:
    /// This value is provided at index `2`
    pub user_id: u64,

    /// The amount of stars this [`Profile`] has collected.
    ///
    /// ## GD Internals:
    /// This value is provided at index `3`
    pub stars: u32,

    /// The demons of stars this [`Profile`] has beaten.
    ///
    /// ## GD Internals:
    /// This value is provided at index `4`
    pub demons: u16,

    /// The global leaderboard position for this [`Profile`].
    /// This data is returned when requesting leaderboard data, otherwise `None`.
    ///
    /// ## GD Internals:
    /// This value is provided at index `6`
    pub leaderboard_position: Option<u64>,

    /// The account ID for this [`Profile`], just used to highlight leaderboard ranking.
    /// This data is returned when requesting leaderboard data, otherwise `None`.
    ///
    ///
    /// ## GD Internals:
    /// This value is provided at index `7`
    pub account_highlight: Option<u64>,

    /// The amount of creator points this [`Profile`] was awarded.
    ///
    /// ## GD Internals:
    /// This value is provided at index `8`
    pub creator_points: u16,

    /// The icon this [`Profile`] has displayed for leaderboards and comments.
    /// This data is returned when requesting leaderboard or comment data, otherwise `None`.
    ///
    /// ## GD Internals:
    /// This value is provided at index `9`
    pub icon_id: Option<u16>,

    /// This [`Profile`]'s primary color
    ///
    /// ## GD Internals:
    /// This value is provided at index `10`. The game internally assigned each color some really
    /// obscure ID that doesn't correspond to the index in the game's color selector at all, which
    /// makes it pretty useless. dash-rs thus translates all in-game colors into their RGB
    /// representation.
    pub primary_color: Color,

    /// This [`Profile`]'s secondary color
    ///
    /// ## GD Internals:
    /// This value is provided at index `11`. Same things as above apply
    pub secondary_color: Color,

    /// The amount of secret coins this [`Profile`] has collected.
    ///
    /// ## GD Internals:
    /// This value is provided at index `13`
    pub secret_coins: u8,

    /// The icon type this [`Profile`] has displayed for leaderboards and comments.
    /// This data is returned when requesting leaderboard or comment data, otherwise `None`.
    ///
    /// ## GD Internals:
    /// This value is provided at index `14`
    pub icon_type: Option<IconType>,

    /// The [`Profile`]'s unique account ID
    ///
    /// ## GD Internals:
    /// This value is provided at index `16`
    pub account_id: u64,

    /// The amount of user coins this [`Profile`] has collected.
    ///
    /// ## GD Internals:
    /// This value is provided at index `17`
    pub user_coins: u16,

    /// The privacy option for messages that this ['Profile'] has set
    /// ## GD Internals:
    /// This value is provided at index `18`
    pub message_state: MessageState,

    /// The privacy option for friends that this ['Profile'] has set
    /// ## GD Internals:
    /// This value is provided at index `19`
    pub friends_state: FriendsState,

    /// The link to the [`Profile`]'s [YouTube](https://youtube.com) channel, if provided
    ///
    /// ## GD Internals
    /// This value is provided at index `20`. The value provided is only the `username` section of an `https://www.youtube.com/user/{username}` URL
    pub youtube_url: Option<Youtube<'a>>,

    /// The 1-based index of the cube this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `21`
    pub cube_index: u16,

    /// The 1-based index of the ship this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `22`
    pub ship_index: u8,

    /// The 1-based index of the ball this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `23`
    pub ball_index: u8,

    /// The 1-based index of the UFO this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `24`
    pub ufo_index: u8,

    /// The 1-based index of the wave this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `25`
    pub wave_index: u8,

    /// The 1-based index of the robot this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `26`
    pub robot_index: u8,

    /// Values indicating whether this [`Profile`] has glow activated or not.
    ///
    /// ## GD Internals:
    /// This value is provided at index `28`, as an integer
    pub has_glow: bool,

    /// Flag for if this [`Profile`] is registered or not.
    ///
    /// ## GD Internals:
    /// This value is provided at index `29`, 0 if not registered, 1 if registed
    pub is_registered: bool,

    /// This [`Profile`]'s global rank. [`None`] if he is banned or not ranked.
    ///
    /// ## GD Internals:
    /// This value is provided at index `30`. For unranked/banned users it's `0`
    pub global_rank: Option<u32>,

    /// Holds the status of the friend request sent to this [`Profile`] by the [`AuthenticatedUser`].
    ///
    /// ## GD Internals:
    /// This value is provided at index `31`
    pub friend_state: FriendState,

    /// Number of unread messages this [`Profile`] has.
    /// This data is only returned when requesting a [`Profile`] as your [`AuthenticatedUser'].
    ///
    /// ## GD Internals:
    /// This value is provided at index `38`
    pub unread_messages_count: Option<u32>,

    /// Number of unread friend requests this [`Profile`] has.
    /// This data is only returned when requesting a [`Profile`] as your [`AuthenticatedUser'].
    ///
    /// ## GD Internals:
    /// This value is provided at index `39`
    pub unread_friend_request_count: Option<u32>,

    /// Number of new friend this [`Profile`] has.
    /// This data is only returned when requesting a [`Profile`] as your [`AuthenticatedUser'].
    ///
    /// ## GD Internals:
    /// This value is provided at index `40`
    pub new_friends_count: Option<u32>,

    /// Flag for if a friend request is new.
    /// This data is only returned when requesting a [`Profile`] as your [`AuthenticatedUser'].
    ///
    /// ## GD Internals:
    /// This value is provided at index `41`
    pub new_friend_request: Option<bool>,

    /// The time since this [`Profile`] has submitted a new level score.
    /// This data is only returned when requesting leaderboard data
    ///
    /// ## GD Internals:
    /// This value is provided at index `42`
    pub time_since_score_update: Option<Cow<'a, str>>,

    /// The 1-based index of the spider this [`Profile`] currently uses. Indexing of icons starts at
    /// the top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `43`
    pub spider_index: u8,

    /// The link to the [`Profile`]'s [Twitter](https://twitter.com) account, if provided
    ///
    /// ## GD Internals
    /// This value is provided at index `44`. The value provided is only the `username` section of an `https://www.twitter.com/{username}` URL
    pub twitter_url: Option<Twitter<'a>>,

    /// The link to the [`Profile`]'s [Twitch](https://twitch.tv) channel, if provided
    ///
    /// ## GD Internals
    /// This value is provided at index `45`. The value provided is only the `username` section of an `https://twitch.tv/{username}` URL
    pub twitch_url: Option<Twitch<'a>>,

    /// The amount of diamonds this [`Profile`] has collected.
    ///
    /// ## GD Internals:
    /// This value is provided at index `46`
    pub diamonds: u16,

    /// The 1-based index of the death-effect this [`Profile`] currently uses. Indexing of icons
    /// starts at the top left corner and then goes left-to-right and top-to-bottom
    ///
    /// ## GD Internals:
    /// This value is provided at index `48`
    pub death_effect_index: u8,

    /// The level of moderator this [`Profile`] is
    ///
    /// ## GD Internals:
    /// This value is provided at index `49`
    pub mod_level: ModLevel,

    /// The privacy option this for viewing comment history [`Profile`] has set.
    /// ## GD Internals:
    /// This value is provided at index `50`
    pub comment_history_state: CommentHistoryState,
}

mod internal {
    use crate::model::user::profile::{Profile, Twitch, Twitter, Youtube};

    #[allow(non_upper_case_globals, unused_imports)]
    const _profile: () = {
        use crate::{
            serde::{DeError, HasRobtopFormat, IndexedDeserializer, IndexedSerializer, PercentDecoded, SerError, Thunk, RefThunk, Base64Decoded},
        };
        use serde::{Deserialize, Serialize};
        use std::{borrow::{Cow, Borrow}, io::Write};
        #[derive(Serialize, Deserialize)]
        struct InternalProfile<'src> {
            #[serde(rename = "1")]
            index_1: &'src str,
            #[serde(rename = "2")]
            index_2: u64,
            #[serde(rename = "3")]
            index_3: u32,
            #[serde(rename = "4")]
            index_4: u16,
            #[serde(rename = "6")]
            index_6: Option<u64>,
            #[serde(rename = "7")]
            index_7: Option<u64>,
            #[serde(rename = "8")]
            index_8: u16,
            #[serde(rename = "9")]
            index_9: Option<u16>,
            #[serde(rename = "10")]
            index_10: u8,
            #[serde(rename = "11")]
            index_11: u8,
            #[serde(rename = "13")]
            index_13: u8,
            #[serde(rename = "14")]
            index_14: Option<u8>,
            #[serde(rename = "16")]
            index_16: u64,
            #[serde(rename = "17")]
            index_17: u16,
            #[serde(rename = "18")]
            index_18: u8,
            #[serde(rename = "19")]
            index_19: u8,
            #[serde(rename = "20")]
            index_20: Option<&'src str>,
            #[serde(rename = "21")]
            index_21: u16,
            #[serde(rename = "22")]
            index_22: u8,
            #[serde(rename = "23")]
            index_23: u8,
            #[serde(rename = "24")]
            index_24: u8,
            #[serde(rename = "25")]
            index_25: u8,
            #[serde(rename = "26")]
            index_26: u8,
            #[serde(rename = "28")]
            index_28: bool,
            #[serde(rename = "29")]
            index_29: bool,
            #[serde(rename = "30")]
            index_30: Option<u32>,
            #[serde(rename = "31")]
            index_31: u8,
            #[serde(rename = "38")]
            index_38: Option<u32>,
            #[serde(rename = "39")]
            index_39: Option<u32>,
            #[serde(rename = "40")]
            index_40: Option<u32>,
            #[serde(rename = "41")]
            index_41: Option<bool>,
            #[serde(rename = "42")]
            index_42: Option<&'src str>,
            #[serde(rename = "43")]
            index_43: u8,
            #[serde(rename = "44")]
            index_44: Option<&'src str>,
            #[serde(rename = "45")]
            index_45: Option<&'src str>,
            #[serde(rename = "46")]
            index_46: u16,
            #[serde(rename = "48")]
            index_48: u8,
            #[serde(rename = "49")]
            index_49: u8,
            #[serde(rename = "50")]
            index_50: u8,
        }
        impl<'src> HasRobtopFormat<'src> for Profile<'src> {
            fn from_robtop_str(input: &'src str) -> Result<Self, DeError> {
                let internal = InternalProfile::deserialize(&mut IndexedDeserializer::new(input, ":", true))?;
                Ok(Self {
                    name: Cow::Borrowed(internal.index_1),
                    user_id: internal.index_2,
                    stars: internal.index_3,
                    demons: internal.index_4,
                    leaderboard_position: internal.index_6,
                    account_highlight: internal.index_7,
                    creator_points: internal.index_8,
                    icon_id: internal.index_9,
                    primary_color: internal.index_10.into(),
                    secondary_color: internal.index_11.into(),
                    secret_coins: internal.index_13,
                    icon_type: match internal.index_14 {
                        None => None,
                        Some(icon_type_value) => Some(icon_type_value.into())
                    },
                    account_id: internal.index_16,
                    user_coins: internal.index_17,
                    message_state: internal.index_18.into(),
                    friends_state: internal.index_19.into(),
                    cube_index: internal.index_21,
                    ship_index: internal.index_22,
                    ball_index: internal.index_23,
                    ufo_index: internal.index_24,
                    wave_index: internal.index_25,
                    robot_index: internal.index_26,
                    has_glow: internal.index_28,
                    is_registered: internal.index_29,
                    global_rank: internal.index_30,
                    friend_state: internal.index_31.into(),
                    unread_messages_count: internal.index_38.into(),
                    unread_friend_request_count: internal.index_39,
                    new_friends_count: internal.index_40,
                    new_friend_request: internal.index_41,
                    time_since_score_update: match internal.index_42 {
                        None => None,
                        Some(age_value) => Some(Cow::Borrowed(age_value))
                    },
                    spider_index: internal.index_43,
                    diamonds: internal.index_46,
                    death_effect_index: internal.index_48,
                    mod_level: internal.index_49.into(),
                    comment_history_state: internal.index_50.into(),
                    youtube_url: internal.index_20.map(Cow::Borrowed).map(Youtube)
                    ,
                    twitch_url: internal.index_45.map(Cow::Borrowed).map(Twitch),
                    twitter_url: internal.index_44.map(Cow::Borrowed).map(Twitter)
                    ,
                })
            }
            fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {
                let internal = InternalProfile {
                    index_1: self.name.as_ref(),
                    index_2: self.user_id,
                    index_3: self.stars,
                    index_4: self.demons,
                    index_6: self.leaderboard_position,
                    index_7: self.account_highlight,
                    index_8: self.creator_points,
                    index_9: self.icon_id,
                    index_10: self.primary_color.into(),
                    index_11: self.secondary_color.into(),
                    index_13: self.secret_coins,
                    index_14: match self.icon_type {
                        None => None,
                        Some(icon_type_value) => Some(icon_type_value.into())
                    },
                    index_16: self.account_id,
                    index_17: self.user_coins,
                    index_18: self.message_state.into(),
                    index_19: self.friends_state.into(),
                    index_20: self.youtube_url.as_ref().map(|y| y.0.borrow()),
                    index_21: self.cube_index,
                    index_22: self.ship_index,
                    index_23: self.ball_index,
                    index_24: self.ufo_index,
                    index_25: self.wave_index,
                    index_26: self.robot_index,
                    index_28: self.has_glow,
                    index_29: self.is_registered.into(),
                    index_30: self.global_rank,
                    index_31: self.friend_state.into(),
                    index_38: self.unread_messages_count.into(),
                    index_39: self.unread_friend_request_count,
                    index_40: self.new_friends_count,
                    index_41: self.new_friend_request,
                    index_42: match &self.time_since_score_update {
                        None => None,
                        Some(age_value) => Some(age_value.as_ref())
                    },
                    index_43: self.spider_index,
                    index_44: self.twitter_url.as_ref().map(|y| y.0.borrow())
                    ,
                    index_45: self.twitch_url.as_ref().map(|y| y.0.borrow())
                    ,
                    index_46: self.diamonds,
                    index_48: self.death_effect_index,
                    index_49: self.mod_level.into(),
                    index_50: self.comment_history_state.into(),
                };
                internal.serialize(&mut IndexedSerializer::new(":", writer, true))
            }
        }
    };


    // include!(concat!(env!("OUT_DIR"), "/profile.boilerplate"));
}

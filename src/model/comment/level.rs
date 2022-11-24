use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::{
    model::user::{Color, IconType, ModLevel},
    Base64Decoded, Thunk,
};
use crate::model::RobtopForm;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct LevelComment<'a> {
    /// Information about the user that made this [`LevelComment`]. Is generally a [`CommentUser`]
    /// object
    pub user: Option<CommentUser<'a>>,

    /// The actual content of the [`LevelComment`] made.
    ///
    /// ## GD Internals
    /// This value is provided at index `2` and is base64 encoded
    #[serde(borrow)]
    pub content: Option<Thunk<'a, Base64Decoded<'a>>>,

    /// The unique user id of the player who made this [`LevelComment`]
    ///
    /// ## GD Internals
    /// This value is provided at index `3`
    pub user_id: u64,

    /// The amount of likes this [`LevelComment`] has received
    ///
    /// ## GD Internals
    /// This value is provided at index `4`
    pub likes: i32,

    /// The unique id of this [`LevelComment`]. Additionally, there is also no [`ProfileComment`]
    /// with this idea
    ///
    /// ## GD Internals
    /// This value is provided at index `6`
    pub comment_id: u64,

    /// Whether this [`LevelComment`] has been flagged as spam (because of having received too many
    /// dislikes or for other reasons)
    ///
    /// ## GD Internals
    /// This value is provided at index `7`
    pub is_flagged_spam: Option<bool>,

    /// Robtop's completely braindead way of keeping track of when this [`LevelComment`] was posted
    ///
    /// ## GD Internals
    /// This value is provided at index `9`
    pub time_since_post: Cow<'a, str>,

    /// If enabled by the user making this [`LevelComment`], the progress they have done on the
    /// level this comment is on.
    ///
    /// ## GD Internals
    /// This value is provided at index `10`
    pub progress: Option<u8>,

    /// The level of moderator the player that made this [`LevelComment`] is
    ///
    /// ## GD Internals
    /// This value is provided at index `11`
    pub mod_level: Option<ModLevel>,

    /// If this [`LevelComment`]'s text is displayed in a special color (blue for robtop, green for
    /// elder mods), the RGB code of that color will be stored here
    ///
    /// Note that the yellow color of comments made by the creator is not reported here.
    ///
    /// ## GD Internals
    /// This value is provided at index `12`
    pub special_color: Option<Color>,
}

impl<'a> RobtopForm for LevelComment<'a> {}

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize)]
pub struct CommentUser<'a> {
    /// This [`CommentUser`]'s name
    ///
    /// ## GD Internals
    /// This value is provided at index `1`
    pub name: Cow<'a, str>,

    /// The index of the icon being displayed.
    ///
    /// ## GD Internals
    /// This value is provided at index `9`
    pub icon_index: u16,

    /// This [`CommentUser`]'s primary color
    ///
    /// ## GD Internals:
    /// This value is provided at index `10`. The game internally assigned each color some really
    /// obscure ID that doesn't correspond to the index in the game's color selector at all, which
    /// makes it pretty useless. dash-rs thus translates all in-game colors into their RGB
    /// representation.
    /// ## GD Internals
    /// This value is provided at index `10`
    pub primary_color: Color,

    /// This [`CommentUser`]'s secondary color
    ///
    /// ## GD Internals
    /// This value is provided at index `11`. Same things as above apply
    pub secondary_color: Color,

    /// The type of icon being displayed
    ///
    /// ## GD Internals
    /// This value is provided at index `14`
    pub icon_type: IconType,

    /// Values indicating whether this [`CommentUser`] has glow activated or not.
    ///
    /// ## GD Internals
    /// This value is provided at index `15`, however the value `true` is encoded as `"2"` instead
    pub has_glow: bool,

    /// The [`CommentUser`]'s unique account ID
    ///
    /// ## GD Internals
    /// This value is provided at index `16`
    pub account_id: Option<u64>,
}

#[allow(unused_imports)]
mod internal {
    use std::borrow::{Borrow, Cow};

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::model::{
        comment::level::{CommentUser, LevelComment},
        user::Color,
    };
    use std::io::Write;
    use crate::model::user::ModLevel;

    struct RGBColor(u8, u8, u8);

    impl Serialize for RGBColor {
        fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&format!("{},{},{}", self.0, self.1, self.2))
        }
    }

    impl<'de> Deserialize<'de> for RGBColor {
        fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where
            D: Deserializer<'de>,
        {
            let color_string = <&str>::deserialize(deserializer)?;
            let mut split = color_string.split(',');

            if let (Some(r), Some(g), Some(b)) = (split.next(), split.next(), split.next()) {
                Ok(RGBColor(
                    r.parse().map_err(serde::de::Error::custom)?,
                    g.parse().map_err(serde::de::Error::custom)?,
                    b.parse().map_err(serde::de::Error::custom)?,
                ))
            } else {
                Err(serde::de::Error::custom(format!("Malformed color string {}", color_string)))
            }
        }
    }

    #[allow(non_upper_case_globals, unused_imports)]
    const _level_comment: () = {
        use crate::{
            serde::{DeError, HasRobtopFormat, IndexedDeserializer, IndexedSerializer, PercentDecoded, SerError, Thunk, RefThunk, Base64Decoded},
        };
        use serde::{Deserialize, Serialize};
        use std::{borrow::{Cow, Borrow}, io::Write};
        #[derive(Serialize, Deserialize)]
        struct InternalLevelComment<'src, 'bor> {
            #[serde(rename = "2")]
            index_2: Option<RefThunk<'src, 'bor, Base64Decoded<'src>>>,
            #[serde(rename = "3")]
            index_3: u64,
            #[serde(rename = "4")]
            index_4: i32,
            #[serde(rename = "6")]
            index_6: u64,
            #[serde(rename = "7")]
            index_7: Option<bool>,
            #[serde(rename = "9")]
            index_9: &'src str,
            #[serde(rename = "10")]
            index_10: Option<u8>,
            #[serde(rename = "11")]
            index_11: Option<u8>,
            #[serde(rename = "12")]
            index_12: Option<RGBColor>,
        }

        impl<'src> HasRobtopFormat<'src> for LevelComment<'src> {
            fn from_robtop_str(input: &'src str) -> Result<Self, DeError> {
                let internal = InternalLevelComment::deserialize(&mut IndexedDeserializer::new(input, "~", true))?;
                Ok(Self {
                    content: match internal.index_2 {
                        None => None,
                        Some (RefThunk::Unprocessed(unproc)) =>
                            Some(Thunk::Unprocessed(unproc)),
                        _ => unreachable!()},
                    user_id: internal.index_3,
                    likes: internal.index_4,
                    comment_id: internal.index_6,
                    is_flagged_spam: internal.index_7,
                    time_since_post: Cow::Borrowed(internal.index_9),
                    progress: internal.index_10,
                    mod_level: match internal.index_11 {
                        None =>
                            None,
                        Some(mod_level) => {
                            Some(mod_level.into())
                        }
                    },
                    special_color: internal.index_12.map(|RGBColor(r, g, b)| Color::Known(r, g, b)),
                    user: None,
                })
            }

            fn write_robtop_data<W: Write>(&self, writer: W) -> Result<(), SerError> {
                let internal = InternalLevelComment {
                    index_2: self.content.as_ref().map(|t| t.as_ref_thunk()),
                    index_3: self.user_id,
                    index_4: self.likes,
                    index_6: self.comment_id,
                    index_7: self.is_flagged_spam,
                    index_9: self.time_since_post.as_ref(),
                    index_10: self.progress,
                    index_11: match self.mod_level {
                        None => None,
                        Some(ModLevel::Normal) => Some(1),
                        Some(ModLevel::Elder) => Some(2),
                        Some(ModLevel::Unknown(value)) => Some(value),
                        _ => unreachable!()
                    },
                    index_12: self.special_color.map(|color| {
                        match color {
                            Color::Known(r, g, b) => RGBColor(r, g, b),
                            _ => panic!("Color::Unknown passed as color of level comment"),
                        }
                    })
                    ,
                };
                internal.serialize(&mut IndexedSerializer::new("~", writer, true))
            }
        }
    };

//    include!(concat!(env!("OUT_DIR"), "/level_comment.boilerplate"));
    include!(concat!(env!("OUT_DIR"), "/comment_user.boilerplate"));
}

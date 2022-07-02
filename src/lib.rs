#![deny(missing_docs)]

//! Type definitions for non-fungible token (NFT) metadata.
//!
//! While [EIP-721](https://eips.ethereum.org/EIPS/eip-721#specification) defines a "ERC-721 Metadata JSON Schema", in practice
//! it is rarely used. Instead, this crate implements the more popular [OpenSea metadata standard](https://docs.opensea.io/docs/metadata-standards).
//!
//! This crate does not attempt to perform validation more than what is strictly necessary. Since every secondary
//! market will use the fields in the metadata in a different way, it is up to the crate consumer to make sure the fields are appropriately populated.

use rgb::RGB8;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use url::Url;

/// Metadata for a token.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub struct Metadata {
    /// URL to image of the item.
    pub image: Url,
    /// External URL to another site.
    pub external_url: Url,
    /// Human-readable description of the item.
    pub description: String,
    /// Name of the item.
    pub name: String,
    /// Attributes for the item.
    pub attributes: Vec<AttributeEntry>,
    /// Background color of the item.
    #[cfg_attr(feature = "serde", serde(with = "rgb8_fromstr_radix_16"))]
    pub background_color: RGB8,
    /// URL to multi-media attachment for the item.
    pub animation_url: Url,
    /// URL to a YouTube video.
    pub youtube_url: Url,
}

/// A key-value pair of attributes for an item.
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case"),
    serde(untagged)
)]
#[derive(Debug, Clone)]
pub enum AttributeEntry {
    /// Textual attribute.
    String {
        /// Name of the trait.
        trait_type: String,
        /// Value of the attribute.
        value: String,
    },
    /// Numerical attribute.
    Number {
        /// Name of the trait.
        trait_type: String,
        /// Value of the attribute.
        value: u64,
        /// How the attribute should be displayed.
        display_type: Option<DisplayType>,
    },
}

/// How a numerical attribute should be displayed.
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case")
)]
#[derive(Copy, Clone, Debug)]
pub enum DisplayType {
    /// As a number.
    Number,
    /// As a boost percentage.
    BoostPercentage,
    /// As a boost number.
    BoostNumber,
    /// As a date.
    Date,
}

#[cfg(feature = "serde")]
mod rgb8_fromstr_radix_16 {
    use rgb::{ComponentSlice, RGB8};
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    #[derive(Serialize, Deserialize)]
    #[serde(transparent)]
    struct Helper(#[serde(with = "hex")] Vec<u8>);

    pub fn deserialize<'de, D>(deserializer: D) -> Result<RGB8, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Helper = Deserialize::deserialize(deserializer)?;
        let s = s.0;
        if s.len() != 3 {
            Err(D::Error::custom("expected color hex string"))
        } else {
            Ok(RGB8 {
                r: s[0],
                g: s[1],
                b: s[2],
            })
        }
    }

    pub fn serialize<S>(value: &RGB8, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Helper(value.as_slice().to_vec()).serialize(serializer)
    }

    #[cfg(test)]
    mod tests {
        use rgb::RGB8;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        struct Target {
            #[serde(with = "crate::rgb8_fromstr_radix_16")]
            color: RGB8,
        }

        #[test]
        fn from_json() {
            let s = r#"{ "color": "f2f2f2" }"#;
            let target: Target = serde_json::from_str(s).unwrap();
            assert_eq!(
                target.color,
                RGB8 {
                    r: 242,
                    g: 242,
                    b: 242
                }
            );
        }

        #[test]
        fn to_json() {
            let target = Target {
                color: RGB8 {
                    r: 242,
                    g: 242,
                    b: 242,
                },
            };
            let s = serde_json::to_string(&target).unwrap();
            assert_eq!(s, r#"{"color":"f2f2f2"}"#)
        }

        #[test]
        fn from_notcolor_json() {
            let s = r#"{ "color": "f2f2f2f2" }"#;
            let target = serde_json::from_str::<Target>(s);
            assert!(target.is_err());
        }
    }
}

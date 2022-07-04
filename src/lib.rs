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
///
/// While even an empty object is "valid" metadata, this crate takes a more opinionated approach.
/// The following fields are strictly required: [`name`](Metadata::name), [`description`](Metadata::description), [`image`](Metadata::image).
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Metadata {
    /// URL to image of the item.
    pub image: Url,
    /// External URL to another site.
    pub external_url: Option<Url>,
    /// Human-readable description of the item.
    pub description: String,
    /// Name of the item.
    pub name: String,
    /// Attributes for the item.
    #[serde(default)]
    pub attributes: Vec<AttributeEntry>,
    /// Background color of the item.
    /// When serialized, it takes the form of a 6-character hexadecimal string without a `#`.
    #[cfg_attr(feature = "serde", serde(with = "rgb8_fromhex_opt", default))]
    pub background_color: Option<RGB8>,
    /// URL to multi-media attachment for the item.
    pub animation_url: Option<Url>,
    /// URL to a YouTube video.
    pub youtube_url: Option<Url>,
}

/// A key-value pair of attributes for an item.
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(rename_all = "snake_case"),
    serde(untagged)
)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
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
mod rgb8_fromhex_opt {
    use rgb::{ComponentSlice, RGB8};
    use serde::{de::Error, Deserializer, Serialize, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<RGB8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Vec<u8> = hex::deserialize(deserializer)?;
        if s.len() != 3 {
            Err(D::Error::custom("expected color hex string"))
        } else {
            Ok(Some(RGB8 {
                r: s[0],
                g: s[1],
                b: s[2],
            }))
        }
    }

    pub fn serialize<S>(value: &Option<RGB8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => hex::serialize(value.as_slice().to_vec(), serializer),
            None => None::<()>.serialize(serializer),
        }
    }

    #[cfg(test)]
    mod tests {
        use rgb::RGB8;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        struct Target {
            #[serde(with = "crate::rgb8_fromhex_opt")]
            color: Option<RGB8>,
        }

        #[test]
        fn from_json() {
            let s = r#"{ "color": "f2f2f2" }"#;
            let target: Target = serde_json::from_str(s).unwrap();
            assert_eq!(
                target.color,
                Some(RGB8 {
                    r: 242,
                    g: 242,
                    b: 242
                })
            );
        }

        #[test]
        fn to_json() {
            let target = Target {
                color: Some(RGB8 {
                    r: 242,
                    g: 242,
                    b: 242,
                }),
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

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use crate::Metadata;

    const PLANETPASS_ITEM: &str = r#"
    {
        "image": "https://assets.wanderers.ai/file/planetpass/vid/0/0.mp4",
        "description": "Visit this planet and get a free Rocketeer NFT from Alucard.eth!",
        "name": "Rocketeer X",
        "attributes": [
          {
            "trait_type": "Core",
            "value": "Vortex"
          },
          {
            "trait_type": "Satellite",
            "value": "Protoplanets"
          },
          {
            "trait_type": "Feature",
            "value": "Icy"
          },
          {
            "trait_type": "Ship",
            "value": "Docking"
          },
          {
            "trait_type": "Space",
            "value": "Green Sun"
          },
          {
            "trait_type": "Terrain",
            "value": "Layers"
          },
          {
            "trait_type": "Faction",
            "value": "Coalition for Uncorrupted Biology"
          },
          {
            "trait_type": "Atmosphere",
            "value": "Alpen Glow"
          }
        ]
      }
    "#;

    #[test]
    pub fn planetpass() {
        let metadata = serde_json::from_str::<Metadata>(PLANETPASS_ITEM);
        assert!(metadata.is_ok());
    }
}

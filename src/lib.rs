#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![doc = include_str!("../README.md")]

pub mod flows;
mod secret;
#[cfg(feature = "serde")]
pub mod serde;

#[doc(inline)]
pub use self::secret::Secret;

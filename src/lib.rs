#![doc(html_root_url = "https://docs.rs/geohash-tools/")]

//! # geohash-tools
//!
//! geohash-tools is a collection of functions for manipulating GeoHashes in Rust.<br>
//! You can find more about geohash algorithm on [Wikipedia](https://en.wikipedia.org/wiki/Geohash).
//!

mod geohash;
mod conversion;

pub use geohash::RUNE_CHAR_INDEX;
pub use geohash::RUNE_INDEX_CHAR;
pub use geohash::RUNE_LEN;
pub use geohash::RUNE_INDEX_LEN;

pub use geohash::GEOHASH_ID_RANGE;

pub use conversion::geohash_to_int;
pub use conversion::int_to_geohash;
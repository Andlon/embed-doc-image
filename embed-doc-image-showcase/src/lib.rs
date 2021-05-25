//! Showcase for `embed-doc-image`.
//!
#![cfg_attr(feature = "doc-images",
cfg_attr(all(),
doc = ::embed_doc_image::embed_image!("ferris", "images/rustacean-orig-noshadow-tiny.png"),
doc = ::embed_doc_image::embed_image!("ferris2", "images/rustacean-flat-gesture-tiny.png")))]
#![cfg_attr(
    not(feature = "doc-images"),
    doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
           to enable."
)]
//!
//! This crate contains no functionality, it is merely a demonstration of how to use
//! `embed-doc-image` to embed images local to the repository that work across both <docs.rs> and
//! local documentation. The motivation for this crate is
//! [rustdoc's inability to include local images](https://github.com/rust-lang/rust/issues/32104)
//! in a way that consistently works across local copies of the repository and `docs.rs`.
//!
//! ![Original Ferris][ferris]
//!
//! ![Ferris making gesture][ferris2]
//!
//! The above picture is included as
//!
//! TODO: Attribute ferris images to its website
//!
//!
//! TODO: support and test typical image file formats (png/jpg/svg/bmp/gif)
//!
use embed_doc_image::embed_doc_image;

/// Returns a ferris.
///
/// ![Original Ferris][ferris]
///
/// But what about gestures?
///
/// ![Ferris makes gesture][gesture]
///
/// Some more docs.
#[embed_doc_image("ferris", "images/rustacean-orig-noshadow-tiny.png")]
#[embed_doc_image("gesture", "images/rustacean-flat-gesture-tiny.png")]
pub fn ferris() {}

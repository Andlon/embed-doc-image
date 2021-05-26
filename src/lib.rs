//! Embed images in documentation.
//!
//! This crate enables the portable embedding of images in
//! `rustdoc`-generated documentation. Standard
//! web-compatible image formats should be supported. Please [file an issue][issue-tracker]
//! if you have problems. Read on to learn how it works.
//!
//! # Showcase
//!
//! See the [showcase documentation][showcase-docs] for an example with embedded images.
//!
//! Please also check out the [source code][showcase-source] for [the showcase crate][showcase]
//! for a fleshed out example.
//!
//! # Motivation
//!
//! A picture is worth a thousand words. This oft quoted adage is no less true for technical
//! documentation. A carefully crafted diagram lets a new user immediately
//! grasp the high-level architecture of a complex library. Illustrations of geometric conventions
//! can vastly reduce confusion among users of scientific libraries. Despite the central role
//! of images in technical documentation, embedding images in Rust documentation in a way that
//! portably works correctly across local installations and [docs.rs](https://docs.rs) has been a
//! [longstanding issue of rustdoc][rustdoc-issue].
//!
//! This crate represents a carefully crafted solution based on procedural macros that works
//! around the current limitations of `rustdoc` and enables a practically workable approach to
//! embedding images in a portable manner.
//!
//! # How to embed images in documentation
//!
//! First, you'll need to depend on this crate. In `cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! // Replace x.x with the latest version
//! embed-doc-image = "x.x"
//! ```
//!
//! What the next step is depends on whether you want to embed images into *inner attribute
//! documentation* or *outer attribute documentation*. Inner attribute documentation is usually
//! used to document crate-level or module-level documentation, and typically starts each line with
//! `//!`. Outer attribute docs are used for most other forms of documentation, such as function
//! and struct documentation. Outer attribute documentation typically starts each line with `///`.
//!
//! In both cases all image paths are relative to the **crate root**.
//!
//! ## Embedding images in outer attribute documentation
//!
//! Outer attribute documentation is typically used for documenting functions, structs, traits,
//! macros and so on. Let's consider documenting a function and embedding an image into its
//! documentation:
//!
//! ```rust
//! // Import the attribute macro
//! use embed_doc_image::embed_doc_image;
//!
//! /// Foos the bar.
//! ///
//! /// Let's drop an image below this text.
//! ///
//! /// ![Alt text goes here][myimagelabel]
//! ///
//! /// And another one.
//! ///
//! /// ![A Foobaring][foobaring]
//! ///
//! /// We can include any number of images in the above fashion. The important part is that
//! /// you match the label ("myimagelabel" or "foobaring" in this case) with the label in the
//! /// below attribute macro.
//! // Paths are always relative to the **crate root**
//! #[embed_doc_image("myimagelabel", "images/foo.png")]
//! #[embed_doc_image("foobaring", "assets/foobaring.jpg")]
//! fn foobar() {}
//! ```
//!
//! And that's it! If you run `cargo doc`, you should hopefully be able to see your images
//! in the documentation for `foobar`, and it should also work on `docs.rs` without trouble.
//!
//! ## Embedding images in inner attribute documentation
//!
//! The ability for macros to do *anything* with *inner attributes* is very limited. In fact,
//! before Rust 1.54 (which at the time of writing has not yet been released),
//! it is for all intents and purposes non-existent. This also means that we can not directly
//! use our approach to embed images in documentation for Rust < 1.54. However, we can make our
//! code compile with Rust < 1.54 and instead inject a prominent message that some images are
//! missing.
//! `docs.rs`, which always uses a nightly compiler, will be able to show the images. We'll
//! also locally be able to properly embed the images as long as we're using Rust >= 1.54
//! (or nightly). Here's how you can embed images in crate-level or module-level documentation:
//!
//! ```rust
//! //! My awesome crate for fast foobaring in latent space.
//! //!
//! // Important: note the blank line of documentation on each side of the image lookup table.
//! // The "image lookup table" can be placed anywhere, but we place it here together with the
//! // warning if the `doc-images` feature is not enabled.
//! #![cfg_attr(feature = "doc-images",
//! cfg_attr(all(),
//! doc = ::embed_doc_image::embed_image!("myimagelabel", "images/foo.png"),
//! doc = ::embed_doc_image::embed_image!("foobaring", "assets/foobaring.png")))]
//! #![cfg_attr(
//! not(feature = "doc-images"),
//! doc = "**Doc images not enabled**. Compile with feature `doc-images` and Rust version >= 1.54 \
//!            to enable."
//! )]
//! //!
//! //! Let's use our images:
//! //! ![Alt text goes here][myimagelabel] ![A Foobaring][foobaring]
//! ```
//!
//! Sadly there is currently no way to detect Rust versions in `cfg_attr`. Therefore we must
//! rely on a feature flag for toggling proper image embedding. We'll need the following in our
//! `Cargo.toml`:
//!
//! ```toml
//! [features]
//! doc-images = []
//!
//! [package.metadata.docs.rs]
//! # docs.rs uses a nightly compiler, so by instructing it to use our `doc-images` feature we
//! # ensure that it will render any images that we may have in inner attribute documentation.
//! features = ["doc-images"]
//! ```
//!
//! Let's summarize:
//!
//! - `docs.rs` will correctly render our documentation with images.
//! - Locally:
//!   - for Rust >= 1.54 with `--features doc-images`, the local documentation will
//!       correctly render images.
//!   - for Rust < 1.54: the local documentation will be missing some images, and will
//!   contain a warning with instructions on how to enable proper image embedding.
//!   - we can also use e.g. `cargo +nightly doc --features doc-images` to produce correct
//!     documentation with a nightly compiler.
//!
//!
//! # How it works
//!
//! The crux of the issue is that `rustdoc` does not have a mechanism for tracking locally stored
//! images referenced by documentation and carry them over to the final documentation. Therefore
//! currently images on `docs.rs` can only be included if you host the image somewhere on the
//! internet and include the image with its URL. However, this has a number of issues:
//!
//! - You need to host the image, which incurs considerable additional effort on the part of
//!   crate authors.
//! - The image is only available for as long as the image is hosted.
//! - Images in local documentation will not work without internet access.
//! - Images are not *versioned*, unless carefully done so manually by the crate author. That is,
//!   the author must carefully provide *all* versions of the image across all versions of the
//!   crate with a consistent naming convention in order to ensure that documentation of
//!   older versions of the crate display the image consistent with that particular version.
//!
//! The solution employed by this crate is based on a remark made in an old
//! [reddit comment from 2017][reddit-comment]. In short, Rustdoc allows images to be provided
//! inline in the Markdown as `base64` encoded binary blobs in the following way:
//!
//! ```rust
//! ![Alt text][myimagelabel]
//!
//! [myimagelabel]: data:image/png;base64,BaSe64EnCoDeDdAtA
//! ```
//!
//! Basically we can use the "reference" feature of Markdown links/images to provide the URL
//! of the image in a different location than the image itself, but instead of providing an URL
//! we can directly provide the binary data of the image in the Markdown documentation.
//!
//! However, doing this manually with images would terribly clutter the documentation, which
//! seems less than ideal. Instead, we do this programmatically. The macros available in this
//! crate essentially follow this idea:
//!
//! - Take a label and image path relative to the crate root as input.
//! - Determine the MIME type (based on extension) and `base64` encoding of the image.
//! - Produce an appropriate doc string and inject it into the Markdown documentation for the
//!   crate/function/struct/etc.
//!
//! Clearly, this is still quite hacky, but it seems like a workable solution until proper support
//! in `rustdoc` arrives, at which point we may rejoice and abandon this crate to the annals
//! of history.
//!
//! # Acknowledgements
//!
//! As an inexperienced proc macro hacker, I would not have managed to arrive at this
//! solution without the help of several individuals on the Rust Programming Language Community
//! Discord server, most notably:
//!
//! - Yandros [(github.com/danielhenrymantilla)](https://github.com/danielhenrymantilla)
//! - Nemo157 [(github.com/Nemo157)](https://github.com/Nemo157)
//!
//! [showcase]: https://crates.io/crates/embed-doc-image-showcase
//! [showcase-docs]: https://docs.rs/embed-doc-image-showcase
//! [showcase-source]: https://github.com/Andlon/embed-doc-image/tree/master/embed-doc-image-showcase
//! [rustdoc-issue]: https://github.com/rust-lang/rust/issues/32104
//! [issue-tracker]: https://github.com/Andlon/embed-doc-image/issues
//! [reddit-comment]: https://www.reddit.com/r/rust/comments/5ljshj/diagrams_in_documentation/dbwg96q?utm_source=share&utm_medium=web2x&context=3
//!
//!

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use std::fs::read;
use std::path::{Path, PathBuf};
use syn::parse;
use syn::parse::{Parse, ParseStream};
use syn::{
    Item, ItemConst, ItemEnum, ItemExternCrate, ItemFn, ItemForeignMod, ItemImpl, ItemMacro,
    ItemMacro2, ItemMod, ItemStatic, ItemStruct, ItemTrait, ItemTraitAlias, ItemType, ItemUnion,
    ItemUse,
};

#[derive(Debug)]
struct ImageDescription {
    label: String,
    path: PathBuf,
}

impl Parse for ImageDescription {
    fn parse(input: ParseStream) -> parse::Result<Self> {
        let label = input.parse::<syn::LitStr>()?;
        input.parse::<syn::Token![,]>()?;
        let path = input.parse::<syn::LitStr>()?;
        Ok(ImageDescription {
            label: label.value(),
            path: PathBuf::from(path.value()),
        })
    }
}

fn encode_base64_image_from_path(path: &Path) -> String {
    let bytes = read(path).unwrap_or_else(|_| panic!("Failed to load image at {}", path.display()));
    base64::encode(bytes)
}

fn determine_mime_type(extension: &str) -> String {
    let extension = extension.to_ascii_lowercase();

    // TODO: Consider using the mime_guess crate? The below list does seem kinda exhaustive for
    // doc purposes though?

    // Matches taken haphazardly from
    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
    match extension.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "bmp" => "image/bmp",
        "svg" => "image/svg+xml",
        "gif" => "image/gif",
        "tif" | "tiff" => "image/tiff",
        "webp" => "image/webp",
        "ico" => "image/vnd.microsoft.icon",
        _ => panic!("Unrecognized image extension, unable to infer correct MIME type"),
    }
    .to_string()
}

fn produce_doc_string_for_image(image_desc: &ImageDescription) -> String {
    let root_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("Failed to retrieve value of CARGO_MANOFEST_DIR.");
    let root_dir = Path::new(&root_dir);
    let encoded = encode_base64_image_from_path(&root_dir.join(&image_desc.path));
    let ext = image_desc.path.extension().unwrap_or_else(|| {
        panic!(
            "No extension for file {}. Unable to determine MIME type.",
            image_desc.path.display()
        )
    });
    let mime = determine_mime_type(&ext.to_string_lossy());
    let doc_string = format!(
        " [{label}]: data:{mime};base64,{encoded}",
        label = &image_desc.label,
        mime = mime,
        encoded = &encoded
    );
    doc_string
}

/// Produces a doc string for inclusion in Markdown documentation.
///
/// Please see the crate-level documentation for usage instructions.
#[proc_macro]
pub fn embed_image(item: TokenStream) -> TokenStream {
    let image_desc = syn::parse_macro_input!(item as ImageDescription);
    let doc_string = produce_doc_string_for_image(&image_desc);

    // Ensure that the "image table" at the end is separated from the rest of the documentation,
    // otherwise the markdown parser will not treat them as a "lookup table" for the image data
    let s = format!("\n \n {}", doc_string);
    let tokens = quote! {
        #s
    };
    tokens.into()
}

/// Produces a doc string for inclusion in Markdown documentation.
///
/// Please see the crate-level documentation for usage instructions.
#[proc_macro_attribute]
pub fn embed_doc_image(attr: TokenStream, item: TokenStream) -> TokenStream {
    let image_desc = syn::parse_macro_input!(attr as ImageDescription);
    let doc_string = produce_doc_string_for_image(&image_desc);

    // Then inject a doc string that "resolves" the image reference and supplies the
    // base64-encoded data inline
    let mut input: syn::Item = syn::parse_macro_input!(item);
    match input {
        Item::Const(ItemConst { ref mut attrs, .. })
        | Item::Enum(ItemEnum { ref mut attrs, .. })
        | Item::ExternCrate(ItemExternCrate { ref mut attrs, .. })
        | Item::Fn(ItemFn { ref mut attrs, .. })
        | Item::ForeignMod(ItemForeignMod { ref mut attrs, .. })
        | Item::Impl(ItemImpl { ref mut attrs, .. })
        | Item::Macro(ItemMacro { ref mut attrs, .. })
        | Item::Macro2(ItemMacro2 { ref mut attrs, .. })
        | Item::Mod(ItemMod { ref mut attrs, .. })
        | Item::Static(ItemStatic { ref mut attrs, .. })
        | Item::Struct(ItemStruct { ref mut attrs, .. })
        | Item::Trait(ItemTrait { ref mut attrs, .. })
        | Item::TraitAlias(ItemTraitAlias { ref mut attrs, .. })
        | Item::Type(ItemType { ref mut attrs, .. })
        | Item::Union(ItemUnion { ref mut attrs, .. })
        | Item::Use(ItemUse { ref mut attrs, .. }) => {
            let str = doc_string;
            // Insert an empty doc line to ensure that we get a blank line between the
            // docs and the "bibliography" containing the actual image data.
            // Otherwise the markdown parser will mess up our output.
            attrs.push(syn::parse_quote! {
                #[doc = ""]
            });
            attrs.push(syn::parse_quote! {
                #[doc = #str]
            });
            input.into_token_stream()
        }
        _ => syn::Error::new_spanned(
            input,
            "Unsupported item. Cannot apply attribute to the given item.",
        )
        .to_compile_error(),
    }
    .into()
}

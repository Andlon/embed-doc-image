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
    let bytes = read(path).expect(&format!("Failed to load image at {}", path.display()));
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
    let ext = image_desc.path.extension().expect(&format!(
        "No extension for file {}. Unable to determine MIME type.",
        image_desc.path.display()
    ));
    let mime = determine_mime_type(&ext.to_string_lossy());
    let doc_string = format!(
        " [{label}]: data:{mime};base64,{encoded}",
        label = &image_desc.label,
        mime = mime,
        encoded = &encoded
    );
    doc_string
}

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

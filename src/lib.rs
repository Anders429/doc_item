//! Attributes for item-level documentation customization.
//!
//! This crate provides attributes for adding various features to items when they are documented by
//! [`rustdoc`](https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html). This includes defining
//! item-info docboxes, annotating an item's minimum version, and marking an item to be displayed as
//! semi-transparent on module lists.
//!
//! This allows for enhanced documentation, similar to what is done in the standard library with the
//! [`staged_api`](https://doc.rust-lang.org/beta/unstable-book/language-features/staged-api.html)
//! feature and what is available on nightly with the
//! [`doc_cfg`](https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html) feature.
//! However, this crate provides even more customization, allowing for use of custom CSS classes and
//! text within docboxes.
//!
//! ## Usage
//!
//! ### Defining an Experimental API
//! Marking an item as experimental (similar to what is done in the standard library through the
//! [`#[unstable]`](https://rustc-dev-guide.rust-lang.org/stability.html#unstable) attribute) can be
//! done as follows:
//!
//! ```
//! /// This is an experimental API.
//! ///
//! /// The docbox will indicate the function is experimental. It will also appear semi-transparent on
//! /// module lists.
//! #[doc_item::docbox(content="<span class='emoji'>🔬</span> This is an experimental API.", class="unstable")]
//! #[doc_item::short_docbox(content="Experimental", class="unstable")]
//! #[doc_item::semi_transparent]
//! pub fn foo() {}
//! ```
//!
//! ### Creating Custom-Styled Docboxes
//! You can create your own custom styles to customize the display of docboxes. Define your item's
//! docbox as follows:
//!
//! ```
//! /// An item with a custom docbox.
//! ///
//! /// The docbox will be a different color.
//! #[doc_item::docbox(content="A custom docbox", class="custom")]
//! #[doc_item::short_docbox(content="Custom", class="custom")]
//! pub fn foo() {}
//! ```
//!
//! Next, create a style definition in a separate HTML file.
//! ```html
//! <style>
//!     .custom {
//!         background: #c4ffd7;
//!         border-color: #7bdba1;
//!     }
//! </style>
//! ```
//!
//! Finally, include the HTML file's contents in your documentation:
//!
//! ```bash
//! $ RUSTDOCFLAGS="--html-in-header custom.html" cargo doc --no-deps --open
//! ```
//!
//! And instruct [docs.rs](https://docs.rs/) to include the HTML file's contents as well by adding to your `Cargo.toml`:
//!
//! ```toml
//! [package.metadata.docs.rs]
//! rustdoc-args = [ "--html-in-header", "custom.html" ]
//! ```

#![warn(
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    unused_qualifications
)]
#![allow(clippy::default_trait_access, clippy::missing_panics_doc)]

extern crate proc_macro;

use darling::FromMeta;
use proc_macro::{token_stream, TokenStream};
use std::str::FromStr;
use syn::{parse_macro_input, AttributeArgs};

#[derive(FromMeta)]
struct BoxArgs {
    #[darling(default)]
    content: String,
    #[darling(default)]
    class: String,
}

#[derive(FromMeta)]
struct SinceArgs {
    #[darling(default)]
    content: String,
}

fn insert_after_attributes(
    result: &mut TokenStream,
    value: TokenStream,
    mut item_iter: token_stream::IntoIter,
) {
    while let Some(token) = item_iter.next() {
        if token.to_string() == "#" {
            Extend::extend::<TokenStream>(result, token.into());
            Extend::extend::<TokenStream>(result, item_iter.next().unwrap().into());
        } else {
            result.extend(value);
            Extend::extend::<TokenStream>(result, token.into());
            Extend::extend::<TokenStream>(result, item_iter.collect());
            return;
        }
    }
    // Catch-all, just in case there are no tokens after the attributes.
    result.extend(value);
}

fn prepend_to_doc(result: &mut TokenStream, value: &str, item_iter: &mut token_stream::IntoIter) {
    while let Some(token) = item_iter.next() {
        if token.to_string() == "#" {
            let attribute = item_iter.next().unwrap().to_string();
            if attribute.starts_with("[doc =") {
                Extend::extend::<TokenStream>(result, token.into());
                let mut old_doc = attribute
                    .splitn(2, '\"')
                    .skip(1)
                    .collect::<String>()
                    .trim_start()
                    .trim_end_matches("\"]")
                    .to_owned();
                if !old_doc.starts_with('<') {
                    old_doc = format!("<p>{}</p>", old_doc);
                }
                Extend::extend::<TokenStream>(
                    result,
                    TokenStream::from_str(&format!("[doc = \"{}{}\"]", value, old_doc)).unwrap(),
                );
                return;
            }
            Extend::extend::<TokenStream>(result, token.into());
            Extend::extend::<TokenStream>(result, TokenStream::from_str(&attribute).unwrap());
        } else {
            // There are no more attributes, and therefore no more docs.
            result.extend(TokenStream::from_str(&format!("#[doc = \"{}\"]", value)).unwrap());
            Extend::extend::<TokenStream>(result, token.into());
            return;
        }
    }
}

/// Adds a docbox to the item's item-info.
///
/// A docbox is defined to be a box below the item's definition within documentation, alerting the
/// user to important information about the item. A common use case is to alert about an
/// experimental item. This can be done as follows:
///
/// ```
/// #[doc_item::docbox(content="This API is experimental", class="unstable")]
/// pub fn foo() {}
/// ```
///
/// # Custom Styles
///
/// The docbox can be styled using the `class` parameter. The class corresponds to a CSS class in
/// the generated HTML. In the above example, `"unstable"` was used, as it is already a predefined
/// class by rustdoc. Other predefined classes include `"portability"` and `"deprecated"`. If
/// different style is desired, a custom class can be provided using the `--html-in-header` rustdoc
/// flag.
///
/// Provide a custom class like this:
///
/// ```
/// #[doc_item::docbox(content="A custom docbox", class="custom")]
/// pub fn foo() {}
/// ```
///
/// Define the custom class in a separate file, potentially named `custom.html`.
///
/// ```html
/// <style>
///     .custom {
///         background: #f5ffd6;
///         border-color: #b9ff00;
///     }
/// </style>
/// ```
///
/// And finally build the documentation with the custom docbox class.
///
/// ```bash
/// $ RUSTDOCFLAGS="--html-in-header custom.html" cargo doc --no-deps --open
/// ```
///
/// # Multiple Docboxes
/// Multiple docbox attributes may be used on a single item. When generating the documentation,
/// `doc_item` will insert the docboxes in the *reverse* order that they are provided in. For
/// example:
///
/// ```
/// #[doc_item::docbox(content="This box will display second", class="unstable")]
/// #[doc_item::docbox(content="This box will display first", class="portability")]
/// pub fn foo() {}
/// ```
///
/// will result in the `"portability"` docbox being displayed above the `"unstable"` docbox.
#[proc_macro_attribute]
pub fn docbox(attr: TokenStream, item: TokenStream) -> TokenStream {
    let box_args = match BoxArgs::from_list(&parse_macro_input!(attr as AttributeArgs)) {
        Ok(args) => args,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let mut result = TokenStream::new();

    // Insert the box after all other attributes.
    insert_after_attributes(
        &mut result,
        TokenStream::from_str(&format!(
            "#[doc = \"\n <div class='item-info'><div class='stab {}'>{}</div></div><script>var box = document.currentScript.previousElementSibling;if(box.parentElement.classList.contains('docblock-short')){{box.remove();}}else if(box.parentElement.parentElement.classList.contains('top-doc')){{box.parentElement.parentElement.before(box);}}else{{box.parentElement.before(box);}}document.currentScript.remove();</script>\"]",
            box_args.class,
            box_args.content
        ))
        .unwrap(),
        item.into_iter()
    );

    result
}

/// Adds a short docbox to the item in module lists.
///
/// A short docbox is defined to be a box immediately before the item's short documentation in
/// module lists, alerting the user to important information about the item. A common use case is to
/// alert about an experimental item. This can be done as follows:
///
/// ```
/// #[doc_item::short_docbox(content="Experimental", class="unstable")]
/// pub fn foo() {}
/// ```
///
/// It is good practice to keep the `content` concise, as short docblocks have limited space. When
/// used with a [`macro@docbox`] attribute, the `short_docbox`'s content should be an abbreviated form of
/// the `docbox`'s content.
///
/// # Custom Styles
///
/// The short docbox can be styled using the `class` parameter. The class corresponds to a CSS class
/// in the generated HTML. In the above example, `"unstable"` was used, as it is already a
/// predefined class by rustdoc. Other predefined classes include `"portability"` and
/// `"deprecated"`. If different style is desired, a custom class can be provided using the
/// `--html-in-header` rustdoc flag.
///
/// Provide a custom class like this:
///
/// ```
/// #[doc_item::short_docbox(content="Custom", class="custom")]
/// pub fn foo() {}
/// ```
///
/// Define the custom class in a separate file, potentially named `custom.html`.
///
/// ```html
/// <style>
///     .custom {
///         background: #f5ffd6;
///         border-color: #b9ff00;
///     }
/// </style>
/// ```
///
/// And finally build the documentation with the custom docbox class.
///
/// ```bash
/// $ RUSTDOCFLAGS="--html-in-header custom.html" cargo doc --no-deps --open
/// ```
///
/// # Multiple Short Docboxes
/// Multiple short docbox attributes may be used on a single item. When generating the
/// documentation, `doc_item` will insert the docboxes in the *reverse* order that they are provided
/// in. For example:
///
/// ```
/// #[doc_item::short_docbox(content="Second", class="unstable")]
/// #[doc_item::short_docbox(content="First", class="portability")]
/// pub fn foo() {}
/// ```
///
/// will result in the `"portability"` short docbox being displayed to the left of the `"unstable"`
/// short docbox.
#[proc_macro_attribute]
pub fn short_docbox(attr: TokenStream, item: TokenStream) -> TokenStream {
    let box_args = match BoxArgs::from_list(&parse_macro_input!(attr as AttributeArgs)) {
        Ok(args) => args,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let mut result = TokenStream::new();
    let mut item_iter = item.clone().into_iter();

    // Insert the short box.
    let short_docbox = &format!(
        "<script>document.currentScript.remove();</script><span class='stab {}'>{}</span><script>var box = document.currentScript.previousElementSibling;var classes = document.currentScript.parentElement.parentElement.getElementsByClassName('module-item');if (classes.length == 0) {{box.remove();}} else {{classes[0].append(box);}}document.currentScript.remove();</script>",
        box_args.class, box_args.content
    );
    prepend_to_doc(&mut result, short_docbox, &mut item_iter);
        
    Extend::extend::<TokenStream>(&mut result, item_iter.collect());

    result
}

/// Makes an item semi-transparent in module lists.
///
/// This is commonly used to denote an item that is unstable and could potentially change in the
/// future, indicating to users that it is not very reliable.
///
/// To make an item semi-transparent, add this attribute before the item as follows:
///
/// ```
/// #[doc_item::semi_transparent]
/// pub fn foo() {}
/// ```
#[proc_macro_attribute]
pub fn semi_transparent(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut result = TokenStream::new();
    let mut item_iter = item.into_iter();

    // Insert script to gray the text.
    prepend_to_doc(
        &mut result,
        "<script>var module_items = document.currentScript.parentElement.parentElement.getElementsByClassName('module-item'); if(module_items.length != 0){{module_items[0].classList.add('unstable');}}document.currentScript.remove();</script>",
        &mut item_iter
    );

    Extend::extend::<TokenStream>(&mut result, item_iter.collect());

    result
}

/// Adds a minimal version to an item.
///
/// This is meant to indicate that an item has been available since a certain version. The value
/// is placed to the right of the item's definition in light text.
///
/// The value is styled the same as the since values used in the standard library's documentation.
///
/// ```
/// #[doc_item::since(content="1.2.0")]
/// pub fn foo() {}
/// ```
#[proc_macro_attribute]
pub fn since(attr: TokenStream, item: TokenStream) -> TokenStream {
    let since_args = match SinceArgs::from_list(&parse_macro_input!(attr as AttributeArgs)) {
        Ok(args) => args,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let mut result = TokenStream::new();

    insert_after_attributes(
        &mut result,
        TokenStream::from_str(&format!(
            "#[doc = \" <script>document.currentScript.remove();</script><span class='since'>{}</span><script>var since=document.currentScript.previousElementSibling;if(since.parentElement.classList.contains('docblock-short')){{since.remove();}}else if(since.parentElement.parentElement.classList.contains('top-doc')){{var out_of_band = since.parentElement.parentElement.parentElement.getElementsByClassName('out-of-band')[0];out_of_band.prepend(' · ');out_of_band.prepend(since);}}else{{var rightside = since.parentElement.parentElement.getElementsByClassName('rightside')[0];rightside.prepend(' · ');rightside.prepend(since);}}document.currentScript.remove();</script>\"]",
            since_args.content
        ))
        .unwrap(),
        item.into_iter()
    );

    result
}

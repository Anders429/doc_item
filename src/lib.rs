extern crate proc_macro;

use darling::FromMeta;
use proc_macro::{token_stream, TokenStream};
use std::str::FromStr;
use syn::{parse_macro_input, AttributeArgs, Lit};
use uuid::Uuid;

#[derive(FromMeta)]
struct BoxArgs {
    #[darling(default)]
    content: String,
    #[darling(default)]
    class: String,
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
            result.extend(TokenStream::from_str(value));
            Extend::extend::<TokenStream>(result, token.into());
        }
    }
}

#[proc_macro_attribute]
pub fn docbox(attr: TokenStream, item: TokenStream) -> TokenStream {
    let box_args = BoxArgs::from_list(&parse_macro_input!(attr as AttributeArgs)).unwrap();

    let mut result = TokenStream::new();

    // Insert the box after all other attributes.
    insert_after_attributes(
        &mut result,
        TokenStream::from_str(&format!(
            "#[doc = \"\n <div class='item-info'><div class='stab {}'>{}</div></div><script>var box = document.currentScript.previousElementSibling;if(box.parentElement.tagName!='TD'){{box.parentElement.before(box);}}else{{box.remove();}}document.currentScript.remove();</script>\"]",
            box_args.class,
            box_args.content
        ))
        .unwrap(),
        item.into_iter()
    );

    result
}

#[proc_macro_attribute]
pub fn short_docbox(attr: TokenStream, item: TokenStream) -> TokenStream {
    let box_args = BoxArgs::from_list(&parse_macro_input!(attr as AttributeArgs)).unwrap();

    let mut result = TokenStream::new();
    let mut item_iter = item.into_iter();

    // Generate a unique id for the span. This allows for easy location and removal in the case of
    // multiple `short_docbox`s being used on one item.
    let id = Uuid::new_v4();

    // Insert the short box.
    prepend_to_doc(
        &mut result,
        &format!(
            "<script>document.currentScript.remove();</script><span class='stab {}' id='{}'>{}</span>",
            box_args.class, id, box_args.content
        ),
        &mut item_iter,
    );

    // Insert short box removal script after all other attributes.
    insert_after_attributes(
        &mut result,
        TokenStream::from_str(&format!(
            "#[doc = \"\n <script>var spans=document.currentScript.parentElement.getElementsByTagName('SPAN');for (var i=0;i<spans.length;i++){{var span=spans.item(i);if (span.id=='{}'){{span.remove();break;}}}}document.currentScript.remove();</script>\"]",
            id
        ))
        .unwrap(),
        item_iter
    );

    result
}

#[proc_macro_attribute]
pub fn semi_transparent(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut result = TokenStream::new();
    let mut item_iter = item.into_iter();

    // Insert script to gray the text.
    prepend_to_doc(
        &mut result,
        "<script>var row=document.currentScript.parentElement.parentElement;if (row.tagName=='TR'){row.classList.add('unstable');}document.currentScript.remove();</script>",
        &mut item_iter
    );

    Extend::extend::<TokenStream>(&mut result, item_iter.collect());

    result
}

#[proc_macro_attribute]
pub fn since(attr: TokenStream, item: TokenStream) -> TokenStream {
    let value = String::from_value(&parse_macro_input!(attr as Lit)).unwrap();

    let mut result = TokenStream::new();

    insert_after_attributes(
        &mut result,
        TokenStream::from_str(&format!(
            "#[doc = \" <script></script><span class='since'>{}</span><script>var since=document.currentScript.previousElementSibling;if (since.parentElement.tagName!='TD'){{var header=since.parentElement.parentElement.firstElementChild;if(header.firstElementChild.tagName=='SPAN'){{header.getElementsByClassName('out-of-band')[0].prepend(since);}}else{{header.lastElementChild.before(since);}}}}else{{since.remove();}}</script>\"]",
            value
        ))
        .unwrap(),
        item.into_iter()
    );

    result
}

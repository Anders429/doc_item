use docmod::*;

#[docbox(content="docbox content", class="docbox")]
#[short_docbox(content="short docbox content", class="short-docbox")]
#[since("1.0.0")]
#[semi_transparent_item]
pub fn function() {}

#[docbox(content="docbox content", class="docbox")]
#[short_docbox(content="short docbox content", class="short-docbox")]
#[since("1.0.0")]
#[semi_transparent_item]
pub struct Struct {}

#[docbox(content="docbox content", class="docbox")]
#[short_docbox(content="short docbox content", class="short-docbox")]
#[since("1.0.0")]
#[semi_transparent_item]
pub enum Enum {}

#[docbox(content="docbox content", class="docbox")]
#[short_docbox(content="short docbox content", class="short-docbox")]
#[since("1.0.0")]
#[semi_transparent_item]
pub const CONST: usize = 0;

#[docbox(content="docbox content", class="docbox")]
#[short_docbox(content="short docbox content", class="short-docbox")]
#[since("1.0.0")]
#[semi_transparent_item]
pub static STATIC: usize = 0;

#[docbox(content="docbox content", class="docbox")]
#[short_docbox(content="short docbox content", class="short-docbox")]
#[since("1.0.0")]
#[semi_transparent_item]
pub union Union {
    a: usize,
}

pub struct Method {}

impl Method {
    #[docbox(content="docbox content", class="docbox")]
    #[since("1.0.0")]
    pub fn method() {}
}

#[docbox(content="docbox content", class="docbox")]
#[short_docbox(content="short docbox content", class="short-docbox")]
#[since("1.0.0")]
#[semi_transparent_item]
pub trait Trait {}

pub struct ImplTrait {}

#[docbox(content="docbox content", class="docbox")]
#[since("1.0.0")]
impl Trait for ImplTrait {}

#[docbox(content="docbox content", class="docbox")]
#[short_docbox(content="short docbox content", class="short-docbox")]
#[since("1.0.0")]
#[semi_transparent_item]
pub mod module {}

#[docbox(content="docbox content", class="docbox")]
#[short_docbox(content="short docbox content", class="short-docbox")]
#[since("1.0.0")]
#[semi_transparent_item]
pub type Type = usize;

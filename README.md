# doc_item

[![MSRV](https://img.shields.io/badge/rustc-1.43.0+-yellow.svg)](#minimum-supported-rust-version)
[![License](https://img.shields.io/crates/l/doc_item)](#license)

Attributes for enhancing documentation.

This crate provides attributes for defining docboxes, making items semi-transparent, and defining
`since` spans in documentation. This allows for enhanced documentation, similar to what is done in
the standard library with the
[`staged_api`](https://doc.rust-lang.org/beta/unstable-book/language-features/staged-api.html)
feature and what is available on nightly with the
[`doc_cfg`](https://doc.rust-lang.org/beta/unstable-book/language-features/doc-cfg.html) feature.
However, this crate provides even more customization, allowing for use of custom CSS classes and
text within docboxes.

## Usage

### Defining an Experimental API
Marking an item as experimental (similar to what is done in the standard library through the
`#[unstable]` attribute) can be done as follows:

```rust
/// This is an experimental API.
///
/// This API is not guaranteed to be stable. It may change at any time.
#[doc_item::docbox(content="<span class='emoji'>🔬</span> This is an experimental API.", class="unstable")]
#[doc_item::short_docbox(content="Experimental", class="unstable")]
#[doc_item::semi_transparent_item]
pub fn foo() {}
```

## Minimum Supported Rust Version
This crate is guaranteed to function properly on `rustc 1.43.0` and up. It may compile on earlier
versions, but it is not guaranteed that all features will display properly.

## Nightly Stability
As [docs.rs](https://docs.rs/) builds documentation on the `nightly` channel, this crate will
attempt to maintain functionality on `nightly`. As this crate's functionality relies on injecting
HTML into the generated documentation, and internal layout of HTML is subject to change, `nightly`
functionality may occasionally break. Please report issues as you find them on the associated github
repository.

## License
This project is licensed under either of

* Apache License, Version 2.0
([LICENSE-APACHE](https://github.com/Anders429/more_ranges/blob/HEAD/LICENSE-APACHE) or
http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
([LICENSE-MIT](https://github.com/Anders429/more_ranges/blob/HEAD/LICENSE-MIT) or
http://opensource.org/licenses/MIT)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

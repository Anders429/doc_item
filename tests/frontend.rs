use doc_item::*;
use std::{env::current_dir, path::Path, process::Command};
#[cfg(test)]
use thirtyfour_sync::prelude::*;

#[docbox(content = "docbox content", class = "docbox")]
#[short_docbox(content = "short docbox content", class = "short-docbox")]
#[since("1.0.0")]
#[semi_transparent]
pub fn function() {}

#[docbox(content = "docbox content", class = "docbox")]
#[short_docbox(content = "short docbox content", class = "short-docbox")]
#[since("1.0.0")]
#[semi_transparent]
pub struct Struct {}

#[docbox(content = "docbox content", class = "docbox")]
#[short_docbox(content = "short docbox content", class = "short-docbox")]
#[since("1.0.0")]
#[semi_transparent]
pub enum Enum {}

#[docbox(content = "docbox content", class = "docbox")]
#[short_docbox(content = "short docbox content", class = "short-docbox")]
#[since("1.0.0")]
#[semi_transparent]
pub const CONST: usize = 0;

#[docbox(content = "docbox content", class = "docbox")]
#[short_docbox(content = "short docbox content", class = "short-docbox")]
#[since("1.0.0")]
#[semi_transparent]
pub static STATIC: usize = 0;

#[docbox(content = "docbox content", class = "docbox")]
#[short_docbox(content = "short docbox content", class = "short-docbox")]
#[since("1.0.0")]
#[semi_transparent]
pub union Union {
    a: usize,
}

pub struct Method {}

impl Method {
    #[docbox(content = "docbox content", class = "docbox")]
    #[since("1.0.0")]
    pub fn method() {}
}

#[docbox(content = "docbox content", class = "docbox")]
#[short_docbox(content = "short docbox content", class = "short-docbox")]
#[since("1.0.0")]
#[semi_transparent]
pub trait Trait {}

pub struct ImplTrait {}

#[docbox(content = "docbox content", class = "docbox")]
#[since("1.0.0")]
impl Trait for ImplTrait {}

#[docbox(content = "docbox content", class = "docbox")]
#[short_docbox(content = "short docbox content", class = "short-docbox")]
#[since("1.0.0")]
#[semi_transparent]
pub mod module {}

#[docbox(content = "docbox content", class = "docbox")]
#[short_docbox(content = "short docbox content", class = "short-docbox")]
#[since("1.0.0")]
#[semi_transparent]
pub type Type = usize;

#[cfg(test)]
fn test_docbox(driver: &WebDriver, prev_element_text: &str) {
    // Check contents.
    let item_info = driver.find_element(By::ClassName("item-info")).expect(&format!("Couldn't find element with previous element text: {}", prev_element_text));
    assert_eq!(item_info.outer_html().expect("Couldn't obtain item-info's outer HTML"), "<div class=\"item-info\"><div class=\"stab docbox\">docbox content</div></div>");
    // Check location.
    let prev_element = item_info.find_element(By::XPath("./preceding-sibling::*[1]")).expect(&format!("Couldn't find previous element with text: {}", prev_element_text));
    assert_eq!(prev_element.text().expect("Couldn't obtain previous element's text"), prev_element_text);
}

#[cfg(test)]
fn test_docbox_html(driver: &WebDriver, prev_element_html: &str) {
    // Check contents.
    let item_info = driver.find_element(By::ClassName("item-info")).expect(&format!("Couldn't find element with previous element text: {}", prev_element_text));
    assert_eq!(item_info.outer_html().expect("Couldn't obtain item-info's outer HTML"), "<div class=\"item-info\"><div class=\"stab docbox\">docbox content</div></div>");
    // Check location.
    let prev_element = item_info.find_element(By::XPath("./preceding-sibling::*[1]")).expect(&format!("Couldn't find previous element with text: {}", prev_element_text));
    assert_eq!(prev_element.outer_html().expect("Couldn't obtain previous element's outer HTML"), prev_element_text);
}

#[cfg(test)]
#[test]
fn doc_ui() {
    // Compile docs.
    Command::new("cargo")
        .arg("doc")
        .arg("--manifest-path")
        .arg("tests/Cargo.toml")
        .arg("--no-deps")
        .spawn()
        .expect("Could not run `cargo doc`")
        .wait()
        .expect("Failed to generate docs");

    // Connect to chromedriver.
    let driver = WebDriver::new("http://localhost:4444", DesiredCapabilities::chrome())
        .expect("Could not connect to chromedriver");

    let base_url = current_dir()
        .unwrap()
        .join(Path::new("tests/target/doc/frontend"));

    // Test individual doc pages.
    driver.get(&format!("file://{}", base_url.join("fn.function.html").to_str().unwrap())).unwrap();
    test_docbox(&driver, "pub fn function()");

    driver.get(&format!("file://{}", base_url.join("struct.Struct.html").to_str().unwrap())).unwrap();
    test_docbox_html(&driver, "<div class=\"docblock type-decl hidden-by-usual-hider\"><pre class=\"rust struct\">pub struct Struct {}</pre></div>");
}

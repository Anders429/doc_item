use std::{env::current_dir, path::Path, process::Command};
use substring::Substring;
use thirtyfour_sync::prelude::*;

fn test_docbox(driver: &WebDriver, prev_element_text: &str) {
    // Check contents.
    let item_info = driver
        .find_element(By::ClassName("item-info"))
        .expect(&format!(
            "Couldn't find element with previous element text: {}",
            prev_element_text
        ));
    assert_eq!(
        item_info
            .outer_html()
            .expect("Couldn't obtain item-info's outer HTML"),
        "<div class=\"item-info\"><div class=\"stab docbox\">docbox content</div></div>"
    );
    // Check location.
    let prev_element = item_info
        .find_element(By::XPath("./preceding-sibling::*[1]"))
        .expect(&format!(
            "Couldn't find previous element with text: {}",
            prev_element_text
        ));
    assert_eq!(
        prev_element
            .text()
            .expect("Couldn't obtain previous element's text"),
        prev_element_text
    );
}

fn test_docbox_in_band(driver: &WebDriver, prev_element_text: &str) {
    // Check contents.
    let item_info = driver
        .find_element(By::ClassName("item-info"))
        .expect(&format!(
            "Couldn't find element with previous element text: {}",
            prev_element_text
        ));
    assert_eq!(
        item_info
            .outer_html()
            .expect("Couldn't obtain item-info's outer HTML"),
        "<div class=\"item-info\"><div class=\"stab docbox\">docbox content</div></div>"
    );
    // Check location.
    let prev_element = item_info
        .find_element(By::XPath("./preceding-sibling::*[1]/*[@class=\"in-band\"]"))
        .expect(&format!(
            "Couldn't find previous element with text: {}",
            prev_element_text
        ));
    assert_eq!(
        prev_element
            .text()
            .expect("Couldn't obtain previous element's text")
            .substring(0, prev_element_text.chars().count()),
        prev_element_text
    );
}

fn test_docbox_html(driver: &WebDriver, prev_element_html: &str) {
    // Check contents.
    let item_info = driver
        .find_element(By::ClassName("item-info"))
        .expect(&format!(
            "Couldn't find element with previous element text: {}",
            prev_element_html
        ));
    assert_eq!(
        item_info
            .outer_html()
            .expect("Couldn't obtain item-info's outer HTML"),
        "<div class=\"item-info\"><div class=\"stab docbox\">docbox content</div></div>"
    );
    // Check location.
    let prev_element = item_info
        .find_element(By::XPath("./preceding-sibling::*[1]"))
        .expect(&format!(
            "Couldn't find previous element with HTML: {}",
            prev_element_html
        ));
    assert_eq!(
        prev_element
            .outer_html()
            .expect("Couldn't obtain previous element's outer HTML"),
        prev_element_html
    );
}

fn test_since_out_of_band(driver: &WebDriver) {
    let out_of_band = driver
        .find_element(By::ClassName("out-of-band"))
        .expect("Couldn't find out-of-band element");
    let first_child_element = out_of_band
        .find_element(By::XPath("./child::*[1]"))
        .expect("Couldn't find first child of out-of-band element");
    assert_eq!(
        first_child_element
            .outer_html()
            .expect("Couldn't find child element's outer HTML"),
        "<span class=\"since\">1.0.0</span>"
    );
}

fn test_since_standalone(driver: &WebDriver, next_element_html: &str) {
    let since = driver
        .find_element(By::ClassName("since"))
        .expect("Couldn't find since element");
    assert_eq!(
        since
            .outer_html()
            .expect("Couldn't get outer HTML of since"),
        "<span class=\"since\">1.0.0</span>"
    );
    let next_element = since
        .find_element(By::XPath("./following-sibling::*[1]"))
        .expect("Couldn't find since's next element");
    assert_eq!(
        next_element
            .outer_html()
            .expect("Couldn't get outer HTML of next element"),
        next_element_html
    );
}

fn test_short_docbox(driver: &WebDriver, link_text: &str) {
    let link = driver
        .find_element(By::LinkText(link_text))
        .expect(&format!("Couldn't find link with text {}", link_text));
    let docblock_short = link
        .find_element(By::XPath("./parent::*[1]/following-sibling::*[1]"))
        .expect("Couldn't find docblock-short");
    assert_eq!(
        docblock_short
            .text()
            .expect("Couldn't get docblock_short's text"),
        "short docbox content"
    );
}

fn test_semi_transparent_item(driver: &WebDriver, link_text: &str) {
    let link = driver
        .find_element(By::LinkText(link_text))
        .expect(&format!("Couldn't find link with text {}", link_text));
    let docblock = link
        .find_element(By::XPath("./parent::*[1]/parent::*[1]"))
        .expect("Couldn't find docblock-short");
    assert_eq!(
        docblock.get_attribute("class").unwrap().unwrap(),
        "module-item unstable"
    );
}

// This test requires an external `chromedriver` to be running on port `4444`.
//
// To run the test, set up a `chromedriver` and then activate the test by setting the `frontend_test` feature.
#[test]
#[cfg_attr(not(feature = "frontend_test"), ignore)]
fn frontend() {
    // Compile docs.
    Command::new("cargo")
        .arg("doc")
        .arg("--manifest-path")
        .arg("tests/test_target/Cargo.toml")
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
        .join(Path::new("tests//test_target/target/doc/test_target"));

    // Test individual doc pages.
    driver
        .get(&format!(
            "file://{}",
            base_url.join("fn.function.html").to_str().unwrap()
        ))
        .unwrap();
    test_docbox(&driver, "pub fn function()");
    test_since_out_of_band(&driver);

    driver
        .get(&format!(
            "file://{}",
            base_url.join("struct.Struct.html").to_str().unwrap()
        ))
        .unwrap();
    test_docbox_html(&driver, "<div class=\"docblock type-decl hidden-by-usual-hider\"><pre class=\"rust struct\">pub struct Struct {}</pre></div>");
    test_since_out_of_band(&driver);

    driver
        .get(&format!(
            "file://{}",
            base_url.join("enum.Enum.html").to_str().unwrap()
        ))
        .unwrap();
    test_docbox(&driver, "pub enum Enum {}");
    test_since_out_of_band(&driver);

    driver
        .get(&format!(
            "file://{}",
            base_url.join("constant.CONST.html").to_str().unwrap()
        ))
        .unwrap();
    test_docbox(&driver, "pub const CONST: usize = 0;");
    test_since_out_of_band(&driver);

    driver
        .get(&format!(
            "file://{}",
            base_url.join("static.STATIC.html").to_str().unwrap()
        ))
        .unwrap();
    test_docbox(&driver, "pub static STATIC: usize");
    test_since_out_of_band(&driver);

    driver
        .get(&format!(
            "file://{}",
            base_url.join("union.Union.html").to_str().unwrap()
        ))
        .unwrap();
    test_docbox_html(&driver, "<div class=\"docblock type-decl hidden-by-usual-hider\"><pre class=\"rust union\">pub union Union {
// some fields omitted
}</pre></div>");
    test_since_out_of_band(&driver);

    driver
        .get(&format!(
            "file://{}",
            base_url.join("struct.Method.html").to_str().unwrap()
        ))
        .unwrap();
    test_docbox(&driver, "pub fn method()\n1.0.0\n[src]\n[−]");
    test_since_standalone(&driver, "<a class=\"srclink\" href=\"../src/test_target/lib.rs.html#48\" title=\"goto source code\">[src]</a>");

    driver
        .get(&format!(
            "file://{}",
            base_url.join("trait.Trait.html").to_str().unwrap()
        ))
        .unwrap();
    test_docbox_html(&driver, "<div class=\"docblock type-decl hidden-by-usual-hider\"><pre class=\"rust trait\">pub trait Trait { }</pre></div>");
    test_since_out_of_band(&driver);

    driver
        .get(&format!(
            "file://{}",
            base_url.join("struct.ImplTrait.html").to_str().unwrap()
        ))
        .unwrap();
    test_docbox(&driver, "impl Trait for ImplTrait\n1.0.0\n[src]");
    test_since_standalone(&driver, "<a class=\"srclink\" href=\"../src/test_target/lib.rs.html#61\" title=\"goto source code\">[src]</a>");

    driver
        .get(&format!(
            "file://{}",
            base_url.join("module/index.html").to_str().unwrap()
        ))
        .unwrap();
    test_docbox_in_band(&driver, "Module test_target::module");
    test_since_out_of_band(&driver);

    driver
        .get(&format!(
            "file://{}",
            base_url.join("type.Type.html").to_str().unwrap()
        ))
        .unwrap();
    test_docbox(&driver, "type Type = usize;");
    test_since_out_of_band(&driver);

    // Test main doc page.
    driver.get(&format!(
        "file://{}",
        base_url.join("index.html").to_str().unwrap()
    )).unwrap();
    test_short_docbox(&driver, "function");
    test_short_docbox(&driver, "Struct");
    test_short_docbox(&driver, "Enum");
    test_short_docbox(&driver, "CONST");
    test_short_docbox(&driver, "STATIC");
    test_short_docbox(&driver, "Union");
    test_short_docbox(&driver, "Trait");
    test_short_docbox(&driver, "module");
    test_short_docbox(&driver, "Type");

    test_semi_transparent_item(&driver, "function");
    test_semi_transparent_item(&driver, "Struct");
    test_semi_transparent_item(&driver, "Enum");
    test_semi_transparent_item(&driver, "CONST");
    test_semi_transparent_item(&driver, "STATIC");
    test_semi_transparent_item(&driver, "Union");
    test_semi_transparent_item(&driver, "Trait");
    test_semi_transparent_item(&driver, "module");
    test_semi_transparent_item(&driver, "Type");
}

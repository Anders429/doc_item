use fantoccini::{Client, ClientBuilder, Locator};
use std::{env::current_dir, path::Path};
use substring::Substring;

async fn test_docbox(client: &mut Client, prev_element_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Check contents.
    let mut item_info = client.find(Locator::Css(".item-info")).await?;
    assert_eq!(item_info.html(false).await.unwrap(), "<div class=\"item-info\"><div class=\"stab docbox\">docbox content</div></div>");
    // Check location.
    let mut prev_element = item_info.find(Locator::XPath("./preceding-sibling::*[1]")).await?;
    assert_eq!(prev_element.text().await.unwrap(), prev_element_text);
    
    Ok(())
}

async fn test_docbox_in_band(client: &mut Client, prev_element_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Check contents.
    let mut item_info = client.find(Locator::Css(".item-info")).await?;
    assert_eq!(item_info.html(false).await.unwrap(), "<div class=\"item-info\"><div class=\"stab docbox\">docbox content</div></div>");
    // Check location.
    let mut prev_element = item_info.find(Locator::XPath("./preceding-sibling::*[1]/*[@class=\"in-band\"]")).await?;
    assert_eq!(prev_element.text().await.unwrap().substring(0, prev_element_text.chars().count()), prev_element_text);
    
    Ok(())
}

async fn test_docbox_html(client: &mut Client, prev_element_html: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Check contents.
    let mut item_info = client.find(Locator::Css(".item-info")).await?;
    assert_eq!(item_info.html(false).await.unwrap(), "<div class=\"item-info\"><div class=\"stab docbox\">docbox content</div></div>");
    // Check location.
    let mut prev_element = item_info.find(Locator::XPath("./preceding-sibling::*[1]")).await?;
    assert_eq!(prev_element.html(false).await.unwrap(), prev_element_html);
    
    Ok(())
}

async fn test_since_out_of_band(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
    let mut out_of_band = client.find(Locator::Css(".out-of-band")).await?;
    let mut first_child_element = out_of_band.find(Locator::XPath("./child::*[1]")).await?;
    assert_eq!(first_child_element.html(false).await.unwrap(), "<span class=\"since\">1.0.0</span>");
    
    Ok(())
}

async fn test_since_standalone(client: &mut Client, next_element_html: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut since = client.find(Locator::Css(".since")).await?;
    assert_eq!(since.html(false).await.unwrap(), "<span class=\"since\">1.0.0</span>");
    let mut next_element = since.find(Locator::XPath("./following-sibling::*[1]")).await?;
    assert_eq!(next_element.html(false).await.unwrap(), next_element_html);
    
    Ok(())
}

async fn test_short_docbox(client: &mut Client, link_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut link = client.find(Locator::LinkText(link_text)).await?;
    let mut docblock_short = link.find(Locator::XPath("./parent::*[1]/following-sibling::*[1]")).await?;
    assert_eq!(docblock_short.text().await.unwrap(), "short docbox content");
    
    Ok(())
}

async fn test_semi_transparent_item(client: &mut Client, link_text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut link = client.find(Locator::LinkText(link_text)).await?;
    let mut docblock = link.find(Locator::XPath("./parent::*[1]/parent::*[1]")).await?;
    assert_eq!(docblock.attr("class").await.unwrap().unwrap(), "module-item unstable");
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to webdriver instance that is listening on port 4444
    let mut client = ClientBuilder::native()
        .connect("http://localhost:4444")
        .await?;
        
    let base_url = current_dir().unwrap().join(Path::new("../test_target/target/doc/test_target"));
    
    // Test individual doc pages.
    client.goto(&format!("file://{}", base_url.join("fn.function.html").to_str().unwrap())).await?;
    test_docbox(&mut client, "pub fn function()").await?;
    test_since_out_of_band(&mut client).await?;
    
    client.goto(&format!("file://{}", base_url.join("struct.Struct.html").to_str().unwrap())).await?;
    test_docbox_html(&mut client, "<div class=\"docblock type-decl hidden-by-usual-hider\"><pre class=\"rust struct\">pub struct Struct {}</pre></div>").await?;
    test_since_out_of_band(&mut client).await?;
    
    client.goto(&format!("file://{}", base_url.join("enum.Enum.html").to_str().unwrap())).await?;
    test_docbox(&mut client, "pub enum Enum {}").await?;
    test_since_out_of_band(&mut client).await?;
    
    client.goto(&format!("file://{}", base_url.join("constant.CONST.html").to_str().unwrap())).await?;
    test_docbox(&mut client, "pub const CONST: usize = 0;").await?;
    test_since_out_of_band(&mut client).await?;
    
    client.goto(&format!("file://{}", base_url.join("static.STATIC.html").to_str().unwrap())).await?;
    test_docbox(&mut client, "pub static STATIC: usize").await?;
    test_since_out_of_band(&mut client).await?;
    
    client.goto(&format!("file://{}", base_url.join("union.Union.html").to_str().unwrap())).await?;
    test_docbox_html(&mut client, "<div class=\"docblock type-decl hidden-by-usual-hider\"><pre class=\"rust union\">pub union Union {
    // some fields omitted
}</pre></div>").await?;
    test_since_out_of_band(&mut client).await?;

    client.goto(&format!("file://{}", base_url.join("struct.Method.html").to_str().unwrap())).await?;
    test_docbox(&mut client, "pub fn method()\n1.0.0\n[src]\n[−]").await?;
    test_since_standalone(&mut client, "<a class=\"srclink\" href=\"../src/test_target/lib.rs.html#46\" title=\"goto source code\">[src]</a>").await?;
    
    client.goto(&format!("file://{}", base_url.join("trait.Trait.html").to_str().unwrap())).await?;
    test_docbox_html(&mut client, "<div class=\"docblock type-decl hidden-by-usual-hider\"><pre class=\"rust trait\">pub trait Trait { }</pre></div>").await?;
    test_since_out_of_band(&mut client).await?;
    
    client.goto(&format!("file://{}", base_url.join("struct.ImplTrait.html").to_str().unwrap())).await?;
    test_docbox(&mut client, "impl Trait for ImplTrait\n1.0.0\n[src]").await?;
    test_since_standalone(&mut client, "<a class=\"srclink\" href=\"../src/test_target/lib.rs.html#59\" title=\"goto source code\">[src]</a>").await?;
    
    client.goto(&format!("file://{}", base_url.join("module/index.html").to_str().unwrap())).await?;
    test_docbox_in_band(&mut client, "Module test_target::module").await?;
    test_since_out_of_band(&mut client).await?;
    
    client.goto(&format!("file://{}", base_url.join("type.Type.html").to_str().unwrap())).await?;
    test_docbox(&mut client, "type Type = usize;").await?;
    test_since_out_of_band(&mut client).await?;
    
    // Test main doc page.
    client.goto(&format!("file://{}", base_url.join("index.html").to_str().unwrap())).await?;
    test_short_docbox(&mut client, "function").await?;
    test_short_docbox(&mut client, "Struct").await?;
    test_short_docbox(&mut client, "Enum").await?;
    test_short_docbox(&mut client, "CONST").await?;
    test_short_docbox(&mut client, "STATIC").await?;
    test_short_docbox(&mut client, "Union").await?;
    test_short_docbox(&mut client, "Trait").await?;
    test_short_docbox(&mut client, "module").await?;
    test_short_docbox(&mut client, "Type").await?;
    
    test_semi_transparent_item(&mut client, "function").await?;
    test_semi_transparent_item(&mut client, "Struct").await?;
    test_semi_transparent_item(&mut client, "Enum").await?;
    test_semi_transparent_item(&mut client, "CONST").await?;
    test_semi_transparent_item(&mut client, "STATIC").await?;
    test_semi_transparent_item(&mut client, "Union").await?;
    test_semi_transparent_item(&mut client, "Trait").await?;
    test_semi_transparent_item(&mut client, "module").await?;
    test_semi_transparent_item(&mut client, "Type").await?;
    
    
    Ok(())
}
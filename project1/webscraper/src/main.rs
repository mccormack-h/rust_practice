use scraper::{Html, Selector};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let body = reqwest::get("https://books.toscrape.com/").await?
    .text().await?;

    let doc = Html::parse_document(&body);

    let selector = Selector::parse("p.price_color").unwrap();

    for element in doc.select(&selector) 
    {
        println!("{}", element.inner_html());
    }

    Ok(())
}

pub fn book_count()
{
    
}

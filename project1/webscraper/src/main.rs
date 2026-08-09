use scraper::{Html, Selector};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
        let body = reqwest::get("https://books.toscrape.com/").await?
    .text().await?;

    let count = book_count(&body);

    println!("{count}");

    let titles = title_fetch(&body);

    println!("{titles}");

    Ok(())
}

pub fn book_count(body: &str) -> usize
{
    let doc = Html::parse_document(&body);
    let book_selector = Selector::parse("article.product_pod").unwrap();

    doc.select(&book_selector).count()
}

pub fn title_fetch(body: &str) -> usize
{
    let doc = Html::parse_document(&body);
    let title_selector = Selector::parse("h3 > a").unwrap();

    for element in doc.select(&title_selector) 
    {
        let book_title = element.value().attr("title").unwrap_or("#");
        println!("{book_title}");
    }

    return 1;
}

use exitfailure::ExitFailure;
use failure::ResultExt;
use scraper::{Html, Selector};
use std::format;
use std::io;
use futures::executor::block_on;

struct Report {
    loc: String,
    temp: String,
    scale: String,
    condition: String,
}

async fn main() -> Result<(), ExitFailure> {
    print_header();

    println!("What zipcode do you want the weather for (97201)?");

    let mut code = String::new();
    io::stdin()
        .read_line(&mut code)
        .expect("Cannot read the input.");

    let process = process_zipcode(&code);
    let weather = block_on(process).and().with_context(|_| format!("An error occured while processing."))?;

    println!("{}", weather);
    Ok(())
}

async fn process_zipcode(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let html = get_html_from_web(&input).await?;
    let report = get_weather_from_html(&html).await?;
    let formatted = format!(
        "The temp in {} is {} {} and {}.",
        report.loc, report.temp, report.scale, report.condition
    );

    Ok(formatted)
}

fn print_header() {
    println!("---------------------------------");
    println!("           WEATHER APP");
    println!("---------------------------------");
}

async fn get_html_from_web(zipcode: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!("http://www.wunderground.com/weather-forecast/{}", zipcode);
    let text = reqwest::get(&url).await?.text().await?;
    Ok(text)
}

async fn get_weather_from_html(html: &str) -> Result<Report, Box<dyn std::error::Error>> {
    let document = Html::parse_document(html);
    let loc_selector =
        Selector::parse(".city-header > h1:nth-child(2) > span:nth-child(1)").unwrap();
    let condition_selector = Selector::parse(".condition-icon > p:nth-child(2)").unwrap();
    let temp_selector = Selector::parse(
        ".current-temp > lib-display-unit:nth-child(1) > span:nth-child(1) > span:nth-child(1)",
    )
    .unwrap();
    let scale_selector = Selector::parse(".current-temp > lib-display-unit:nth-child(1) > span:nth-child(1) > span:nth-child(2) > span:nth-child(1)").unwrap();

    let loc = document.select(&loc_selector).last()?.inner_html();
    let condition = document.select(&condition_selector).last()?.inner_html();
    let temp = document.select(&temp_selector).last()?.inner_html();
    let scale = document.select(&scale_selector).last()?.inner_html();

    // using initializatoin shorthand.
    Ok(Report {
        loc,
        temp,
        scale,
        condition,
    })
}

use std::io;
use std::format;
use reqwest::Error;
use scraper::{Html, Selector};

struct Report {
    loc: String,
    temp: String,
    scale: String,
    condition: String
}

fn main() {
    print_header();

    println!("What zipcode do you want the weather for (97201)?");

    let mut code = String::new();
    io::stdin().read_line(&mut code).expect("Cannot read the input.");
    
    let weather = match process_zipcode(&code) {
        Ok(report) => report,
        _ => String::from("An error occured while processing.")
    };

    println!("{}", weather);
}

fn process_zipcode(input: &str) -> Result<String, Error> {
    let html = get_html_from_web(&input)?;
    let formatted = match  get_weather_from_html(&html) {
        Some(report) => format!("The temp in {} is {} {} and {}.", report.loc, report.temp, report.scale, report.condition),
        None => String::from("An error occured while parsing the weather report.")
    };
    Ok(formatted)
}

fn print_header() {
    println!("---------------------------------");
    println!("           WEATHER APP");
    println!("---------------------------------");
}

fn get_html_from_web(zipcode: &str) -> Result<String, Error> {
    let url = format!("http://www.wunderground.com/weather-forecast/{}", zipcode);
    let mut resp = reqwest::get(&url)?;
    resp.text()
}

fn get_weather_from_html(html: &str) -> Option<Report> {
    let document = Html::parse_document(html);
    let loc_selector = Selector::parse(".city-header > h1:nth-child(2) > span:nth-child(1)").unwrap();
    let condition_selector = Selector::parse(".condition-icon > p:nth-child(2)").unwrap();
    let temp_selector = Selector::parse(".current-temp > lib-display-unit:nth-child(1) > span:nth-child(1) > span:nth-child(1)").unwrap();
    let scale_selector = Selector::parse(".current-temp > lib-display-unit:nth-child(1) > span:nth-child(1) > span:nth-child(2) > span:nth-child(1)").unwrap();

    let loc = document.select(&loc_selector).last()?.inner_html();
    let condition = document.select(&condition_selector).last()?.inner_html();
    let temp = document.select(&temp_selector).last()?.inner_html();
    let scale = document.select(&scale_selector).last()?.inner_html();

    Some(Report { loc: loc, temp: temp, scale: scale, condition: condition})
} 

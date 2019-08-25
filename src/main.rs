use std::io;
use std::format;
use scraper::{Html, Selector};

fn main() {
    print_header();

    println!("What zipcode do you want the weather for (97201)?");

    let mut code = String::new();
    io::stdin().read_line(&mut code).expect("Cannot read the input.");
    
    let html = get_html_from_web(&code);
    let report = get_weather_from_html(&html);

    println!("The temp in {} is {} {} and {}.", report.0, report.1, report.2, report.3)
}

fn print_header() {
    println!("---------------------------------");
    println!("           WEATHER APP");
    println!("---------------------------------");
}

fn get_html_from_web(zipcode: &str) -> String {
    let url = format!("http://www.wunderground.com/weather-forecast/{}", zipcode);
    let mut resp = reqwest::get(&url).unwrap();
    resp.text().unwrap()
}

fn get_weather_from_html(html: &str) -> (String, String, String, String) {
    let document = Html::parse_document(html);
    let loc_selector = Selector::parse(".city-header > h1:nth-child(2) > span:nth-child(1)").unwrap();
    let condition_selector = Selector::parse(".condition-icon > p:nth-child(2)").unwrap();
    let temp_selector = Selector::parse(".current-temp > lib-display-unit:nth-child(1) > span:nth-child(1) > span:nth-child(1)").unwrap();
    let scale_selector = Selector::parse(".current-temp > lib-display-unit:nth-child(1) > span:nth-child(1) > span:nth-child(2) > span:nth-child(1)").unwrap();

    let loc = document.select(&loc_selector).last().unwrap().inner_html();
    let condition = document.select(&condition_selector).last().unwrap().inner_html();
    let temp = document.select(&temp_selector).last().unwrap().inner_html();
    let scale = document.select(&scale_selector).last().unwrap().inner_html();
    
    println!("{}", loc);
    println!("{}", condition);
    println!("{}", temp);
    println!("{}", scale);

    (loc, condition, temp, scale)
} 

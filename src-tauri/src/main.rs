// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use scraper::{Html, Selector};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn greet() {
    // let response = reqwest::blocking::get("https://chainspect.app/dashboard");

    // let body = reqwest::blocking::get("https://chainspect.app/dashboard");

    // let html_content = body.unwrap().text().unwrap();
    // let document = scraper::Html::parse_document(&html_content);
    // let html_product_selector = scraper::Selector::parse("tr").unwrap();
    // let html_products = document.select(&html_product_selector);

    // struct Items {
    //     url: Option<String>,
    // }

    // let mut coins = "";

    // for html_product in html_products {
    //     // scraping logic to retrieve the info
    //     // of interest
    //     let url = html_product
    //         .select(&scraper::Selector::parse("a").unwrap())
    //         .next()
    //         .and_then(|a| a.value().attr("aria-label"))
    //         .map(str::to_owned);

    //     println!("{:#?}", url);

    //     // let img = html_product
    //     //     .select(&scraper::Selector::parse("a svg path").unwrap())
    //     //     .next()
    //     //     .and_then(|a| a.value().attr("fill"))
    //     //     .map(str::to_owned);

    //     // println!("{:#?}", img);

    //     let tps = html_product
    //         .select(&scraper::Selector::parse("td div").unwrap())
    //         .next()
    //         .map(|span| span.text().collect::<String>());

    //     println!("Status: {:#?}", tps);

    //     let table =
    //         html_product.select(&scraper::Selector::parse("table.block").unwrap())
    //         .next()
    //         .map(|thead| thead.text().collect::<String>());

    //     println!("{:#?} SVG", table);

    //     // let items = Items { url };
    //     // coins.push(items);
    // }

    let url = "https://news.ycombinator.com/";
    let html = reqwest::blocking::get(url).unwrap().text().unwrap();
    let fragment = Html::parse_fragment(html.as_str());

    println!("{:#?}", fragment);
    
    let selector_items = Selector::parse(".itemlist tr").unwrap();

    let selector_title = Selector::parse("a.storylink").unwrap();
    let selector_score = Selector::parse("span.score").unwrap();
    let selector_user = Selector::parse("a.hnuser").unwrap();

    let nodes = fragment.select(&selector_items).collect::<Vec<_>>();

    let list = nodes
        .chunks_exact(3)
        .map(|rows| {
            let title_elem = rows[0].select(&selector_title).next().unwrap();
            let title_text = title_elem.text().nth(0).unwrap();
            let title_href = title_elem.value().attr("href").unwrap();

            let score_text = rows[1]
                .select(&selector_score)
                .next()
                .and_then(|n| n.text().nth(0))
                .unwrap_or("0 points");

            let user_text = rows[1]
                .select(&selector_user)
                .next()
                .and_then(|n| n.text().nth(0))
                .unwrap_or("Unknown user");

            [title_text, title_href, score_text, user_text]
        })
        .collect::<Vec<_>>();

    println!("links: {:#?}", list);
    // println!("{:#?}", html_product_selector);

    // println!("{html_content}");
}

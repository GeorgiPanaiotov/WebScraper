use clap::Parser;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::{error::Error, thread::sleep, time::Duration};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,
    #[arg(short, long, default_value_t = 10)]
    delay: u64,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
    #[arg(short, long)]
    selector: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    scrape()
}

fn scrape() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let url = args.url;
    let selector = Selector::parse(&args.selector.clone()).unwrap();

    for _ in 0..args.count {
        let response = get(url.clone())?.text()?;

        let document = Html::parse_document(&response);

        for element in document.select(&selector) {
            println!("{}", element.text().collect::<Vec<_>>().join(" "));
        }

        if args.count > 1 {
            println!("Sleeping for {0} seconds...", args.delay);
            sleep(Duration::from_secs(args.delay));
        }
    }

    Ok(())
}

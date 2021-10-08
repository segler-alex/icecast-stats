use reqwest::blocking::get;
use std::env;
use std::error::Error;
use url::Url;

use icecast_stats::generate_icecast_stats_url;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("url parameter expected");
        std::process::exit(1);
    } else {
        println!("* Parsing arg as url '{}'", args[1]);
        let base_url = Url::parse(&args[1])?;
        println!("* Create icecast status url from '{}'", base_url);
        let url = generate_icecast_stats_url(base_url);
        println!("* Fetching from '{}'", url);
        let resp = get(url.to_string())?;
        println!("* Decode json from response");
        let j: icecast_stats::IcecastStatsRoot = resp.json()?;
        println!("* Result:\n{:#?}", j.icestats);
    }
    Ok(())
}

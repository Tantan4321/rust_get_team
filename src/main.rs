extern crate reqwest;
extern crate scraper;

use scraper::{Html, Selector};

fn main() {
    scrape_team_data("http://www.nfl.com/teams/minnesotavikings/statistics?team=MIN");
}

fn scrape_team_data(url: &str) {
    let mut resp = reqwest::get(url).unwrap();
    assert!(resp.status().is_success());

    //parse response into document
    let doc_body = Html::parse_document(&resp.text().unwrap());

    let record = Selector::parse(".team-overall-ranking").unwrap();

    for record in doc_body.select(&record) {
        let records = record.text().collect::<Vec<_>>();
        println!("The Minnesota Vikings were {} with a record of {}", records[0].trim(), records[1].trim());
    }


}

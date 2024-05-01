use rocket::get;
use rocket::launch;
use rocket::routes;

#[get("/price?<ids>&<vs>")]
async fn price(ids: Option<&str>, vs: Option<&str>) -> String {
    let ids = ids.unwrap();
    let vs = vs.unwrap();
    let url =
        format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}&include_24hr_change=true&include_market_cap=true&include_24hr_vol=true", ids, vs);
    reqwest::get(url).await.unwrap().text().await.unwrap()
}

#[get("/binance?<symbol>")]
async fn binance(symbol: Option<&str>) -> String {
    let url = format!("https://api.binance.com/api/v3/avgPrice?symbol={}", symbol.unwrap());
    reqwest::get(url).await.unwrap().text().await.unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![price, binance])
}

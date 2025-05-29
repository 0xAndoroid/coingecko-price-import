use rocket::get;
use rocket::launch;
use rocket::routes;

#[get("/price?<ids>&<vs>&<include_24hr_change>&<include_market_cap>")]
async fn price(
    ids: Option<&str>,
    vs: Option<&str>,
    include_24hr_change: Option<&str>,
    include_market_cap: Option<&str>,
) -> String {
    let ids = ids.unwrap();
    let vs = vs.unwrap_or("usd");
    let include_24hr_change = include_24hr_change.unwrap_or("false").parse::<bool>().unwrap();
    let include_market_cap = include_market_cap.unwrap_or("false").parse::<bool>().unwrap();
    let url =
        format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}&include_24hr_change={}&include_market_cap={}", ids, vs, include_24hr_change, include_market_cap);
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

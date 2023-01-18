use rocket::launch;
use rocket::get;
use rocket::routes;

#[get("/price?<ids_>&<vs_>")]
async fn price(ids_: Option<&str>, vs_: Option<&str>) -> String {
    let ids = ids_.unwrap();
    let vs = vs_.unwrap();
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies={}", ids, vs);
    reqwest::get(url).await.unwrap().text().await.unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![price])
}

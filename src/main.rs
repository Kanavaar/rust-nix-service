#[tokio::main]
async fn main() {
    let respons = reqwest::get("https://api.thecatapi.com/v1/images/search")
        .await
        .unwrap();
    println!("Status: {}", respons.status());
}

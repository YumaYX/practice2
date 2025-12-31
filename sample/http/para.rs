use reqwest::Client;
use tokio;

async fn fetch(client: &Client, url: &str) -> Result<String, reqwest::Error> {
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let url1 = "http://localhost/";
    let url2 = "http://localhost/";
    let url3 = "http://localhost/";

    let (r1, r2, r3) = tokio::join!(
        fetch(&client, url1),
        fetch(&client, url2),
        fetch(&client, url3),
    );

    println!("site1: {} bytes", r1?.len());
    println!("site2: {} bytes", r2?.len());
    println!("site3: {} bytes", r3?.len());

    Ok(())
}
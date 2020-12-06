#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "https://products.dm.de/store-availability/DE/availability?dans=28171&storeNumbers=2787";
    let res = reqwest::get(url).await?.text().await?;

    println!("{}", res);
    Ok(())
}

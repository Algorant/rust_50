use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
enum MyError {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("HTTP request error")]
    HttpRequest(#[from] reqwest::Error),
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status:{}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    let body = res.text().await?;
    println!("Body:\n{}", body);
    Ok(())
}

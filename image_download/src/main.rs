use anyhow::Result;
use std::fs::File;
use std::io::copy;
use tempfile::Builder;
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
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let response = reqwest::get(target).await?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("File to Download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("File Location: '{}'", fname.display());
        File::create(fname)?
    };

    let content = response.bytes().await?;
    let mut content = content.as_ref();
    copy(&mut content, &mut dest)?;

    Ok(())
}

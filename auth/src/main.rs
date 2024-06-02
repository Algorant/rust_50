use reqwest::{blocking::Client, Error};

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user = "testuser".to_string();
    let passwd: Option<String> = None;

    let response = client
        .get("https://httpbin.org/get")
        .basic_auth(user, passwd)
        .send();

    println!("{:?}", response);

    Ok(())
}


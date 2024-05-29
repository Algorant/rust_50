use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>
}

fn main() {

    let article: Article = Article {
        article: String::from("how to work with json in Rust"),
        author: String::from("Algorant"),
        paragraph: vec![
            Paragraph {
                name: String::from("First sentence")
            },
            Paragraph {
                name: String::from("Body of paragraph")
            },
            Paragraph {
                name: String::from("End of the paragraph")
            }
        ]
    };

    // I modified this to show in pretty format instead of just ugly string.
    let json = serde_json::to_string_pretty(&article).unwrap();
    println!("\nThe JSON is: {}", json)
}




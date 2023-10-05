mod read_config;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // TEST READ TOML FILE
    read_config::Data::read_file("test.toml");
    // TEST CALL TO JSON PLACEHOLDER
    // let posts = reqwest::Client::new()
    // .get("https://jsonplaceholder.typicode.com/posts?userId=1")
    // .send()
    // .await?
    // .text()
    // .await?;

    // println!("{:#?}",posts);

    Ok(())
}

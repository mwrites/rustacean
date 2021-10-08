use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
	.build()?;

    let response = client
	.get("https://httpbin.org/ip")
	.send()
	.await?;

    let json = response
	.json::<HashMap<String, String>>()
	.await?;


    println!("{:?}", json);
    Ok(())
}

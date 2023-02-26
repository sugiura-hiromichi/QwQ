use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error,>,> {
	let resp =
		reqwest::get("https://httpbin.org/ip",).await?.json::<HashMap<String, String,>>().await?;
	println!("{:#?}", resp);

	let client = reqwest::Client::new();
	let _ = client.post("http://httpbin.org/post",).body("this is sent🫠🫠🫠🫠",).send().await?;
	let rsp = reqwest::get("https://twitter.com/home",).await?.text().await?;
	println!("|>	<|\n{rsp}");
	Ok((),)
}

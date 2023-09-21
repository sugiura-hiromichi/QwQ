use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error,>,> {
	let resp = reqwest::get(
"https://www.soudegesu.com/post/rust/rust-with-rustup/",
//		"https://raw.githubusercontent.com/sugiura-hiromichi/.config/master/init.sh",
	)
	.await?
	//.json::<std::collections::HashMap<String, String,>>()
	.text()
	.await?;

	let path = "/tmp/reqwest.txt";
	let mut f = std::fs::File::create(path,)?;
	f.write_all(resp.as_bytes(),)?;

	sugiura_hiromichi_mylibrary::sh_cmd!("open", [path])?;

	//	let client = reqwest::Client::new();
	//	let _ = client.post("http://httpbin.org/post",).body("this is sent🫠🫠🫠🫠",).send().await?;
	//	let rsp = reqwest::get("https://twitter.com/home",).await?.text().await?;
	//	println!("|>	<|\n{rsp}");
	Ok((),)
}

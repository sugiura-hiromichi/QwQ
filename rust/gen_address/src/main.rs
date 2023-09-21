//use mylibrary_::sh_cmd;
use std::fs;

/*
async fn get_request() -> String {
	reqwest::get("https://rakko.tools/tools/131/",)
		.await
		.expect("🫠Unable to get request",)
		.text()
		.await
		.expect("🫠Unable to textize response",)
}

fn create_mail_alts() -> Vec<String,> {
	let f = fs::read_to_string("prof_list.txt",).expect("Unable to read file",);
	let mut buf = vec![];
	f.split_whitespace().for_each(|l| {
		let mut name = format!("NAME: {l}\n\n");
		for c in 'a'..='z' {
			let mut tmp = String::new();
			for i in 0..10 {
				tmp.push_str(&format!("{l}.{i}{c}@kyoto-u.ac.jp\n"),);
			}
			tmp.push('\n',);
			name.push_str(&tmp,);
		}
		buf.push(name,);
	},);
	buf
	//fs::write("email_list.txt", buf,).expect("Unable to write file",);
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error,> {
	let lists = create_mail_alts();
	let rsp = get_request().await;
	fs::write("tmp.html", rsp,).expect("🫠Unable to write file",);
	Ok((),)
}
*/

fn main() {
	let f = fs::read_to_string("prof_list.txt",).expect("🫠Unable to read file",);
	f.split_whitespace().for_each(|l| {
		if l.contains("//",) {
			return;
		}
		let mut tmp = String::new();
		for c in 'a'..='z' {
			for i in 0..10 {
				tmp.push_str(&format!("{l}.{i}{c}@kyoto-u.ac.jp "),);
			}
			tmp.push('\n',);
		}
		fs::write(format!("mail_list/{l}.txt"), tmp,)
			.expect(&format!("🫠Unable to write to `{l}.txt`"),);
	},);
	//	sh_cmd!("python3", ["./vaild_mail.py"]).expect("🫠Failed to run python script",);
}

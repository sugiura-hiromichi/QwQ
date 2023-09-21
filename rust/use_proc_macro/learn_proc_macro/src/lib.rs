//#![crate_type = "proc-macro"]
//extern crate proc_macro;
/// d: these above are not needed since `Cargo.toml` has:
/// ```toml
/// [lib]
/// proc-macro = true
/// ```
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_answer(_item: TokenStream,) -> TokenStream {
	"impl Baz{fn answer() -> u32 { 42 }}".parse().unwrap()
}

#[proc_macro_derive(Answer)]
pub fn derive_answer(_item: TokenStream,) -> TokenStream {
	"fn derived_answer() -> u32 { 43 }".parse().unwrap()
}

#[proc_macro_derive(HelperAttr, attributes(helper))]
pub fn derive_helper(_item: TokenStream,) -> TokenStream { TokenStream::new() }

#[proc_macro_attribute]
pub fn show_streams(attr: TokenStream, item: TokenStream,) -> TokenStream {
	println!("attr: \"{}\"", attr.to_string());
	println!("item: \"{}\"", item.to_string());
	item
}

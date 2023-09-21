use learn_proc_macro::make_answer;
use learn_proc_macro::show_streams;
use learn_proc_macro::Answer;
use learn_proc_macro::HelperAttr;

make_answer!();

#[derive(Answer,)]
struct Baz;

#[derive(HelperAttr, Debug,)]
struct Foo {
	#[helper]
	bar: i32,
}

#[show_streams]
fn invoke1() {}

#[show_streams(bar)]
fn invoke2() {}

#[show_streams(multiple=>tokens)]
fn invoke3() {}

#[show_streams{delimiters}]
fn invoke4() {}

fn main() {
	assert_eq!(Baz::answer(), 42);
	assert_eq!(derived_answer(), 43);
	println!("{:?}", Foo { bar: 0, });
	invoke1();
	invoke2();
	invoke3();
	invoke4();
}

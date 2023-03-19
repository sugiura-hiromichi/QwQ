macro_rules! matcher {
	() => {
		"here is nothing"
	};
	(;) => {
		"here is semicolon"
	};
	(:) => {
		"here is colon"
	};
	(,) => {
		"here is comma"
	};
	(=>) => {
		"here is fat arrow"
	};
	(->) => {
		"here is arrow"
	};
	(+) => {
		"here is plus"
	};
	(-) => {
		"here is minus"
	};
	(*) => {
		"here is star"
	};
	(/) => {
		"here is slash"
	};
	(%) => {
		"here is percent"
	};
	(&) => {
		"here is ampersand"
	};
	(|) => {
		"here is pipe"
	};
	(^) => {
		"here is caret"
	};
	(!) => {
		"here is exclamation"
	};
	(@) => {
		"here is at"
	};
	(#) => {
		"here is hash"
	};
	() => {
		"here is dollar"
	};
	(?) => {
		"here is question"
	};
	(<) => {
		"here is less than"
	};
	(>) => {
		"here is greater than"
	};
	(=) => {
		"here is equal"
	};
}

macro_rules! arrow {
	(->) => {
		"left arrow"
	};
	(<-) => {
		"right arrow"
	};
}

fn main() {
	assert_eq!(matcher!(), "here is nothing");
	assert_eq!(matcher!(@), "here is at");
	assert_eq!(arrow!(->), "left arrow");
	assert_eq!(arrow!(<-), "right arrow");
	println!("🫠");
}

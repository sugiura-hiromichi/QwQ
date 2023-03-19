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

macro_rules! op {
	(add $a:expr, $b:expr) => {
		$a + $b
	};
	(sub $a:expr, $b:expr) => {
		$a - $b
	};
	(mul $a:expr, $b:expr) => {
		$a * $b
	};
	(div $a:expr, $b:expr) => {
		$a / $b
	};
}

fn main() {
	assert_eq!(matcher!(), "here is nothing");
	assert_eq!(matcher!(@), "here is at");
	assert_eq!(arrow!(->), "left arrow");
	assert_eq!(arrow!(<-), "right arrow");
	assert_eq!(op!(add 1, 2), 3);
	assert_eq!(op!(sub 1, 2), -1);
	assert_eq!(op!(mul 1, 2), 2);
	assert_eq!(op!(div 1, 2), 0);
	println!("🫠");
}

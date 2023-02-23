#![allow(unused)]
//!Documentation for crate
use std::any::Any;
use std::any::TypeId;
use std::env::args;

///Confirmation of semantic highlight
enum SemTili {
	NonTuple,
	TupleMem(u8,),
}

///Documentation
fn main() {

	///===============================================================
	//Checking idea that returning private method's pointer enables to access
	// private method Result is bad at rust-nightly 1.64.0
	let has_prv = HasPrivate { pub_member: 0, private_member: 0, };
	assert_eq!(has_prv.pub_f()(), "from pub_f");

	//Sort result between '-' & alphanumerics
	let mut string_vecs: Vec<&str,> =
		vec!["--options", "-h", "--help", "a", "z", "0", "9", "A", "Z"];
	string_vecs.sort();
	assert_eq!(string_vecs, vec!["--help", "--options", "-h", "0", "9", "A", "Z", "a", "z",]);

	///===============================================================
	//Experiment pub(path)'s behavior
	assert_eq!(
		mod1::allowed_view(),
		"calling from mod1::allowed_view()---------mod1::mod2::visible()"
	);

	///===============================================================
	//bool::then method
	assert_eq!(Some(0), (0 == 0).then(|| 0));
	assert_eq!(None, (1 == 0).then(|| 0));

	///===============================================================
	//difference of map() & flat_map()
	let vector = vec![0, 1, 2];
	let from_map: Vec<u8,> = vector.iter().map(|n| n * 2,).collect();
	let vecvec = vec![vector.clone(); 3];
	let from_flat_map: Vec<u8,> = vecvec.iter().flat_map(|i| i.clone(),).collect();
	assert_eq!(from_map, [0, 2, 4]);
	assert_eq!(from_flat_map, [0, 1, 2, 0, 1, 2, 0, 1, 2]);

	///===============================================================
	//let else syntax is available on rust 1.65.0
	|| {
		let some = Some("a",);

		let Some(a)=some else{
      assert_eq!(some,Some("a"));
        return;
    };

		let Some(b): Option<&str>=None else{
      assert_eq!(a,"a");
      return;
    };
	};

	///===============================================================
	//break from labeled blocks is available from rust 1.65.0
	let rslt = 'b: {
		if false {
			break 'b 1;
		}

		if true {
			break 'b 2;
		}
		3
	};
	assert_eq!(rslt, 2);

	///===============================================================
	// `..=X` ranges in patterns enabled from 1.66.0
	match 9 {
		0..=3 => assert!(false),
		_ => assert!(true),
	}

	///===============================================================
	// scientfic notation in rust
	let sn = 1e5 as i32;
	assert!(sn == 100000);

	///===============================================================
	// `from(bool)` for numeric type
	fn usize_from() {
		assert_eq!(usize::from(true), 1);
		assert_eq!(usize::from(false), 0);
	}
	usize_from();

	///===============================================================
	// &str comparison
	assert!("0" < "1");

	///===============================================================
	// unary minus
	let a = 10;
	let b = -a;
	assert!(b == -10);

	///===============================================================
	// unicode sequence
	let dollar = "\u{24}";
	assert_eq!(dollar, "$");

	///===============================================================
	// closure's capture
	fn return_fn(n: i32,) -> impl FnMut() -> i32 {
		let mut rslt = 0;
		move || {
			rslt += n;
			rslt
		}
	}

	let mut inc_by_10 = return_fn(10,);
	inc_by_10();
	inc_by_10();
	inc_by_10();
	assert_eq!(inc_by_10(), 40); // seems static value
	let mut inc_by_6 = return_fn(6,);
	inc_by_6();
	inc_by_6();
	inc_by_6();
	assert_eq!(inc_by_6(), 24);

	///===============================================================
	// closure is reference
	let mut also_inc_10 = &mut inc_by_10; // if `&mut` removed, then `inc_by_10` is moved
	assert_eq!(also_inc_10(), 50);
	assert_eq!(inc_by_10(), 60);

	///===============================================================
	// function in rust is 1st citizen object. that means functions can be stored in variable
	fn fst_citizen() { assert!(true) }
	let mut fn_p = fst_citizen;
	fn fst_citizen2() { assert!(true) }
	//fn_p = fst_citizen2; this cause error because each named function has unique type

	///===============================================================
	// format modifier for `double/float`. This `.3(or any natural number)` specificates numbers of
	// digits after `.`
	assert_eq!(format!("{:.4}", 10.0), "10.0000");

	println!("\n |> 🫠🫠🫠🫠");
} // d: end of main

struct HasPrivate {
	pub pub_member: usize,
	private_member: usize,
}
impl HasPrivate {
	pub fn pub_f(self,) -> impl Fn() -> &'static str {
		//self.prv_fn | this cause error. compiler recognize as field, not method
		|| "from pub_f"
	}

	fn prv_fn(self,) { println!("in plivate function") }
}

pub mod mod1 {
	//!Documentation for module
	pub mod mod2 {
		pub(in crate::mod1) fn visible() -> &'static str { "mod1::mod2::visible()" }

		pub(in crate::mod1::mod2) fn only_in_mod3() {
			println!("mod1::only_in_mod3()");
		}

		pub mod mod3 {
			fn in_mod3() {}
		}
	}

	pub mod mod4 {
		pub mod mod5 {
			fn private_fn() {}
		}
	}

	pub fn allowed_view() -> String {
		"calling from mod1::allowed_view()---------".to_string() + mod2::visible()
	}

	pub struct InMod1 {}
}

pub mod mod6 {
	pub fn in_mod6() {}
}

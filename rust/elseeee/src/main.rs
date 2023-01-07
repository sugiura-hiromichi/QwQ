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
	//Rust can define function in function, return closure intuitive way by using
	// impl trait.
	// TODO

	///=============================================================
	//formatting modifier
	let printout = "printout";
	println!("printout: {printout}");
	println!("printout:?: {printout:?}");

	///This returns closure
	fn return_closure() -> impl Fn() -> i32 {
		fn fn_in_main() -> impl Fn(String,) -> String { |x| x }
		|| fn_in_main()("7".to_string(),).parse::<i32>().unwrap()
	}

	assert_eq!(return_closure()(), 7);

	//How rust identify closure itself and closure's return value in fn's argument
	fn closure_ornot<GenT: 'static,>(_which: GenT,) -> &'static str {
		//typeid of generic parameter's type
		let gen_id = TypeId::of::<GenT,>();
		if TypeId::of::<i32,>() == gen_id {
			"GenT is i32"
		} else if TypeId::of::<dyn Fn() -> i32,>() == gen_id {
			"GenT is Fn()->i32"
		} else {
			"unexpected!!"
		}
	}

	assert_eq!(closure_ornot(return_closure()(),), "GenT is i32");

	//closure's type is different even arguments and return types are same this
	// call's "anonymous type"
	let cl1 = || 1;
	let cl2 = || 2;
	assert!(cl1.type_id() != cl2.type_id());

	//fn's type is same if types of all arguments and return value are same
	fn ret1() -> i32 { 1 }
	fn ret2() -> i32 { 2 }
	fn retn() -> i32 { 1 }
	assert!(ret1.type_id() != ret2.type_id());
	assert!(ret1.type_id() != retn.type_id());

	///=============================================================

	//rust's trait has alot advanced features:D
	trait MyName {
		fn is(&self,) -> &str;
	}

	impl<T,> MyName for Vec<T,> {
		fn is(&self,) -> &str { "!Vec<T>!" }
	}

	impl MyName for i32 {
		fn is(&self,) -> &str { "!int!" }
	}

	impl<T,> MyName for (i32, Vec<T,>,) {
		fn is(&self,) -> &str { "!(i32, Vec<T>)!" }
	}

	let v = vec![0, 1, 1, 2, 23];
	let ai = 0;
	assert_eq!(v.is(), "!Vec<T>!");
	assert_eq!(ai.is(), "!int!");
	assert_eq!((ai, v,).is(), "!(i32, Vec<T>)!");

	//methods (in this case, is()) are only able to use within defined crate.
	// This guarantees safety

	///==============================================================
	//Option<T> supports iterator
	let a = Some("a",);
	for &i in a.iter() {
		assert_eq!(i, "a");
	}

	///===============================================================

	//Rust's pattern matching arm & @ binding
	fn odd(i: i32,) -> bool {
		if i % 2 == 0 {
			true
		} else {
			false
		}
	}

	let v = vec![8, 10, 33, 11, 666, 1];
	v.iter().map(|&n| match n {
		c @ (4..=8 | 33) => assert!(4 <= c && c <= 8 || c == 33),
		c if odd(c,) => assert_eq!(c % 2, 0),
		666 if false => println!("666"),
		c @ (11 | 22) => assert!(c == 11 || c == 22),
		_ => println!("I know that I know nothing"),
	},);

	///===============================================================
	//Option::map uses raw-value if is Some().
	//But how about self is None?
	let mut some_none = None;
	some_none.map(|_one| panic!("This painc shouldn't be executed"),);
	some_none = Some(1,);
	some_none.map(|one| assert_eq!(one, 1),);

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
	let some = Some("a",);

	let Some(a)=some else{
      assert_eq!(some,Some("a"));
        return;
    };

	let Some(b): Option<&str>=None else{
      assert_eq!(a,"a");
      return;
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
}

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

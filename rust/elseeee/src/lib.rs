//!Documentation for crate
#[cfg(test)]
mod tests {
	use std::any::type_name_of_val;

	struct HasPrivate {
		pub pub_member: usize,
		private_member: usize,
	}
	impl HasPrivate {
		pub fn pub_f(self,) -> String {
			//self.prv_fn | this cause error. compiler recognize as field, not
			// method
			self.prv_fn().to_string() + self.private_member.to_string().as_str()
		}

		fn prv_fn(&self,) -> String {
			self.pub_member.to_string() + "in plivate function"
		}
	}

	pub mod mod1 {
		//!Documentation for module
		pub mod mod2 {
			pub(in crate::tests::mod1) fn visible() -> &'static str {
				"mod1::mod2::visible()"
			}
		}

		pub mod mod4 {}

		pub fn allowed_view() -> String {
			"calling from mod1::allowed_view()---------".to_string()
				+ mod2::visible()
		}
	}

	#[test]
	///Checking idea that returning private method's pointer enables to
	/// access private method Result is bad at rust-nightly 1.64.0
	fn ref_to_private_method() {
		let has_prv = HasPrivate { pub_member: 0, private_member: 0, };
		assert_eq!(has_prv.pub_f(), "0in plivate function0");
	}

	#[test]
	///Sort result between '-' & alphanumerics
	fn sort_strings() {
		let mut string_vecs: Vec<&str,> =
			vec!["--options", "-h", "--help", "a", "z", "0", "9", "A", "Z"];
		string_vecs.sort();
		assert_eq!(
			string_vecs,
			vec!["--help", "--options", "-h", "0", "9", "A", "Z", "a", "z",]
		);
	}

	#[test]
	///Experiment pub(path)'s behavior
	fn pub_range() {
		assert_eq!(
			mod1::allowed_view(),
			"calling from mod1::allowed_view()---------mod1::mod2::visible()"
		);
	}

	#[test]
	///bool::then method
	fn bool_then_method() {
		assert_eq!(Some(0), (0 == 0).then(|| 0));
		assert_eq!(None, (1 == 0).then(|| 0));
	}

	#[test]
	///difference of map() & flat_map()
	fn flat_map_and_map() {
		let vector = vec![0, 1, 2];
		let from_map: Vec<u8,> = vector.iter().map(|n| n * 2,).collect();
		let vecvec = vec![vector.clone(); 3];
		let from_flat_map: Vec<u8,> =
			vecvec.iter().flat_map(|i| i.clone(),).collect();
		assert_eq!(from_map, [0, 2, 4]);
		assert_eq!(from_flat_map, [0, 1, 2, 0, 1, 2, 0, 1, 2]);
	}

	#[test]
	///let else syntax is available on rust 1.65.0
	fn let_else() {
		let some = Some("a",);

		let Some(a,) = some else {
			assert_eq!(some, Some("a"));
			return;
		};

		let Some(_b,): Option<&str,> = None else {
			assert_eq!(a, "a");
			return;
		};
	}

	#[test]
	///break from labeled blocks is available from rust 1.65.0
	fn break_from_block() {
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
	}

	#[test]
	/// `..=X` ranges in patterns enabled from 1.66.0
	fn range_exp_in_pattern_matching() {
		match 9 {
			0..=3 => assert!(false),
			_ => assert!(true),
		}
	}

	#[test]
	/// scientific notation in rust
	fn scientific_notation() {
		let sn = 1e5 as i32;
		assert!(sn == 100000);
	}

	#[test]
	/// `from(bool)` for numeric type
	fn from_bool_to_numeric_type() {
		fn usize_from() {
			assert_eq!(usize::from(true), 1);
			assert_eq!(usize::from(false), 0);
		}
		usize_from();
	}

	#[test]
	// &str comparison
	fn str_comparison() {
		assert!("0" < "1");
	}

	#[test]
	/// unicode sequence
	fn unicode_sequence() {
		let dollar = "\u{24}";
		assert_eq!(dollar, "$");
	}

	#[test]
	/// value captured by closure seems to behave as static value
	fn closure_captured_static_value() {
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
	}

	#[test]
	/// when referencing to closure
	fn ref_to_closure() {
		fn return_fn(n: i32,) -> impl FnMut() -> i32 {
			let mut rslt = 0;
			move || {
				rslt += n;
				rslt
			}
		}

		let mut inc_by_10 = return_fn(10,);
		let also_inc_10 = &mut inc_by_10; // if `&mut` removed, then `inc_by_10` is moved
		assert_eq!(also_inc_10(), 10);
		assert_eq!(inc_by_10(), 20);
	}

	#[test]
	/// function in rust is 1st citizen object. that means functions can be
	/// stored in variable
	fn type_of_fn() {
		fn fst_citizen() { assert!(true) }
		let fn_ref = fst_citizen;
		fn fst_citizen2() { assert!(true) }
		//fn_p = fst_citizen2; this cause error because each named function has
		// unique type
		assert_ne!(type_name_of_val(&fn_ref,), type_name_of_val(&fst_citizen2));
		assert_eq!(type_name_of_val(&fn_ref,), type_name_of_val(&fst_citizen))
	}

	#[test]
	fn array_and_slice() {
		let ary = [0; 10];
		let slc = &ary[..];

		assert_ne!(type_name_of_val(&ary,), type_name_of_val(&slc));
	}

	#[test]
	fn positional_parameter_on_formatting() {
		let string = format!("{1} {} {0} {}", "pos_param_0", "pos_param_1");
		assert_eq!(string, "pos_param_1 pos_param_0 pos_param_0 pos_param_1")
	}

	/// When requesting that an argument be formatted with a particular
	/// type, you are actually requesting that an argument ascribes to a
	/// particular trait. This allows multiple actual types to be formatted
	/// via {:x} (like i8 as well as isize). The current mapping of types to
	/// traits is
	mod format_specifier {
		#[test]
		fn width() {
			let x = "x";
			let output = x.to_string() + "    !";
			assert_eq!(output, format!("{x:5}!"));
			assert_eq!(output, format!("{x:0$}!", 5));
			assert_eq!(output, format!("{x:width$}!", width = 5));
			assert_eq!(output, format!("{1:0$}!", 5, x));
			assert_eq!(output, format!("{0:1$}!", x, 5));
			assert_eq!(output, format!("{x:0$}!", 5));
		}

		#[test]
		fn fill_alignment() {
			assert_eq!(format!("{:<5}!", "x"), "x    !"); //left align
			assert_eq!(format!("{:-<5}!", "x"), "x----!"); //left align filled
			assert_eq!(format!("{:^5}!", "x"), "  x  !"); //center align
			assert_eq!(format!("{:>5}!", "x"), "    x!"); //right align
			assert_eq!(format!("{:->5}!", "x"), "----x!"); //right align filled
		}

		#[test]
		fn sign() {
			let s = "goodbye void";

			let s_display = "goodbye void";
			assert_eq!(format!("{}", s), s_display);
			assert_eq!(format!("{:}", s), s_display);
			assert_eq!(format!("{:#}", s), s_display);

			let s_debug = "\"goodbye void\"";
			assert_eq!(format!("{:?}", s), s_debug);

			let _s_pointer = format!("{s:p}");

			let v = vec![10, 11];
			let v_debug_lower_hex = "[a, b]";
			assert_eq!(format!("{v:x?}"), v_debug_lower_hex);

			let v_debug_upper_hex = "[0xA, 0xB]";
			assert_eq!(format!("{v:x?}"), v_debug_upper_hex);

			let v_debug_pretty_print = "[\n    10,\n    11\n]";
			assert_eq!(format!("{v:#?}"), v_debug_pretty_print);

			let i = 27;
			let i_octal = "0o33";
			assert_eq!(format!("{i:o}"), i_octal);

			let i_lower_hex = "0x1b";
			assert_eq!(format!("{i:x}"), i_lower_hex);

			let i_upper_hex = "0x1B";
			assert_eq!(format!("{i:X}"), i_upper_hex);

			let i_binary = "0b11011";
			assert_eq!(format!("{i:b}"), i_binary);

			let i_lower_exp = "2.7e1";
			assert_eq!(format!("{i:e}"), i_lower_exp);

			let i_upper_exp = "2.7E1";
			assert_eq!(format!("{i:E}"), i_upper_exp);
		}
	}
}
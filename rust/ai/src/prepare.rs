#![allow(dead_code)]

// TODO: replace RefCell with Rc

//struct Var<'a,> {
//	pub var:       f64,
//	pub creator: Option<Box<Function<'a,>,>,>,
//}
//impl<'a,> Var<'a,> {
//	pub fn new(var: f64,) -> Var<'a,> { Var { a, creator: None, } }
//}
//
//use std::cell::Ref;
//use std::cell::RefCell;
//type PVar<'a,> = RefCell<Var<'a,>,>;
//type SharedVar<'a,> = Ref<'a, Var<'a,>,>;
//
//trait PVarTrait<'a,> {
//	fn with(var: f64,) -> PVar<'a,>;
//}
//impl<'a,> PVarTrait<'a,> for PVar<'a,> {
//	fn with(var: f64,) -> PVar<'a,> { RefCell::new(Var::new(a,),) }
//}
//
//struct Function<'a,> {
//	pub vars: Vec<SharedVar<'a,>,>,
//}
//impl<'a,> Function<'a,> {
//	pub fn forward(mut self, var1: PVar<'a,>, var2: PVar<'a,>,) -> PVar<'a,> {
//		self.vars.push(var1.borrow(),);
//		self.vars.push(var2.borrow(),);
//		let pv = PVar::with(0.0,);
//		pv.borrow_mut().creator = Some(Box::new(self,),);
//		pv.borrow_mut().a = var1.borrow().a + var2.borrow().a;
//		pv
//	}
//}

mod experimental {
	#[derive(Clone,)]
	struct Var {
		pub var:     f64,
		pub creator: Option<Box<Function,>,>,
	}
	impl Var {
		pub fn new(var: f64,) -> Var { Var { var, creator: None, } }
	}

	#[derive(Clone,)]
	struct Function {
		pub v:      Vec<Var,>,
		nureric_rc: Vec<i32,>,
	}

	impl Function {
		pub fn forward(mut self, v1: Var, v2: Var,) -> Var {
			self.v.push(v1.clone(),);
			self.v.push(v2.clone(),);
			let mut pv = Var::new(0.0,);
			pv.creator = Some(Box::new(self,),);
			pv.var = v1.var + v2.var;
			pv
		}
	}
}

mod learning_smart_pointer {
	#[cfg(test)]
	mod tests {
		#[allow(unused_imports)] use super::*;

		#[test]
		fn box_test() {
			let b = Box::new(1,);
			assert_eq!(format!("{b:?}").parse::<i32>().unwrap(), *b, "{b:?}");
		}

		#[test]
		fn ptr_as_ref() {
			let x = 5;
			let y = &x;
			assert_eq!(x, *y, "x={x:?}, y={y:?}");
		}
	}
}

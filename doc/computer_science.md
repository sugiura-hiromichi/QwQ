# Rust

## `pub(path)` Conjection

> see
> [official reference](https://doc.rust-lang.org/reference/visibility-and-privacy.htmlpubin-path-pubcrate-pubsuper-and-pubself)
> [related error](https://doc.rust-lang.org/error-index.html#E0704)
> [my playground example](https://github.com/sugiura-hiromichi/QwQ/blob/master/rust/elseeee/src/main.rs)

Since 2018 Edition, `path` for `pub(path)` must start with crate, super.

## Format Modifier

### `format!` Macro

`format!` macro formats text and return `String`. And format style can be specificated by using format modifier.

here is example

```rust
let printout = "printout";
assert_eq!(format!("{printout:?}"), "\"printout\"");
```

### `p`

> see [more detail](https://doc.rust-lang.org/core/fmt/trait.Pointer.html)

show pointer's memory location by using p formatting.

```rust
let x = &42;
let address = format!("{x:p}"); // this produces something like '0x7f06092ac6d0'
```

### ``


## `fn` in `fn`

In Rust, we can define `fn` inside a `fn`. and also able to return `fn` by using `impl trait` syntax

here is example

```rust
fn return_closure() -> impl Fn() -> i32 {
	fn fn_in_main() -> impl Fn(String,) -> String { |x| x }
	|| fn_in_main()("7".to_string(),).parse::<i32>().unwrap()
}

assert_eq!(return_closure()(), 7);
```

## Type of Function and Closure

Every closure and function has it's identical type. So following code is valid.

```rust
fn closure_ornot<GenT: 'static,>(_which: GenT,) -> &'static str {
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
assert_eq!(closure_ornot(return_closure(),), "unexpected!!");

```

function's type is same as well.


```rust
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
```

## User Defined `trait`

A `trait` which is defined by user can be implemented even for primitive type (of course for types which standard library offers as well).

```rust
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
```

However, trait's method (in this case, `is()`) are only able to use within defined crate. This limitation guarantees safety.

## `Option<T>` Supports Iteration

```rust
let a = Some("a",);
for &i in a.iter() {
	assert_eq!(i, "a");
}
```
## `@` Binding in Pattern Matching

`@` binding is useful when you want to specify pattern's range

```rust
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
```

## `Option::map()` Argument

`Option::map()` takes closure as argument. The closure takes contained value in `Option<T>`.

```rust
None.map(|_one: i32| panic!("This painc won't be executed"),);
Some(1,).map(|one| assert_eq!(one, 1),);
```



# Vim

## Mode as a Motion

In Normal mode, typing `v` enter visual mode. Then type `iw` selects word the cursor is currently on.
Other example, typing `vi"` selects inner " .. ".

# Lua

`assert(v,message)` returns value of v. This is useful for nil check

```lua
local function return_nil(b)
	return b
end

assert(assert(return_nil(1)) == 1)
assert(return_nil(nil), 'a is nil') -- this should fail

print '|> 🫠'
```

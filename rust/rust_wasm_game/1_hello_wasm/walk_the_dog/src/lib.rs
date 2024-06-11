use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
// TODO: implement mutable args macos of `console!()`

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue,> {
	// redirect any panic to browser's console
	console_error_panic_hook::set_once();

	// Your code goes here!
	// let's draw triangle
	let win = web_sys::window().unwrap();
	let doc = win.document().unwrap();
	let canvas = doc
		.get_element_by_id("canvas",)
		.unwrap()
		.dyn_into::<web_sys::HtmlCanvasElement>()
		.unwrap();
	let context = canvas
		.get_context("2d",)
		.unwrap()
		.unwrap()
		.dyn_into::<web_sys::CanvasRenderingContext2d>()
		.unwrap();

	context.begin_path();
	context.move_to(300.0, 0.0,); // top of triangle
	context.line_to(0.0, 600.0,); // bottom of left triangle
	context.line_to(600.0, 600.0,); // bottom of right triangle
	context.close_path();
	context.stroke();
	context.fill();

	Ok((),)
}

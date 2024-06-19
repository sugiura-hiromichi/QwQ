use gloo_utils::format::JsValueSerdeExt;
use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use web_sys::CanvasRenderingContext2d;
// TODO: implement mutable args number macos of `console!()`

#[derive(Deserialize,)]
struct Sheet {
	frames: HashMap<String, Cell,>,
}

#[derive(Deserialize,)]
struct Cell {
	frame: Rect,
}

#[derive(Deserialize,)]
struct Rect {
	x: u16,
	y: u16,
	w: u16,
	h: u16,
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue,> {
	// redirect any panic to browser's console
	console_error_panic_hook::set_once();
	console::log_1(&wasm_bindgen::JsValue::from_str("🫠🫠🫠🫠",),);

	// Your code goes here!
	// show triangle(chapter1)
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

	// load img
	wasm_bindgen_futures::spawn_local(async move {
		let img = web_sys::HtmlImageElement::new().unwrap();
		let jsn =
			fetch_json("rhb.json",).await.expect("failed to fetch rhb.json🫠",);
		let sheet: Sheet = jsn
			.into_serde()
			.expect("failed to convert rhb.json into a Sheet struct",);
		load_img("rhb.png", &img,).await;

		let mut f = 0;
		let interval_callback = Closure::wrap(Box::new(move || {
			f = f % 8 + 1;
			let sprite = sheet
				.frames
				.get(&format!("Run ({f}).png"),)
				.expect("Cell not found",);

			context.clear_rect(0.0, 0.0, 600.0, 600.0,);
			let _=context.
			draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
				&img,
				sprite.frame.x.into(),
				sprite.frame.y.into(),
				sprite.frame.w.into(),
				sprite.frame.h.into(),
				300.0,
				300.0,
				sprite.frame.w.into(),
				sprite.frame.h.into(),
				);
		},) as Box<dyn FnMut(),>,);
		let _ = win.set_interval_with_callback_and_timeout_and_arguments_0(
			interval_callback.as_ref().unchecked_ref(),
			50,
		);
		interval_callback.forget();
	},);

	Ok((),)
}

async fn load_img(s: &str, img: &web_sys::HtmlImageElement,) {
	let (success_tx, success_rx,) =
		futures::channel::oneshot::channel::<Result<(), JsValue,>,>();

	let success_tx =
		std::rc::Rc::new(std::sync::Mutex::new(Some(success_tx,),),);
	let error_tx = std::rc::Rc::clone(&success_tx,);

	let callback = Closure::once(move || {
		if let Some(success_tx,) =
			success_tx.lock().ok().and_then(|mut opt| opt.take(),)
		{
			let _ = success_tx.send(Ok((),),);
		}
	},);
	let err_callback = Closure::once(move |e| {
		if let Some(error_tx,) =
			error_tx.lock().ok().and_then(|mut opt| opt.take(),)
		{
			let _ = error_tx.send(Err(e,),);
		}
	},);

	img.set_onload(Some(callback.as_ref().unchecked_ref(),),);
	img.set_onerror(Some(err_callback.as_ref().unchecked_ref(),),);
	img.set_src(s,);

	let _ = success_rx.await;
}

async fn fetch_json(json_path: &str,) -> Result<JsValue, JsValue,> {
	let w = web_sys::window().unwrap();
	let rsp_val =
		wasm_bindgen_futures::JsFuture::from(w.fetch_with_str(json_path,),)
			.await?;

	let rsp: web_sys::Response = rsp_val.dyn_into()?;

	wasm_bindgen_futures::JsFuture::from(rsp.json()?,).await
}

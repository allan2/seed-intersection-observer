use seed::{prelude::*, *};
use web_sys::{IntersectionObserver, IntersectionObserverInit};

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
	let doc = document();
	let msg_sender = orders.msg_sender();

	// Create the intersection observer.
	let opts = IntersectionObserverInit::new();
	let cb = js_sys::Function::new_with_args("entries", "console.log(entries)");
	let observer = IntersectionObserver::new_with_options(&cb, &opts).unwrap();

	orders.after_next_render({
		move |_| {
			let target = doc.query_selector(".box-red").unwrap().unwrap();
			observer.observe(&target);
			msg_sender(Some(Msg::Observed(observer.take_records()))); // I don't think this is working
		}
	});
	Model { records: None }
}

struct Model {
	records: Option<js_sys::Array>,
}

#[derive(Clone)]

enum Msg {
	Observed(js_sys::Array), // name can be improved
}

fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
	match msg {
		Msg::Observed(r) => model.records = Some(r),
	}
}

fn view(model: &Model) -> Node<Msg> {
	let records = match model.records.clone() {
		None => js_sys::Array::new(),
		Some(r) => r,
	};

	div![
		format!("{:#?}", records.get(0)),
		view_red_box(),
		view_blue_box()
	]
}

fn view_red_box() -> Node<Msg> {
	div![C!["box-red"], ".box-red is being observed"]
}

fn view_blue_box() -> Node<Msg> {
	div![C!["box-blue"], ".box-blue"]
}

#[wasm_bindgen(start)]
pub fn start() {
	App::start("app", init, update, view);
}

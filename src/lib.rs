use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

macro_rules! document {
    () => (
        web_sys::window().expect("no global window exist")
        .document().expect("no document")
    )
}

macro_rules! body {
    () => (
        web_sys::window().expect("no global window exist")
        .document().expect("no document")
        .body().expect("no body")
    )
}


fn add_click_event() -> Result<(), JsValue> {

    let btn = document!().get_element_by_id("button_1").unwrap();

    let on_click = Closure::wrap(Box::new(move || {
        let p = document!().create_element("p").expect("failed to create element");
        p.set_text_content(Some("Init"));
        body!().append_child(&p).expect("failed to append to body");

    }) as Box<dyn FnMut()>);

    btn.add_event_listener_with_callback("click", on_click.as_ref().unchecked_ref())?;
    on_click.forget();

    Ok(())
}

#[wasm_bindgen]
pub fn entry_point() -> Result<(), JsValue> {
    console_log!("Hello, world!");

    add_click_event()?;
    Ok(())
}

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


fn add_click_event() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    let on_click = Closure::wrap(Box::new(move || {

        let p = document.create_element("p")?;
        p.set_text_content(Some("Init"));
        body.append_child(&p);

        p.set_text_content(Some("Init"));

        
    }) as Box<dyn FnMut()>);

    body.set_onclick(Some(on_click.as_ref().unchecked_ref()));
    on_click.forget();

    //document.getElementById("button_1");

    Ok(())
}

#[wasm_bindgen]
pub fn entry_point() -> Result<(), JsValue> {
    console_log!("Hello, world!");

    add_click_event();
    Ok(())
}

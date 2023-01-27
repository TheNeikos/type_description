//
//   This Source Code Form is subject to the terms of the Mozilla Public
//   License, v. 2.0. If a copy of the MPL was not distributed with this
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
use base64::Engine;
use console_error_panic_hook::set_once as set_panic_hook;
use type_description::TypeDescription;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{window, Event, HtmlDivElement, HtmlTextAreaElement};

fn start_app() {
    let document = window()
        .and_then(|win| win.document())
        .expect("Could not access document");
    let input: HtmlTextAreaElement = document
        .query_selector("#input")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();

    let cb: Closure<dyn FnMut(_)> = Closure::new(move |_event: web_sys::Event| {
        let document = window()
            .and_then(|win| win.document())
            .expect("Could not access document");

        let input: HtmlTextAreaElement = document
            .query_selector("#input")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        let output: HtmlDivElement = document
            .query_selector("#output")
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();

        let location = document.location().unwrap();

        let encoded_input = base64::engine::general_purpose::URL_SAFE.encode(&input.value());

        location.set_hash(&encoded_input).unwrap();

        let desc: TypeDescription = serde_json::from_str(&input.value()).unwrap();

        let md = type_description::render::render_to_markdown(&desc).unwrap();

        let html = markdown::to_html(&md);

        output.set_inner_html(&html);
    });

    input.set_oninput(Some(cb.as_ref().unchecked_ref()));
    cb.forget();

    let location = document.location().unwrap();

    if let Ok(decoded_hash) =
        base64::engine::general_purpose::URL_SAFE.decode(&location.hash().unwrap()[1..])
    {
        if let Ok(value) = String::from_utf8(decoded_hash) {
            input.set_value(&value);
            let event = Event::new("input").unwrap();
            input.dispatch_event(&event).unwrap();
        }
    }
}

fn main() {
    set_panic_hook();
    start_app();
}

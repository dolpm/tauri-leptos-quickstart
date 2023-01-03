use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[tauri_glue::bind_command(name = hello)]
pub async fn hello(name: String) -> Result<JsValue, JsValue>;

#[component]
pub fn Hello(cx: Scope) -> Element {
    let (name, set_name) = create_signal(cx, "".to_string());
    view! { cx,
        <div>
            <div>
              <button on:click=move |_| spawn_local(async move {
                match hello("example_name_sent_from_frontend".to_string()).await {
                    Ok(message) => {
                        set_name.update(|name| *name = message.as_string().unwrap());
                    }
                    Err(e) => {
                        let window = window().unwrap();
                        window
                            .alert_with_message(&format!("Error: {:?}", e))
                            .unwrap();
                    }
                }
            })>"Click me!"</button>
              <span class="name">{move || name()}</span>
            </div>
        </div>
    }
}

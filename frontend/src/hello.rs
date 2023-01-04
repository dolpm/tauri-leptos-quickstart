use leptos::*;
use tauri_glue::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[tauri_glue::bind_command(name = hello)]
pub async fn hello(name: Option<String>, others: (i32, i32)) -> Result<String, String>;

#[component]
pub fn Hello(cx: Scope) -> Element {
    let (name, set_name) = create_signal(cx, "".to_string());
    view! { cx,
        <div>
            <div>
              <button on:click=move |_| spawn_local(async move {
                match hello(Some("example_name_sent_from_frontend".to_string()), (10, 1084)).await {
                    Ok(message) => {
                        set_name.update(|name| *name = message);
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

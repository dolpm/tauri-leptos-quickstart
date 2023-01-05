use bridge::{MyChildStruct, MyStruct};
use leptos::*;
use tauri_glue::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::window;

#[tauri_glue::bind_command(name = hello)]
pub async fn hello(name: String, test_struct: MyStruct) -> Result<String, String>;

#[component]
pub fn Hello(cx: Scope) -> Element {
    let (name, set_name) = create_signal(cx, "".to_string());
    view! { cx,
        <div>
            <div>
              <button on:click=move |_| spawn_local(async move {
                match hello("example_name_sent_from_frontend".to_string(), MyStruct {
                    tuple: (i32::MIN, u32::MAX, u64::MAX),
                    string: "test".to_string(),
                    child: Some(MyChildStruct {
                        v: vec![0, 1, 2, 3, 4, 5]
                    })
                }).await {
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
              <span>{move || name()}</span>
            </div>
        </div>
    }
}

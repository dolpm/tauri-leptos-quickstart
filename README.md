# tauri-leptos-quickstart
Just a simple template to get tauri communicating with a leptos front-end.

No, this combo isn't a be-all-end-all performance wise... It uses some JS glue; performance hasn't been tested but don't get your hopes up ;)

# Running the app
### dev: ```cargo tauri dev```
### prod: ```cargo tauri build```

# Calling the backend
1. create a tauri command like normal
   ```rust
   #[tauri::command]
   fn hello(name: Option<&str>) -> Result<String, String> {
       Ok(format!("Hello from Tauri, {:?} :P", name))
   }
   ```
2. use tauri-glue to create a binding on the front end:
   ```rust
   use tauri_glue::*;
   
   #[tauri_glue::bind_command(name = hello)]
   pub async fn hello(name: Option<String>) -> Result<JsValue, JsValue>;
   ```
3. call the bound function like normal
   ```rust
   match hello(Some("example_name_sent_from_frontend".to_string())).await {
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
   ```
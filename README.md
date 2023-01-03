# tauri-leptos-quickstart
Just a simple template to get tauri communicating with a leptos front-end.

No, this combo isn't a be-all-end-all performance wise... It uses some JS glue; performance hasn't been tested but don't get your hopes up ;)

# Running the app
### dev: ```cargo tauri dev```
### prod: ```cargo tauri build```

# Calling the backend
1. create a tauri command like normal
2. add the following fn to ```./frontend/public/glue.js```
   ```
   export async function invokeMyCommand(paramX, paramY) {
     return await invoke("my_command_name", { paramX, paramY });
   }
   ```
3. create the wasm binding in your frontend file
   ```
   #[wasm_bindgen(module = "/public/glue.js")]
   extern "C" {
       #[wasm_bindgen(js_name = invokeMyCommand, catch)]
       pub async fn hello(param1: String, param2: i32) -> Result<JsValue, JsValue>;
   }
   ```
1. call hello from the frontend :P
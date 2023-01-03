const invoke = window.__TAURI_INVOKE__

export async function invokeHello(name) {
  return await invoke("hello", { name });
}
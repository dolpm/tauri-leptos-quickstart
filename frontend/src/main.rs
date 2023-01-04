use frontend::hello::*;
use leptos::*;

pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <Hello />
        }
    })
}

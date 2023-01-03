use frontend::hello::*;
use leptos::*;

pub fn main() {
    mount_to_body(|cx| {
        view! { cx,
            <Hello />
        }
    })
}

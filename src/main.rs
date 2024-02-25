mod app;

use leptos::{
    mount_to_body,
    view,
};

fn main() {
    mount_to_body(|| {
        view! { <app::App/> }
    })
}



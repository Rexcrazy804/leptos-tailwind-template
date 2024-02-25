// use serde::{Deserialize, Serialize};
// use serde_wasm_bindgen::to_value;
// use wasm_bindgen::prelude::*;
//
//to invoke from tauri see src-tauri/build.rs
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
//     async fn invoke(cmd: &str, args: JsValue) -> JsValue;
// }
//
// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }
//

use leptos::{component, create_signal, view, IntoView, SignalGet, SignalUpdate};
use leptos_router::{Route, Router, Routes};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/love" view=Love/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <section> // section has h-screen applied by default (refer tailwind.css file)
            <div class="h-full flex flex-col items-center justify-center font-mono p-3">
                <a
                    class="text-pink-400 text-5xl text-center hover:text-pink-300 active:text-pink-500 transition-all duration-500"
                    href="/love"
                >
                    "Hello World!"
                </a>
            </div>
        </section>
    }
}

#[component]
fn Love() -> impl IntoView {
    let (count, set_count) = create_signal(1);
    let update_count = move |_| set_count.update(|c| *c += 1);

    view! {
        <section>
            <div class="h-full flex flex-col items-center justify-center font-mono p-3">
                <button
                    class="text-pink-400 text-5xl text-center hover:text-pink-300 active:text-pink-500 transition-all animate-bounce"
                    on:click=update_count
                >
                    {move || {
                        let count = count.get();
                        let count_str = &count.to_string();
                        let times = match count {
                            1 => "".into(),
                            _ => "x".to_string() + count_str + &"!".repeat(count_str.len()),
                        };
                        format!("I <3 Leptos {times}")
                    }}

                </button>
            </div>
        </section>
        <section>
            <div class="h-full flex flex-col items-center justify-center font-mono p-3">
                <a
                    class=
                        "text-5xl text-center text-pink-400 active:text-pink-500 transition-all duration-500 bg-ctp-surface0 rounded-sm p-3 drop-shadow-2xl active:translate-y-2 hover:-translate-y-2"
                    href="/"
                >
                    "Go Home!"
                </a>
            </div>
        </section>
    }
}

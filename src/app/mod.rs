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

use leptos::{
    component,
    view,
    create_signal,
    IntoView, SignalUpdate, SignalGet,
};
use leptos_router::{
    Router,
    Routes,
    Route,
};

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
        <section> // tailwind.css file defines a defualt utility class for sections: h-screen
            <div class="h-full flex flex-col items-center justify-center font-mono p-3">
                <a
                    class="text-pink-300 text-5xl text-center hover:text-pink-400 active:text-pink-500 transition-all duration-500"
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
    let update_count = move |_| {
        set_count.update(|c| *c += 1)
    };

    view! {
        <section> 
            <div class="h-full flex flex-col items-center justify-center font-mono p-3">
                <button
                    class="text-pink-300 text-5xl text-center hover:text-pink-400 active:text-pink-500 transition-all animate-bounce"
                    on:click = update_count
                >
                {
                    move || {
                        let count = count.get();
                        let times = match count {
                            1 => "".into(),
                            _ => {
                                "x".to_string() + &count.to_string() + &"!".repeat(count.to_string().len())
                            },
                        };
                        format!("I <3 Leptos {times}")
                    }
                }
                </button>
            </div>
        </section>
        <section>
            <div class="h-full flex flex-col items-center justify-center font-mono p-3">
                <a
                    class="text-pink-300 text-5xl text-center hover:text-pink-400 active:text-pink-500 transition-all duration-500"
                    href="/"
                >
                    "Go Home!"
                </a>
            </div>
        </section>
    }
}

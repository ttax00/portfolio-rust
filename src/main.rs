mod components;

use components::{Introduction, NavBar, Technologies};
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <NavBar />
            <main class="bg-slate-100 flex flex-col bg-gradient-to-br from-slate-100 to-slate-400 dark:from-slate-600 dark:to-red-900">
            <Introduction/>
            <Technologies/>
            </main>
            <footer class="text-center text-black dark:text-white bg-gray-500 dark:bg-gray-800 pb-8 pt-6">{"Designed & Built by TechTheAwesome"}</footer>

        </>
    }
}

#[function_component(Index)]
fn index() -> Html {
    html! {
        <>
            <noscript>{"You need to enable JavaScript to run this app."}</noscript>
            <App/>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Index>();
}

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::{
    console::{self, log},
    window,
};
use yew::prelude::*;

#[function_component]
fn Counter() -> Html {
    let counter = use_state(|| 0);
    let increment = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    let decrement = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };
    let set_zero = {
        let counter = counter.clone();
        move |_| {
            counter.set(0);
        }
    };

    html! {
        <div>
            <p>{ *counter }</p>
            <button onclick={decrement}>{ "-1" }</button>
            <button onclick={increment}>{ "+1" }</button>
            <button onclick={set_zero}>{ "0" }</button>
        </div>
    }
}

#[function_component]
fn AlertDisplay() -> Html {
    let display_alert = move |_: MouseEvent| {
        window().unwrap().alert_with_message("Hello!").unwrap();
    };
    html! {
        <button onclick={display_alert}>{"Display Alert"}</button>
    }
}

#[function_component]
fn ShowConsoleMessage() -> Html {
    let show_console = move |_: MouseEvent| console::log_1(&JsValue::from_str("Hello"));
    html! {
        <button onclick={show_console}>{"Show console"}</button>
    }
}

#[function_component]
fn DataFetchingComponent() -> Html {
    let window = window().unwrap();

    spawn_local(async move {
        JsFuture::from(window.fetch_with_str("https://jsonplaceholder.typicode.com/posts"))
            .await
            .unwrap();
    });

    html! {
        <div></div>
    }
}

#[function_component]
pub fn App() -> Html {
    let title = "Hello, Yew!";
    html! {
        <div>
            <h1>{title}</h1>
            <p>{"This is a simple Yew app."}</p>
        <Counter />
        <AlertDisplay />
        <ShowConsoleMessage />
        <DataFetchingComponent />
        </div>
    }
}

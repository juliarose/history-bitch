use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, KeyboardEvent};

use crate::app::AppState;

pub struct HeaderProps {
    pub app_state: AppState
}

#[component(Header<G>)]
pub fn header(props: HeaderProps) -> Template<G> {
    let HeaderProps { app_state } = props;
    let value = Signal::new(String::new());
    let handle_submit = cloned!((app_state, value) => move |event: Event| {
        let event: KeyboardEvent = event.unchecked_into();

        if event.key() == "Enter" {
            let mut name = value.get().as_ref().clone();
            name = name.trim().to_string();

            if !name.is_empty() {
                value.set("".to_string());
            }
        }
    });

    template! {
        header(class="header") {
            input(class="new-item",
                placeholder="yes?",
                bind:value=value,
                on:keyup=handle_submit,
            )
        }
    }
}
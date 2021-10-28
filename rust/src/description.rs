
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlDivElement, KeyboardEvent};
use crate::parser;

pub struct DescriptionProps {
    pub item: Signal::<parser::Item>,
    pub description: Signal<parser::Description>,
}

#[component(Description<G>)]
pub fn description(props: DescriptionProps) -> Template<G> {
    let DescriptionProps { item, description } = props;
    let value = Signal::new(description.get().value.clone().unwrap_or("".to_string()));
    let has_value = Signal::new(!description.get().value.is_none());
    let mut style: String = String::from("");
    
    if let Some(color) = &description.get().color {
        style = format!("color: #{}", color);
    }
    
    template! {
        (if *has_value.get() {
            cloned!((value, style) => template! {
                li(class="description", style=style) {
                    (value.get())
                }
            })
        } else {
            Template::empty()
        })
    }
}
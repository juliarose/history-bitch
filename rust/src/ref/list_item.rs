
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, KeyboardEvent};
use crate::app::{AppState, Item};

pub struct ListItemProps {
    pub item: Signal<Item>,
    pub app_state: AppState,
}

#[component(ListItem<G>)]
pub fn list_item(props: ListItemProps) -> Template<G> {
    let ListItemProps { item, app_state } = props;
    let name = cloned!((item) => move || item.get().name.clone());
    let id = item.get().id;
    let editing = Signal::new(false);
    let input_ref = NodeRef::<G>::new();
    let value = Signal::new("".to_string());
    let handle_input = cloned!((value) => move |event: Event| {
        let target: HtmlInputElement = event.target().unwrap().unchecked_into();
        value.set(target.value());
    });
    let handle_blur = cloned!((item, app_state, editing, value) => move || {
        editing.set(false);
        
        let mut value = value.get().as_ref().clone();
        
        value = value.trim().to_string();

        if value.is_empty() {
            app_state.remove_item(id);
        } else {
            item.set(Item {
                name: value,
                ..item.get().as_ref().clone()
            })
        }
    });
    let handle_dblclick = cloned!((name, editing, input_ref, value) => move |_| {
        editing.set(true);
        input_ref.get::<DomNode>().unchecked_into::<HtmlInputElement>().focus().unwrap();
        value.set(name());
    });
    let handle_submit = cloned!((editing, input_ref, handle_blur, name) => move |event: Event| {
        let event: KeyboardEvent = event.unchecked_into();
        match event.key().as_str() {
            "Enter" => handle_blur(),
            "Escape" => {
                input_ref.get::<DomNode>().unchecked_into::<HtmlInputElement>().set_value(&name());
                editing.set(false);
            },
            _ => {}
        }
    });
    let handle_destroy = cloned!((app_state) => move |_| {
        app_state.remove_item(id);
    });
    
    let class = cloned!((editing) => move || {
        format!("{}",
            if *editing.get() { "editing" } else { "" }
        )
    });

    template! {
        li(class=class()) {
            div(class="view") {
                label(on:dblclick=handle_dblclick) {
                    (name())
                }
                button(class="destroy", on:click=handle_destroy) {
                    "Destroy"
                }
            }

            (if *editing.get() {
                cloned!((item, input_ref, handle_blur, handle_submit, handle_input) => template! {
                    input(ref=input_ref,
                        class="edit",
                        value=item.get().name.clone(),
                        on:blur=move |_| handle_blur(),
                        on:keyup=handle_submit,
                        on:input=handle_input,
                    )
                })
            } else {
                Template::empty()
            })
        }
    }
}
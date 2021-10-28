use sycamore::prelude::*;

use crate::app::{AppState};
use crate::list_item::{ListItemProps, ListItem};

#[component(List<G>)]
pub fn list(app_state: AppState) -> Template<G> {
    let items = create_memo(cloned!((app_state) => move || {
        app_state.items.get().iter().cloned().collect::<Vec<_>>()
    }));

    template! {
        section(class="main") {
            ul(class="todo-list") {
                Keyed(KeyedProps {
                    iterable: items,
                    template: move |item| template! {
                        ListItem(ListItemProps { item, app_state: app_state.clone() })
                    },
                    key: |item| item.get().id,
                })
            }
        }
    }
}
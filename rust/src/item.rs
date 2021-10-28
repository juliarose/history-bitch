
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlDivElement, KeyboardEvent};
use crate::{parser};
use crate::description::{Description, DescriptionProps};
use log::info;

pub struct ItemProps {
    pub item: Signal<parser::Item>,
}

#[component(Item<G>)]
pub fn item(props: ItemProps) -> Template<G> {
    let ItemProps { item } = props;
    let assetid = item.get().asset.assetid.clone();
    let div_ref = NodeRef::<G>::new();
    let value = Signal::new("".to_string());
    let id = cloned!((item) => move || item.get().asset.assetid.clone().to_string());
    let handle_dblclick = cloned!((assetid, div_ref, value) => move |_| {
        info!("{}", assetid);
        div_ref.get::<DomNode>().unchecked_into::<HtmlDivElement>().set_inner_text(&id());
        value.set(id());
    });
    let classinfo_s = item.get().classinfo.clone();
    let descriptions_memo = create_memo(cloned!((classinfo_s) => move || {
        if let Some(classinfo) = &classinfo_s {
            if let Some(descriptions) = &classinfo.descriptions {
                return descriptions
                    .iter()
                    .cloned()
                    .filter(|description| {
                        !description.value.is_none()
                    })
                    .map(|description| Signal::new(description))
                    .collect::<Vec<_>>();
            }
        }
        
        Vec::new()
    }));
    let mut style: String = String::from("");
    
    if let Some(classinfo) = &item.get().classinfo {
        if let Some(name_color) = &classinfo.name_color {
            style = format!("color: #{}", name_color);
        }
    }

    template! {
        li(class="item") {
            (match item.get().classinfo.clone() {
                Some(classinfo) => {
                    cloned!((item, classinfo, style, handle_dblclick, descriptions_memo) => {
                        template! {
                            div(class="item.name",
                                on:dblclick=handle_dblclick,
                                style=style
                            ) {
                                (classinfo.market_hash_name.clone().to_string())
                            }
                            
                            ul(class="item.descriptions") {
                                Keyed(KeyedProps {
                                    iterable: descriptions_memo,
                                    template: move |description| template! {
                                        Description(DescriptionProps { item: item.clone(), description })
                                    },
                                    key: |_description| 0,
                                })
                            }
                        }
                    })
                },
                None => {
                    template! {
                        div(class="item.name") {
                            ("unknown item".to_string())
                        }
                    }
                }
            })
            
            div(ref=div_ref,
                class="edit"
            ) {
                ("")
            }
        }
    }
}
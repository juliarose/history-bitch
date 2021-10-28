
use sycamore::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, KeyboardEvent};
use crate::app::{AppState};
use crate::parser::MappedTrade;
use crate::item::{ItemProps, Item};

pub struct TradeListItemProps {
    pub trade: Signal<MappedTrade>,
}

#[component(TradeListItem<G>)]
pub fn trade_list_item(props: TradeListItemProps) -> Template<G> {
    let TradeListItemProps { trade } = props;
    let id = Signal::new(trade.get().tradeid.clone());
    let has_assets_received = Signal::new(trade.get().clone().assets_received.len() > 0 as usize);
    let assets_received_memo = create_memo(cloned!((trade) => move || {
        trade.get().assets_received.iter().cloned().map(|trade| Signal::new(trade)).collect::<Vec<_>>()
    }));
    let has_assets_given = Signal::new(trade.get().clone().assets_given.len() > 0 as usize);
    let assets_given_memo = create_memo(cloned!((trade) => move || {
        trade.get().assets_given.iter().cloned().map(|trade| Signal::new(trade)).collect::<Vec<_>>()
    }));

    template! {
        li(class="trade") {
            (format!("Trade #{}", id.get().to_string()))
            
            (if *has_assets_received.get() {
                cloned!((assets_received_memo) => template! {
                    h4 { "Received" }
                    ul(class="items received") {
                        Keyed(KeyedProps {
                            iterable: assets_received_memo,
                            template: move |item| template! {
                                Item(ItemProps { item })
                            },
                            key: |item| item.get().asset.assetid,
                        })
                    }
                })
            } else {
                Template::empty()
            })
            
            (if *has_assets_given.get() {
                cloned!((assets_given_memo) => template! {
                    h4 { "Given" }
                    ul(class="items given") {
                        Keyed(KeyedProps {
                            iterable: assets_given_memo,
                            template: move |item| template! {
                                Item(ItemProps { item })
                            },
                            key: |item| item.get().asset.assetid,
                        })
                    }
                })
            } else {
                Template::empty()
            })
        }
    }
}
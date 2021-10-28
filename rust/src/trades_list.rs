use sycamore::prelude::*;
use crate::app::{AppState};
use crate::{trade_list_item};

pub struct TradesListProps {
    pub app_state: StateHandle<AppState>,
}

#[component(TradesList<G>)]
pub fn trades_list(props: TradesListProps) -> Template<G> {
    let TradesListProps { app_state } = props;
    let trades_memo = create_memo(cloned!((app_state) => move || {
        app_state.get().trades.get().iter().cloned().map(|trade| trade).collect::<Vec<_>>()
    }));
    
    template! {
        ul(class="trades") {
            Keyed(KeyedProps {
                iterable: trades_memo,
                template: move |trade| template! {
                    trade_list_item::TradeListItem(trade_list_item::TradeListItemProps { trade })
                },
                key: |trade| trade.get().tradeid,
            })
        }
    }
}
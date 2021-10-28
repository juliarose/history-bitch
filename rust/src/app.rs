
use sycamore::prelude::*;
use crate::{trades_list, parsing, parser};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct AppState {
    pub trades: Signal<Vec<Signal<parser::MappedTrade>>>,
}

impl AppState {
    
    pub fn add_trade(&self, trade: parser::MappedTrade) {
        self.trades.set(
            self.trades
                .get()
                .as_ref()
                .clone()
                .into_iter()
                .chain(Some(Signal::new(trade)))
                .collect(),
        )
    }
    
    pub fn update_from_api(&self) {
        // wasm_bindgen_futures::spawn_local(async move {
        //     let trades = parsing::get_trades().await;
            
        //     match trades {
        //         Ok(trades) => {
        //             for trade in trades {
        //                 self.add_trade(trade);
        //             }
        //         },
        //         Err(_error) => {
        //             // error
        //         },
        //     };
        // });
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            trades: Signal::new(Vec::new()),
        }
    }
}

// fn create_state() -> Signal<&'static AppState> {
//     let state = AppState {
//         trades: Signal::new(Vec::new()),
//     };
//     let app_state: Signal<&'static AppState> = Signal::new(&state);
    
//     app_state
// }

#[component(App<G>)]
pub fn app() -> Template<G> {
    let app_state = Signal::new(AppState::default());
    
    // app_state.get().update_from_api();
    // wasm_bindgen_futures::spawn_local(async move {
    //     let trades = parsing::get_trades().await;
        
    //     match trades {
    //         Ok(trades) => {
    //             for trade in trades {
    //                 app_state.get().add_trade(trade);
    //             }
    //         },
    //         Err(_error) => {
    //             // error
    //         },
    //     };
    // });
    
    template! {
        div(class="wrapper") {
            trades_list::TradesList(trades_list::TradesListProps {
                app_state: app_state.handle()
            })
        }
    }
}
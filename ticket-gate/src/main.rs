use std::sync::{Arc, RwLock};

use anyhow::Result;
use clap::Parser;
use ticket_house::Event;
use tracing::info;

mod docker;
mod events_reader;
mod metadata;
mod state;
mod web_api;

#[derive(Debug, Parser, Clone)]
struct Args {
    /// The address of the ticket house contract
    #[arg(long)]
    ticket_house_address: String,
    /// Log parser
    #[arg(long)]
    event_log_emitter: String,
}

#[rocket::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    let state = Arc::new(RwLock::new(state::State::default()));
    {
        let state = state.clone();
        std::thread::spawn(move || {
            events_reader::read_events(&args, |event| {
                match event {
                    Event::Recharged(v) => {
                        info!("Recharged: {:?}", v);
                        let mut state = state.write().unwrap();
                        state.recharge(&v.account, v.amount);
                    }
                    _ => {}
                }
                Ok(())
            })
        });
    }
    web_api::http_serve(state).await?;
    Ok(())
}

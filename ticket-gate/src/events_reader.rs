use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use crate::metadata;
use anyhow::Result;
use scale::Decode;
use ticket_house::Event;

use super::Args;

pub(crate) fn read_events(
    args: &Args,
    mut on_event: impl FnMut(Event) -> Result<()>,
) -> Result<()> {
    let house_address =
        hex::decode(args.ticket_house_address.trim_start_matches("0x")).expect("Invalid address");
    let log_reader = Command::new("bash")
        .arg("-c")
        .arg(&args.event_log_emitter)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let stdout = log_reader.stdout.expect("Failed to read log");
    let stdout = BufReader::new(stdout);
    for line in stdout.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                anyhow::bail!("Failed to read line: {}", err);
            }
        };
        if !line.contains("phactory::event_chain:") {
            continue;
        }
        if let Some(block) = parse_event_block(&line) {
            // TODO: verify signature
            let block = hex::decode(block.payload).unwrap();
            let events = metadata::decode_contract_emits(&block).unwrap();
            for event in events {
                if event.contract.as_ref() == house_address {
                    let Ok(event) = Event::decode(&mut &event.data[..]) else {
                        continue;
                    };
                    on_event(event)?;
                }
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
struct EventBlock<'a> {
    payload: &'a str,
    // signature: &'a str,
    // pubkey: &'a str,
}

fn parse_event_block(log_line: &str) -> Option<EventBlock<'_>> {
    let payload = extract_value(log_line, "payload=")?;
    Some(EventBlock { payload })
}

fn extract_value<'a>(log_line: &'a str, prefix: &str) -> Option<&'a str> {
    let start = log_line.find(prefix)?;
    let remain = log_line[start..].trim_start_matches(prefix);
    match remain.find(' ') {
        Some(index) => Some(&remain[..index]),
        None => Some(remain),
    }
}

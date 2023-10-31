use clap::Parser;
use scale::Decode;
use ticket_house::Event;

mod metadata;

#[derive(Debug, Parser)]
struct Args {
    /// The address of the ticket house contract
    #[arg(long)]
    ticket_house_address: String,
    /// Log parser
    #[arg(long)]
    event_log_emitter: String,
}

fn main() {
    let blocks_file = include_str!("../assets/events.log");
    let house_address =
        hex::decode("d96713cfb189bf78effa371417dc3c2f8722122382b0e28abf0bfdd6850ef549").unwrap();
    for line in blocks_file.lines() {
        if let Some(block) = parse_event_block(line) {
            let block = hex::decode(block.payload).unwrap();
            let events = metadata::decode_contract_emits(&block).unwrap();
            for event in events {
                if event.contract.as_ref() == house_address {
                    let event = Event::decode(&mut &event.data[..]).unwrap();
                    match event {
                        Event::Extracted(_) => todo!(),
                        Event::Recharged(charged) => {
                            println!("event: {charged:?}");
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct EventBlock<'a> {
    payload: &'a str,
    signature: &'a str,
    pubkey: &'a str,
}

fn parse_event_block(log_line: &str) -> Option<EventBlock<'_>> {
    let payload = extract_value(log_line, "payload=")?;
    let signature = extract_value(log_line, "signature=")?;
    let pubkey = extract_value(log_line, "pubkey=")?;
    Some(EventBlock {
        payload,
        signature,
        pubkey,
    })
}

fn extract_value<'a>(log_line: &'a str, prefix: &str) -> Option<&'a str> {
    let start = log_line.find(prefix)?;
    let remain = log_line[start..].trim_start_matches(prefix);
    match remain.find(' ') {
        Some(index) => Some(&remain[..index]),
        None => Some(remain),
    }
}

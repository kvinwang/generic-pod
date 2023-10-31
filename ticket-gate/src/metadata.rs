use anyhow::{Context, Result};
use scale::Decode;
use subxt::{config::substrate::H256, utils::AccountId32};
use types::pink::rt::{EventsBlockBody, EventsBlockHeader};

#[derive(Debug)]
pub struct ContractEmitted {
    pub event_block_number: u64,
    pub phala_block_number: u32,
    pub contract: AccountId32,
    pub data: Vec<u8>,
    pub topics: Vec<H256>,
}

macro_rules! build_runtime_types {
    ($(($major: literal, $minor: literal) => ($module: ident, $path: literal);)*) => {
        $(
            #[subxt::subxt(runtime_metadata_path = $path)]
            pub mod $module {
                use ::subxt::utils::{H256 as Hash, AccountId32 as AccountId};
                pub type SystemEvents = Vec<
                    EventRecord<Event, Hash>
                >;
                pub use runtime_types::frame_system::{EventRecord, Phase};
                pub use runtime_types::pallet_contracts::pallet::Event as ContractsEvent;
                pub fn try_into_contract_event(
                    event: EventRecord<Event, Hash>,
                ) -> Result<ContractsEvent, EventRecord<Event, Hash>> {
                    match event.event {
                        Event::Contracts(e) => Ok(e),
                        _ => Err(event),
                    }
                }
                pub fn try_into_contract_emitted(
                    event: ContractsEvent,
                ) -> Result<(AccountId, Vec<u8>), ContractsEvent> {
                    match event {
                        ContractsEvent::ContractEmitted { contract, data } => Ok((contract, data)),
                        _ => Err(event),
                    }
                }
            }
        )*

        pub fn decode_contract_emits(block: &[u8]) -> Result<Vec<ContractEmitted>> {
            let mut buf = &block[..];
            let header = EventsBlockHeader::decode(&mut buf).context("Failed to decode header")?;
            match header.runtime_version {
                $(
                    ($major, $minor) => {
                        use $module::{ContractsEvent, Event, SystemEvents};
                        let body = EventsBlockBody::<SystemEvents>::decode(&mut buf)
                            .context("Failed to decode body")?;
                        let events = body.events
                            .into_iter()
                            .filter_map(|event| match event.event {
                                Event::Contracts(ContractsEvent::ContractEmitted {
                                    contract,
                                    data,
                                }) => Some(ContractEmitted {
                                    event_block_number: header.number,
                                    phala_block_number: body.phala_block_number,
                                    contract,
                                    data,
                                    topics: event.topics,
                                }),
                                _ => None,
                            })
                            .collect();
                        Ok(events)
                    }
                )*
                _ => anyhow::bail!("Unsupported runtime version: {:?}", header.runtime_version),
            }
        }
    };
}

build_runtime_types! {
    (1, 1) => (v1_1, "metadata/metadata-1.1.bin");
    (1, 2) => (v1_2, "metadata/metadata-1.2.bin");
}

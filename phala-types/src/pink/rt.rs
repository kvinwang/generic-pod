use scale::{Decode, Encode};
use sp_core::crypto::AccountId32;

pub type Hash = sp_core::H256;
pub type AccountId = AccountId32;
pub type BlockNumber = u32;

#[derive(Encode, Decode, Clone, Debug)]
pub struct EventsBlockHeader {
    pub number: u64,
    pub runtime_version: (u32, u32),
    pub parent_hash: Hash,
    pub body_hash: Hash,
}

// The body layout may change in the future. They should be decoded as type determined by
// header.runtime_version.
#[derive(Encode, Decode, Clone, Debug)]
pub struct EventsBlockBody<SystemEvents> {
    pub phala_block_number: BlockNumber,
    pub contract_call_nonce: Option<Vec<u8>>,
    pub entry_contract: Option<AccountId>,
    pub events: SystemEvents,
}

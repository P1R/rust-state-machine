use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
	pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce:BTreeMap::new()
        }
	}
}

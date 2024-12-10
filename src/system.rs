use std::{collections::BTreeMap, ops::Add};

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

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        //crashes if overflow for purpose
        self.block_number = self.block_number.checked_add(1).unwrap();
    }

    pub fn inc_nonce(&mut self, who: &String) {
        let nonce = self.nonce.get(who).unwrap_or(&0);
        self.nonce.insert(who.clone(), nonce + 1);
    }
    pub fn get_nonce(&self, who: &String) -> u32 {
        *self.nonce.get(who).unwrap_or(&0)
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn init_system(){
        let system = super::Pallet::new();
        assert_eq!(system.block_number(), 0);
    }

    #[test]
    fn inc_block_number(){
        let mut system = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number, 1);
    }

    #[test]
    fn inc_nonce(){
        let alice = String::from("alice");
        let mut system = super::Pallet::new();
        system.inc_nonce(&alice.clone());
        assert_eq!(system.get_nonce(&alice.clone()), 1);
    }
}

use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer (
        &mut self,
        caller: String,
        to: String,
        amount: u128
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(amount).
            ok_or("Insufficient balance")?;

        let new_to_balance = to_balance
            .checked_add(amount)
            .ok_or("Overflow when adding to balance")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);


        Ok(())
    }
}

#[cfg(test)]
mod tests {
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

		assert_eq!(balances.balance(&alice), 0);
		balances.set_balance(&alice, 100);
		assert_eq!(balances.balance(&alice), 100);
		assert_eq!(balances.balance(&bob), 0);
	}

	#[test]
	fn transfer_balance() {
        let mut balances = super::Pallet::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        balances.set_balance(&alice, 100);

        let _ = balances.transfer(alice.clone(), bob.clone(), 90);

        assert_eq!(balances.balance(&alice), 10);
        assert_eq!(balances.balance(&bob), 90);

    }
	#[test]
	fn transfer_balance_insufficient() {
        let mut balances = super::Pallet::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        balances.set_balance(&alice, 100);

        let result = balances.transfer(alice.clone(), bob.clone(), 110);

        assert_eq!(result, Err("Insufficient balance"));

        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), 0);

    }
	#[test]
	fn transfer_balance_overflow() {
        let mut balances = super::Pallet::new();
        let alice = "alice".to_string();
        let bob = "bob".to_string();

        balances.set_balance(&alice, 100);
        balances.set_balance(&bob, u128::MAX);

        let result = balances.transfer(alice.clone(), bob.clone(), 1);

        assert_eq!(result, Err("Overflow when adding to balance"));

        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), u128::MAX);

    }

}


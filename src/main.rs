mod balances;
mod system;

fn main() {
	println!("Hello, world!");
    let mut balance = balances::Pallet::new();
    let mut system = system::Pallet::new();
}


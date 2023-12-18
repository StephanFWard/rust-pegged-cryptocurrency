// Import necessary dependencies
use frame_system::Module as System;
use frame_system::RawEvent;
use frame_support::{decl_module, decl_storage, decl_event, ensure, StorageMap};
use sp_runtime::traits::{CheckedAdd, CheckedSub, Member, One, Zero};
use sp_std::vec::Vec;

// Define the currency trait
pub trait Trait: frame_system::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    type Currency: Currency<Self::AccountId>;
}

// Define the Currency trait
pub trait Currency<AccountId> {
    fn deposit_creating(account: &AccountId, value: u64);
    fn withdraw(account: &AccountId, value: u64) -> Result<(), &'static str>;
}

// Implement the Currency trait for the runtime
impl<T: Trait> Currency<T::AccountId> for Module<T> {
    fn deposit_creating(account: &T::AccountId, value: u64) {
        <Balances<T>>::mutate(account, |balance| {
            *balance = balance.checked_add(&value).unwrap_or_else(|| {
                frame_support::print("Error: Overflow during deposit");
                *balance
            });
        });
    }

    fn withdraw(account: &T::AccountId, value: u64) -> Result<(), &'static str> {
        <Balances<T>>::mutate(account, |balance| {
            *balance = balance.checked_sub(&value).ok_or("Error: Insufficient funds")?;
            Ok(())
        })
    }
}

// Define the module
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Initialize the module
        fn deposit_event() = default;

        // Mint new pegged tokens by depositing the corresponding amount in the reserve
        pub fn mint(origin, amount: u64) {
            let sender = ensure_signed(origin)?;

            // Deposit the corresponding amount in the reserve
            T::Currency::deposit_creating(&sender, amount);

            // Mint new pegged tokens to the sender
            <TotalSupply<T>>::mutate(|supply| {
                *supply = supply.checked_add(&amount).unwrap_or_else(|| {
                    frame_support::print("Error: Overflow during minting");
                    *supply
                });
            });

            // Emit an event
            Self::deposit_event(RawEvent::Minted(sender, amount));
        }

        // Burn pegged tokens and withdraw the corresponding amount from the reserve
        pub fn burn(origin, amount: u64) {
            let sender = ensure_signed(origin)?;

            // Withdraw the corresponding amount from the reserve
            T::Currency::withdraw(&sender, amount)?;

            // Burn the pegged tokens from the sender
            <TotalSupply<T>>::mutate(|supply| {
                *supply = supply.checked_sub(&amount).ok_or("Error: Insufficient supply")?;
            });

            // Emit an event
            Self::deposit_event(RawEvent::Burned(sender, amount));
        }
    }
}

// Define the storage items
decl_storage! {
    trait Store for Module<T: Trait> as PeggedToken {
        // Store the total supply of pegged tokens
        TotalSupply get(fn total_supply): u64;

        // Store the account balances of pegged tokens
        Balances get(fn balances): map hasher(twox_64_concat) T::AccountId => u64;
    }
}

// Define the events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        Minted(AccountId, u64),
        Burned(AccountId, u64),
    }
);

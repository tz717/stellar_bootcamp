use soroban_sdk::{contract, contractevent, contractimpl, contracttype, Address, Env};

#[contracttype]
pub enum DataKey {
    // Stores the Address of the person who can claim the funds
    Claimant,
    // Stores the ledger sequence number when the funds can be released
    ReleaseLedger,
    // Stores the status: true if funds have been claimed, false otherwise
    Claimed,
}

#[contract]
pub struct LockboxContract;

#[contractimpl]
impl LockboxContract {
    pub fn initialize(env: Env, claimant: Address, release_ledger: u32) {
        let storage = env.storage().persistent();

        if storage.has(&DataKey::Claimant) {
            panic!("Contract is already initialized");
        }

        storage.set(&DataKey::Claimant, &claimant);
        storage.set(&DataKey::ReleaseLedger, &release_ledger);
        storage.set(&DataKey::Claimed, &false); // Initialize status as unclaimed
    }

    pub fn claim(env: Env, claimant: Address) {
        let storage = env.storage().persistent();

        claimant.require_auth();
        let stored_claimant: Address = storage
            .get(&DataKey::Claimant)
            .unwrap_or_else(|| panic!("Claimant not set"));

        if claimant != stored_claimant {
            panic!("Only the designated claimant can claim funds");
        }
        let is_claimed: bool = storage
            .get(&DataKey::Claimed)
            .unwrap_or_else(|| panic!("Claimed status not set"));

        if is_claimed {
            panic!("Funds have already been claimed");
        }

        let release_ledger: u32 = storage
            .get(&DataKey::ReleaseLedger)
            .unwrap_or_else(|| panic!("Release ledger not set"));

        let current_ledger: u32 = env.ledger().sequence();

        if current_ledger < release_ledger {
            panic!(
                "Funds are still locked. Release ledger: {}, Current ledger: {}",
                release_ledger, current_ledger
            );
        }

        storage.set(&DataKey::Claimed, &true);
    }
}

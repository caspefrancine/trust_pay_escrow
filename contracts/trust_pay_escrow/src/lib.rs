#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol};

// Define storage keys for escrow state
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Escrow(u64), // Maps Task ID to Escrow Details
}

// Define Escrow Status
#[contracttype]
#[derive(Clone, PartialEq, Debug)]
pub enum EscrowStatus {
    Funded,
    Submitted,
    Completed,
}

// Struct to store Escrow details on-chain
#[contracttype]
#[derive(Clone, Debug)]
pub struct Escrow {
    pub client: Address,
    pub worker: Address,
    pub amount: i128,
    pub task_url: String,
    pub status: EscrowStatus,
}

#[contract]
pub struct TrustPayContract;

#[contractimpl]
impl TrustPayContract {
    /// 1. Client locks funds in escrow for a specific task ID
    pub fn create_escrow(
        env: Env,
        task_id: u64,
        client: Address,
        worker: Address,
        amount: i128,
    ) {
        // Ensure client authorizes the transaction
        client.require_auth();

        // Save initial escrow state as Funded
        let escrow = Escrow {
            client: client.clone(),
            worker,
            amount,
            task_url: String::from_str(&env, ""),
            status: EscrowStatus::Funded,
        };

        env.storage().persistent().set(&DataKey::Escrow(task_id), &escrow);
    }

    /// 2. Worker submits completed work link
    pub fn submit_work(env: Env, task_id: u64, worker: Address, task_url: String) {
        worker.require_auth();

        let mut escrow: Escrow = env
            .storage()
            .persistent()
            .get(&DataKey::Escrow(task_id))
            .expect("Escrow not found");

        assert_eq!(escrow.worker, worker, "Unauthorized worker");
        assert_eq!(escrow.status, EscrowStatus::Funded, "Invalid status for submission");

        escrow.task_url = task_url;
        escrow.status = EscrowStatus::Submitted;

        env.storage().persistent().set(&DataKey::Escrow(task_id), &escrow);
    }

    /// 3. Client approves task and releases funds to worker
    pub fn release_payment(env: Env, task_id: u64, client: Address) {
        client.require_auth();

        let mut escrow: Escrow = env
            .storage()
            .persistent()
            .get(&DataKey::Escrow(task_id))
            .expect("Escrow not found");

        assert_eq!(escrow.client, client, "Only client can release payment");
        assert_eq!(
            escrow.status,
            EscrowStatus::Submitted,
            "Work must be submitted before release"
        );

        // Update status to Completed
        escrow.status = EscrowStatus::Completed;
        env.storage().persistent().set(&DataKey::Escrow(task_id), &escrow);
    }

    /// Helper to fetch escrow state
    pub fn get_escrow(env: Env, task_id: u64) -> Escrow {
        env.storage()
            .persistent()
            .get(&DataKey::Escrow(task_id))
            .expect("Escrow not found")
    }
}
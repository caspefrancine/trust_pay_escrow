#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Admin,
    Issuer(Address),
    DocHash(String),
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct DocumentRecord {
    pub issuer: Address,
    pub timestamp: u64,
    pub is_valid: bool,
}

#[contract]
pub struct FrontlineLedger;

#[contractimpl]
impl FrontlineLedger {
    /// 1. Initialize Admin
    pub fn initialize(env: Env, admin: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already initialized");
        }
        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
    }

    /// 2. Add Authorized Issuer (RBAC)
    pub fn add_issuer(env: Env, issuer: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).expect("Not initialized");
        admin.require_auth();
        env.storage().persistent().set(&DataKey::Issuer(issuer.clone()), &true);
    }

    /// 3. Issue Document Hash (SHA-256)
    pub fn issue_doc(env: Env, issuer: Address, doc_hash: String) {
        issuer.require_auth();
        let is_authorized: bool = env.storage().persistent().get(&DataKey::Issuer(issuer.clone())).unwrap_or(false);
        if !is_authorized {
            panic!("Not an authorized issuer");
        }

        let record = DocumentRecord {
            issuer,
            timestamp: env.ledger().timestamp(),
            is_valid: true,
        };

        env.storage().persistent().set(&DataKey::DocHash(doc_hash), &record);
    }

    /// 4. Verify Document Hash
    pub fn verify_doc(env: Env, doc_hash: String) -> Option<DocumentRecord> {
        env.storage().persistent().get(&DataKey::DocHash(doc_hash))
    }

    /// 5. Revoke Document
    pub fn revoke_doc(env: Env, issuer: Address, doc_hash: String) {
        issuer.require_auth();
        let mut record: DocumentRecord = env.storage().persistent().get(&DataKey::DocHash(doc_hash.clone())).expect("Document not found");
        if record.issuer != issuer {
            panic!("Only the original issuer can revoke");
        }
        record.is_valid = false;
        env.storage().persistent().set(&DataKey::DocHash(doc_hash), &record);
    }
}

#[cfg(test)]
mod test;

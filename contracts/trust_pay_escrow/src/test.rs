#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_frontline_ledger_full_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, FrontlineLedger);
    let client = FrontlineLedgerClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let issuer = Address::generate(&env);
    let doc_hash = String::from_str(&env, "60ab6a7aa7d6abde4df6730a8a976cbb18e4325bb6d33220b453186b8b11ece2");

    // 1. Initialize Contract
    client.initialize(&admin);

    // 2. Add Issuer
    client.add_issuer(&issuer);

    // 3. Issue Document
    client.issue_doc(&issuer, &doc_hash);

    // 4. Verify Document
    let record = client.verify_doc(&doc_hash);
    assert!(record.is_some());
    let doc = record.unwrap();
    assert_eq!(doc.issuer, issuer);
    assert_eq!(doc.is_valid, true);

    // 5. Revoke Document
    client.revoke_doc(&issuer, &doc_hash);
    let revoked_record = client.verify_doc(&doc_hash).unwrap();
    assert_eq!(revoked_record.is_valid, false);
}

#[test]
#[should_panic(expected = "Not an authorized issuer")]
fn test_unauthorized_issuer_fails() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, FrontlineLedger);
    let client = FrontlineLedgerClient::new(&env, &contract_id);

    let unauth_issuer = Address::generate(&env);
    let doc_hash = String::from_str(&env, "fakehash1234567890");

    client.issue_doc(&unauth_issuer, &doc_hash);
}

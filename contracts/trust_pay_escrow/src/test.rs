#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Address, Env, String};

    // Setup helper environment
    fn setup_test() -> (Env, TrustPayContractClient<'static>, Address, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, TrustPayContract);
        let client = TrustPayContractClient::new(&env, &contract_id);

        let client_addr = Address::generate(&env);
        let worker_addr = Address::generate(&env);

        (env, client, client_addr, worker_addr)
    }

    // Test 1 (Happy path): The MVP transaction executes successfully end-to-end
    #[test]
    fn test_happy_path_escrow_flow() {
        let (env, contract, client_addr, worker_addr) = setup_test();
        let task_id = 101u64;
        let amount = 500_0000000i128; // 500 XLM

        // Step 1: Create Escrow
        contract.create_escrow(&task_id, &client_addr, &worker_addr, &amount);

        // Step 2: Submit Work
        let proof = String::from_str(&env, "https://figma.com/file/sample-design");
        contract.submit_work(&task_id, &worker_addr, &proof);

        // Step 3: Release Payment
        contract.release_payment(&task_id, &client_addr);

        let escrow = contract.get_escrow(&task_id);
        assert_eq!(escrow.status, EscrowStatus::Completed);
    }

    // Test 2 (Edge case): Unauthorized worker submission failure
    #[test]
    #[should_panic(expected = "Unauthorized worker")]
    fn test_unauthorized_worker_submission() {
        let (env, contract, client_addr, worker_addr) = setup_test();
        let wrong_worker = Address::generate(&env);
        let task_id = 102u64;

        contract.create_escrow(&task_id, &client_addr, &worker_addr, &100_0000000i128);

        let proof = String::from_str(&env, "https://figma.com/file/fake");
        // Should panic because wrong_worker is not worker_addr
        contract.submit_work(&task_id, &wrong_worker, &proof);
    }

    // Test 3 (Edge case): Client releases before submission failure
    #[test]
    #[should_panic(expected = "Work must be submitted before release")]
    fn test_premature_release_failure() {
        let (_env, contract, client_addr, worker_addr) = setup_test();
        let task_id = 103u64;

        contract.create_escrow(&task_id, &client_addr, &worker_addr, &100_0000000i128);

        // Try releasing before worker submits work
        contract.release_payment(&task_id, &client_addr);
    }

    // Test 4 (State verification): Verify initial Funded state storage
    #[test]
    fn test_initial_state_verification() {
        let (_env, contract, client_addr, worker_addr) = setup_test();
        let task_id = 104u64;

        contract.create_escrow(&task_id, &client_addr, &worker_addr, &250_0000000i128);

        let escrow = contract.get_escrow(&task_id);
        assert_eq!(escrow.client, client_addr);
        assert_eq!(escrow.worker, worker_addr);
        assert_eq!(escrow.status, EscrowStatus::Funded);
    }

    // Test 5 (State verification): Verify Task URL update upon submission
    #[test]
    fn test_submission_state_verification() {
        let (env, contract, client_addr, worker_addr) = setup_test();
        let task_id = 105u64;

        contract.create_escrow(&task_id, &client_addr, &worker_addr, &250_0000000i128);

        let proof_url = String::from_str(&env, "https://github.com/pr/1");
        contract.submit_work(&task_id, &worker_addr, &proof_url);

        let escrow = contract.get_escrow(&task_id);
        assert_eq!(escrow.task_url, proof_url);
        assert_eq!(escrow.status, EscrowStatus::Submitted);
    }
}
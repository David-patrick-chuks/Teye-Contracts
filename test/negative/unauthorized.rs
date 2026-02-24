use your_crate_name::errors::ContractError;
use your_crate_name::*;

#[test]
fn test_unauthorized_access() {
    let contract = Contract::new("owner".to_string());

    let result = contract.restricted_action("attacker");

    assert_eq!(result, Err(ContractError::Unauthorized));
}

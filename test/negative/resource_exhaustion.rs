use your_crate_name::errors::ContractError;
use your_crate_name::*;

#[test]
fn test_resource_exhaustion() {
    let contract = Contract::new("owner".to_string());

    let result = contract.consume_resource(50_000);

    assert_eq!(result, Err(ContractError::ResourceExhausted));
}

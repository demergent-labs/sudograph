use sudograph_tests::{
    CASES,
    LOGGING,
    utilities::agent::{
        copy_schema,
        deploy_canister,
        update_test
    }
};

#[test]
fn test_offset() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        copy_schema("canisters/graphql/src/tests/offset/test_offset_schema.graphql");
        deploy_canister();
        update_test(
            "test_offset",
            CASES,
            LOGGING
        ).await.unwrap();
    });
}
// TODO the offset and limit and order tests are so similar that they should really use generics and closures to reuse most of their code

use graphql_parser::schema::parse_schema;
use proptest::test_runner::{
    Config,
    TestRunner
};
use std::fs;
use sudograph_tests::{
    arbitraries::search::{
        search_create::get_search_create_arbitrary,
        search_read::get_search_read_arbitrary
    },
    utilities::graphql::{
        get_object_types,
        graphql_mutation,
        graphql_query
    }
};

#[test]
fn test_search() -> Result<(), Box<dyn std::error::Error>> {
    let schema_file_contents: &'static str = Box::leak(fs::read_to_string("canisters/graphql/src/test_search_schema.graphql")?.into_boxed_str());
    let graphql_ast = Box::leak(Box::new(parse_schema::<String>(&schema_file_contents)?));
    let object_types = Box::leak(Box::new(get_object_types(graphql_ast)));

    wasm_rs_async_executor::single_threaded::block_on(async {
        graphql_mutation(
            "
                mutation {
                    clear
                }
            ",
            "{}"
        ).await.unwrap();
    });

    for object_type in object_types.iter() {
        let mut runner = TestRunner::new(Config {
            cases: 10,
            max_shrink_iters: 100,
            .. Config::default()
        });

        let search_create_arbitrary = get_search_create_arbitrary(
            graphql_ast,
            object_types,
            object_type,
            None,
            2
        );

        runner.run(&search_create_arbitrary, |search_create_concrete| {
            let search_read_arbitrary = get_search_read_arbitrary(
                graphql_ast,
                object_type,
                true,
                Some(object_type.name.clone()),
                None,
                search_create_concrete.objects,
                search_create_concrete.search_info_map
            );

            let mut runner = TestRunner::new(Config {
                cases: 100,
                max_shrink_iters: 0, // TODO shrinking does not seem to be working at all
                .. Config::default()
            });

            runner.run(&search_read_arbitrary, |search_read_concrete| {
                println!("search_read_concrete.selection\n");
                println!("{:#?}", search_read_concrete.selection);

                // let result_json = tokio::runtime::Runtime::new()?.block_on(async {
                //     return graphql_query(
                //         &format!(
                //             "query {{
                //                 {selection}
                //             }}",
                //             selection = search_read_concrete.selection
                //         ),
                //         "{}"
                //     ).await;
                // }).unwrap();

                let result_json = wasm_rs_async_executor::single_threaded::block_on(async {
                    return graphql_query(
                        &format!(
                            "query {{
                                {selection}
                            }}",
                            selection = search_read_concrete.selection
                        ),
                        "{}"
                    ).await.unwrap();
                });

                let query_name = format!(
                    "read{object_type_name}",
                    object_type_name = object_type.name
                );

                let expected_value = serde_json::json!({
                    "data": {
                        query_name: search_read_concrete.expected_value
                    }
                });

                println!("result_json\n");
                println!("{:#?}", result_json);

                println!("expected_value\n");
                println!("{:#?}", expected_value);

                assert_eq!(
                    result_json,
                    expected_value
                );

                return Ok(());
            }).unwrap();

            wasm_rs_async_executor::single_threaded::block_on(async {
                graphql_mutation(
                    "
                        mutation {
                            clear
                        }
                    ",
                    "{}"
                ).await.unwrap();
            });

            println!("Test complete");
            println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");

            return Ok(());
        })?;
    }
    
    return Ok(());
}
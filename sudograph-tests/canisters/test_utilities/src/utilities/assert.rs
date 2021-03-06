use crate::arbitraries::queries::queries::{
    ArbitraryMutationInfo,
    ArbitraryQueryInfo,
    InputInfo,
    QueriesArbitrary
};

pub fn assert_correct_result(
    result_json: &serde_json::Value,
    selection_name: &str,
    input_infos: &Vec<InputInfo>
) -> Result<bool, Box<dyn std::error::Error>> {
    let data_option = result_json.as_object().ok_or("None")?.get("data");
    let errors_option = result_json.as_object().ok_or("None")?.get("errors");
    
    match (data_option, errors_option) {
        (Some(data), None) => {
            let selection = data.as_object().ok_or("None")?.get(selection_name).ok_or("None")?;
            let results = selection.as_array().ok_or("None")?;
        
            return Ok(results.iter().all(|result| {
                return input_infos.iter().all(|input_value| {
                    // TODO figure out how to get rid of the unwrap here...seems using ? in closures isn't really figured out
                    let result_value = result.as_object().ok_or("None").unwrap().get(&input_value.field_name).unwrap();
                    let expected_value = &input_value.expected_value;

                    // ic_cdk::println!("result_value: {:#?}", result_value);
                    // ic_cdk::println!("expected_value: {:#?}", expected_value);

                    return serde_json_values_are_equal(
                        result_value,
                        expected_value
                    );
                });
            }));
        },
        (Some(data), Some(errors)) => {
            let errors = errors.as_array().ok_or("None")?;

            return Ok(errors.iter().all(|result| {
                return input_infos.iter().any(|input_value| {
                    // TODO figure out how to get rid of the unwrap here...seems using ? in closures isn't really figured out
                    let result_message = result.as_object().ok_or("None").unwrap().get("message").unwrap();
                    let expected_message = &input_value.expected_value;

                    return result_message == expected_message;
                });
            }));
        },
        (None, Some(errors)) => {
            return Ok(false);
        },
        _ => {
            return Ok(false);
        }
    }
}

// TODO I would love to get rid of this function if possible
// TODO It should be possible to get rid of once this is resolved: https://github.com/async-graphql/async-graphql/issues/565
fn serde_json_values_are_equal(
    value_1: &serde_json::Value,
    value_2: &serde_json::Value
) -> bool {
    match value_1 {
        serde_json::Value::Array(value_1_array) => {
            if value_1_array.len() == 0 {
                if value_2.is_array() && value_2.as_array().unwrap().len() == 0 {
                    return true;
                }
                else {
                    return false;
                }
            }

            return value_1_array.iter().enumerate().all(|(value_1_index, value_1_item)| {
                let value_2_item = match value_2 {
                    serde_json::Value::Array(value_2_array) => value_2_array.get(value_1_index).unwrap(),
                    _ => panic!("")
                };

                match value_1_item {
                    serde_json::Value::Number(value_1_item_number) => {
                        let value_2_item_number = match value_2_item {
                            serde_json::Value::Number(value_2_item_number) => value_2_item_number,
                            _ => panic!("")
                        };

                        // TODO this is really bad
                        return value_1_item_number.as_f64().unwrap() == value_2_item_number.as_u64().unwrap() as f64;
                    },
                    _ => {
                        return serde_json_values_are_equal(
                            value_1_item,
                            value_2_item
                        );
                    }
                };
            });
        },
        serde_json::Value::Object(value_1_object) => {
            // TODO what if the object has no fields? Is that even possible? Look at array above

            return value_1_object.iter().all(|(value_1_object_key, value_1_object_value)| {
                let value_2_object_value = match value_2 {
                    serde_json::Value::Object(value_2_object) => value_2_object.get(value_1_object_key).unwrap(),
                    _ => panic!("")
                };

                return serde_json_values_are_equal(
                    value_1_object_value,
                    value_2_object_value
                );
            });
        },
        _ => {
            return value_1 == value_2;
        }
    };
}

pub fn convert_arbitrary_mutation_info_into_mutation(arbitrary_mutation_info: &ArbitraryMutationInfo) -> (String, String) {
    let mutation = format!(
        "mutation ($input: {input_variable_type}) {{
            {mutation_name}(input: $input){selection}
        }}",
        input_variable_type = arbitrary_mutation_info.input_variable_type,
        mutation_name = arbitrary_mutation_info.mutation_name,
        selection = arbitrary_mutation_info.selection
    );
    
    let variables = serde_json::json!({
        "input": arbitrary_mutation_info.input_value
    }).to_string();

    return (mutation, variables);
}

pub fn convert_arbitrary_query_info_into_query(arbitrary_query_info: &ArbitraryQueryInfo) -> (String, String) {
    let mutation = format!(
        "query ($search: {search_variable_type}) {{
            {query_name}(search: $search){selection}
        }}",
        search_variable_type = arbitrary_query_info.search_variable_type,
        query_name = arbitrary_query_info.query_name,
        selection = arbitrary_query_info.selection
    );
    
    let variables = serde_json::json!({
        "search": arbitrary_query_info.search_value
    }).to_string();

    return (mutation, variables);
}

// TODO maybe we should check if errors.is_some instead of checking if data is null
pub fn assert_equal_disconnect(
    result: &serde_json::value::Value,
    expected: &serde_json::value::Value,
    logging: bool
) -> bool {
    if result.get("data").is_some() && result.get("data").unwrap().is_null() {
        let result_errors: Vec<String> = result.get("errors").unwrap().as_array().unwrap().iter().map(|error| {
            return error.get("message").unwrap().to_string();
        }).collect();

        let expected_errors: Vec<String> = expected.get("errors").unwrap().as_array().unwrap().iter().map(|error| {
            return error.get("message").unwrap().to_string();
        }).collect();

        if logging == true {
            ic_cdk::println!("result_errors {:#?}", result_errors);
            ic_cdk::println!("expected_errors {:#?}", expected_errors);
        }

        return result_errors == expected_errors;
    }
    else {
        return result == expected;
    }
}
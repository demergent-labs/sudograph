// TODO clean up this whole file, it's really messy and confusing

use chrono::prelude::{
    DateTime,
    Utc
};
use crate::{
    convert_field_value_store_to_json_string,
    FieldValue,
    FieldValueScalar,
    FieldValueStore,
    FieldValuesStore,
    FieldType,
    FieldTypesStore,
    get_object_type,
    JSONString,
    ObjectTypeStore,
    ReadInput,
    ReadInputOperation,
    SelectionSet,
    SudodbError
};
use std::error::Error;

const ERROR_PREFIX: &str = "Sudodb::read::error - ";

// TODO prefix all errors with Sudodb::...or something like that
// TODO let's try to make this the simplest experience ever
pub fn read(
    object_type_store: &ObjectTypeStore,
    object_type_name: &str,
    inputs: &Vec<ReadInput>,
    selection_set: &SelectionSet
) -> Result<Vec<JSONString>, Box<dyn Error>> {
    let object_type = get_object_type(
        object_type_store,
        String::from(object_type_name)
    )?;

    let field_value_stores = find_field_value_stores_for_inputs(
        &object_type.field_values_store,
        &object_type.field_types_store,
        &inputs
    )?;

    let field_value_store_strings = field_value_stores.iter().map(|field_value_store| {
        return convert_field_value_store_to_json_string(
            object_type_store,
            field_value_store,
            selection_set
        );
    }).collect();

    return Ok(field_value_store_strings);
}

fn find_field_value_stores_for_inputs(
    field_values_store: &FieldValuesStore,
    field_types_store: &FieldTypesStore,
    inputs: &Vec<ReadInput>
) -> Result<Vec<FieldValueStore>, Box<dyn Error>> {
    // TODO I believe the result in the fold here needs to be mutable for efficiency...not sure, but perhaps
    let field_value_stores = field_values_store.values().try_fold(vec![], |mut result, field_value_store| {
        let inputs_match: bool = field_value_store_matches_inputs(
            field_value_store,
            field_types_store,
            &inputs,
            false
        )?;

        if inputs_match == true {
            result.push(field_value_store.clone());

            return Ok(result);
        }
        else {
            return Ok(result);
        }
    });

    return field_value_stores;
}

fn field_value_store_matches_inputs(
    field_value_store: &FieldValueStore,
    field_types_store: &FieldTypesStore,
    inputs: &Vec<ReadInput>,
    or: bool
) -> Result<bool, Box<dyn Error>> {
    return inputs.iter().try_fold(if or == true { false } else { true }, |result, input| {
        if
            result == false &&
            or == false
        {
            return Ok(false);
        }

        if
            result == true &&
            or == true
        {
            return Ok(true);
        }

        if input.field_name == "and" {
            return field_value_store_matches_inputs(
                field_value_store,
                field_types_store,
                &input.and,
                false
            );
        }

        if input.field_name == "or" {
            return field_value_store_matches_inputs(
                field_value_store,
                field_types_store,
                &input.or,
                true
            );
        }

        let field_type_option = field_types_store.get(&input.field_name);
        let field_value_option = field_value_store.get(&input.field_name);    

        // TODO what if I split based on the scalar types here?
        if let (Some(field_type), Some(field_value)) = (field_type_option, field_value_option) {
            match field_type {
                FieldType::RelationMany(field_type_relation_info) => {
                    return field_value_relation_many_matches_input(
                        field_value,
                        input
                    );
                },
                FieldType::RelationOne(field_type_relation_info) => {
                    return field_value_relation_one_matches_input(
                        field_value,
                        input
                    );
                },
                _ => {
                    return field_value_matches_input(
                        field_value,
                        input
                    );
                }
            };
        }
        else {
            // TODO Should I get more specific about what exact information was not found? the field_type or field_value?
            return Err(Box::new(SudodbError {
                message: format!(
                    "Information not found for field {field_name}",
                    field_name = input.field_name
                )
            }));
        }
    });
}

// TODO it really should not be that hard to have cross-relational filtering on all fields
// TODO use recursion to get this done
fn field_value_relation_many_matches_input(
    field_value_relation_many: &FieldValue,
    input: &ReadInput
) -> Result<bool, Box<dyn Error>> {
    match field_value_relation_many {
        FieldValue::RelationMany(field_value_relation_many_option) => {
            match &input.field_value {
                // TODO we are only supporting filtering by id right now, this will look much different once we
                // TODO add generic cross-relational filtering
                // TODO do not implement cross-relational filtering until reading is cleaned up a lot
                FieldValue::Scalar(input_field_value_scalar_option) => {
                    // if let (
                    //     Some(field_value_relation_many),
                    //     Some(input_field_value_relation_many)
                    // ) = (
                    //     field_value_relation_many_option,
                    //     input_field_value_relation_many_option
                    // ) {
                    //     return field_value_relation_many.
                    // }
                    // else {

                    // }

                    if let Some(field_value_relation_many) = field_value_relation_many_option {
                        if let Some(input_field_value_scalar) = input_field_value_scalar_option {
                            
                            match input_field_value_scalar {
                                FieldValueScalar::String(field_value_scalar_string) => {
                                    return Ok(field_value_relation_many.relation_primary_keys.contains(field_value_scalar_string));
                                },
                                _ => {
                                    return Ok(false);
                                }
                                // FieldValue::Scalar(test)
                            };
                        }
                        else {
                            return Ok(false);
                        }
                    }
                    else {
                        if let Some(input_field_value_scalar) = input_field_value_scalar_option {
                            return Ok(false);
                        }
                        else {
                            return Ok(true);
                        }
                    }
                },
                _ => {
                    // TODO return an error
                    return Ok(false);
                }
            };

            // match &input.input_operation {
            //     ReadInputOperation::Equals => {
            //         match &input.field_value {
            //             FieldValue::RelationMany(field_value_relation_many_option) => {
            //                 // return field_value_relation_many_option == None;

            //                 // if let Some(field_value_relation_many_option)
            //             },
            //             _ => {

            //             }
            //         };

            //         // return 
            //     },
            //     _ => {
            //         // TODO do something
            //     }
            // };
        },
        _ => {
            // TODO I think this should panic?
            return Ok(false);
        }
    };

    // return Ok(true);
}

fn field_value_relation_one_matches_input(
    field_value_relation_one: &FieldValue,
    input: &ReadInput
) -> Result<bool, Box<dyn Error>> {
    match field_value_relation_one {
        FieldValue::RelationOne(field_value_relation_one_option) => {
            match &input.field_value {
                // TODO we are only supporting filtering by id right now, this will look much different once we
                // TODO add generic cross-relational filtering
                // TODO do not implement cross-relational filtering until reading is cleaned up a lot
                FieldValue::Scalar(input_field_value_scalar_option) => {
                    // if let (
                    //     Some(field_value_relation_many),
                    //     Some(input_field_value_relation_many)
                    // ) = (
                    //     field_value_relation_many_option,
                    //     input_field_value_relation_many_option
                    // ) {
                    //     return field_value_relation_many.
                    // }
                    // else {

                    // }

                    if let Some(field_value_relation_one) = field_value_relation_one_option {
                        if let Some(input_field_value_scalar) = input_field_value_scalar_option {
                            
                            match input_field_value_scalar {
                                FieldValueScalar::String(field_value_scalar_string) => {
                                    return Ok(field_value_relation_one.relation_primary_key == String::from(field_value_scalar_string));
                                },
                                _ => {
                                    return Ok(false);
                                }
                                // FieldValue::Scalar(test)
                            };
                        }
                        else {
                            return Ok(false);
                        }
                    }
                    else {
                        if let Some(input_field_value_scalar) = input_field_value_scalar_option {
                            return Ok(false);
                        }
                        else {
                            return Ok(true);
                        }
                    }
                },
                _ => {
                    // TODO return an error
                    return Ok(false);
                }
            };

            // match &input.input_operation {
            //     ReadInputOperation::Equals => {
            //         match &input.field_value {
            //             FieldValue::RelationMany(field_value_relation_many_option) => {
            //                 // return field_value_relation_many_option == None;

            //                 // if let Some(field_value_relation_many_option)
            //             },
            //             _ => {

            //             }
            //         };

            //         // return 
            //     },
            //     _ => {
            //         // TODO do something
            //     }
            // };
        },
        _ => {
            // TODO I think this should panic?
            return Ok(false);
        }
    };

    // return Ok(true);
}

// TODO try to make this much less verbose if possible
// TODO consider if we even need the result here, now that we are not parsing anymore
fn field_value_matches_input(
    field_value: &FieldValue,
    input: &ReadInput
) -> Result<bool, Box<dyn Error>> {

    // TODO this is super verbose, so if we can simplify it
    match field_value {
        FieldValue::Scalar(field_value_scalar_option) => {
            match field_value_scalar_option {
                Some(field_value_scalar) => {
                    match field_value_scalar {
                        FieldValueScalar::Boolean(field_value_scalar_boolean) => {
                            return field_value_matches_input_for_type_boolean(
                                field_value_scalar_boolean,
                                input
                            );
                        },
                        FieldValueScalar::Date(field_value_scalar_date) => {
                            return field_value_matches_input_for_type_date(
                                field_value_scalar_date,
                                input
                            );
                        },
                        FieldValueScalar::Float(field_value_scalar_float) => {
                            return field_value_matches_input_for_type_float(
                                field_value_scalar_float,
                                input
                            );
                        },
                        FieldValueScalar::Int(field_value_scalar_boolean) => {
                            return field_value_matches_input_for_type_int(
                                field_value_scalar_boolean,
                                input
                            );
                        },
                        FieldValueScalar::String(field_value_scalar_boolean) => {
                            return field_value_matches_input_for_type_string(
                                field_value_scalar_boolean,
                                input
                            );
                        }
                    };
                },
                None => {
                    // return Ok(false);
                    match &input.input_operation {
                        ReadInputOperation::Equals => {
                            match &input.field_value {
                                FieldValue::Scalar(input_field_value_scalar_option) => {
                                    match input_field_value_scalar_option {
                                        Some(_) => {
                                            return Ok(false);
                                        },
                                        None => {
                                            // TODO this is too liberal...
                                            // TODO for example, if a date is null and you asked for all dates greater than null, then this will return true
                                            // TODO actually...maybe that is correct?
                                            return Ok(true);
                                        }
                                    };
                                },
                                FieldValue::RelationMany(_) => {
                                    // TODO we might even want to panic here
                                    return Ok(false);
                                },
                                FieldValue::RelationOne(_) => {
                                    // TODO we might even want to panic here
                                    return Ok(false);
                                }
                            };
                        },
                        _ => {
                            return Ok(false);
                        }
                    };
                }
            }
        },
        FieldValue::RelationMany(field_value_relation_many_option) => {
            // TODO we might even want to panic here
            return Ok(false); // TODO relation filtering not yet implemented
        },
        FieldValue::RelationOne(field_value_relation_one_option) => {
            // TODO we might even want to panic here
            return Ok(false); // TODO relation filtering not yet implemented
        }
    }
}

fn field_value_matches_input_for_type_boolean(
    field_value_scalar_boolean: &bool,
    input: &ReadInput
) -> Result<bool, Box<dyn Error>> {
    match &input.field_value {
        FieldValue::Scalar(input_field_value_scalar_option) => {
            match input_field_value_scalar_option {
                Some(input_field_value_scalar) => {
                    match input_field_value_scalar {
                        FieldValueScalar::Boolean(input_field_value_scalar_boolean) => {
                            match input.input_operation {
                                ReadInputOperation::Contains => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation contains is not implemented for field type boolean",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::EndsWith => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation ends with is not implemented for field type boolean",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::Equals => {
                                    return Ok(field_value_scalar_boolean == input_field_value_scalar_boolean);
                                },
                                ReadInputOperation::GreaterThan => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation in is not implemented for field type boolean",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::GreaterThanOrEqualTo => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation in is not implemented for field type boolean",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::In => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation in is not implemented for field type boolean",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::LessThan => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation in is not implemented for field type boolean",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::LessThanOrEqualTo => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation in is not implemented for field type boolean",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::StartsWith => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation starts with is not implemented for field type date",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                }
                            };
                        },
                        _ => {
                            return Ok(false);
                        }
                    };
                },
                None => {
                    return Ok(false);
                }
            };
        },
        FieldValue::RelationMany(_) => {
            // TODO we might even want to panic here
            return Ok(false);
        },
        FieldValue::RelationOne(_) => {
            // TODO we might even want to panic here
            return Ok(false);
        }
    }
}

// TODO the strings here need to be converted into dates for comparison
fn field_value_matches_input_for_type_date(
    field_value_scalar_date_string: &String,
    input: &ReadInput
) -> Result<bool, Box<dyn Error>> {
    let field_value_scalar_date = field_value_scalar_date_string.parse::<DateTime<Utc>>()?;

    match &input.field_value {
        FieldValue::Scalar(input_field_value_scalar_option) => {
            match input_field_value_scalar_option {
                Some(input_field_value_scalar) => {
                    match input_field_value_scalar {
                        FieldValueScalar::Date(input_field_value_scalar_date_string) => {
                            let input_field_value_scalar_date = input_field_value_scalar_date_string.parse::<DateTime<Utc>>()?;

                            match input.input_operation {
                                ReadInputOperation::Contains => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation contains is not implemented for field type date",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::EndsWith => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation ends with is not implemented for field type date",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::Equals => {
                                    return Ok(field_value_scalar_date == input_field_value_scalar_date);
                                },
                                ReadInputOperation::GreaterThan => {
                                    return Ok(field_value_scalar_date > input_field_value_scalar_date);
                                },
                                ReadInputOperation::GreaterThanOrEqualTo => {
                                    return Ok(field_value_scalar_date >= input_field_value_scalar_date);
                                },
                                ReadInputOperation::In => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation in is not implemented for field type date",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::LessThan => {
                                    return Ok(field_value_scalar_date < input_field_value_scalar_date);
                                },
                                ReadInputOperation::LessThanOrEqualTo => {
                                    return Ok(field_value_scalar_date <= input_field_value_scalar_date);
                                },
                                ReadInputOperation::StartsWith => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation starts with is not implemented for field type date",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                }
                            };
                        },
                        _ => {
                            return Ok(false);
                        }
                    };
                },
                None => {
                    return Ok(false);
                }
            };
        },
        FieldValue::RelationMany(_) => {
            // TODO we might even want to panic here
            return Ok(false);
        },
        FieldValue::RelationOne(_) => {
            // TODO we might even want to panic here
            return Ok(false);
        }
    }
}

fn field_value_matches_input_for_type_float(
    field_value_scalar_float: &f32,
    input: &ReadInput
) -> Result<bool, Box<dyn Error>> {
    match &input.field_value {
        FieldValue::Scalar(input_field_value_scalar_option) => {
            match input_field_value_scalar_option {
                Some(input_field_value_scalar) => {
                    match input_field_value_scalar {
                        FieldValueScalar::Float(input_field_value_scalar_float) => {
                            match input.input_operation {
                                ReadInputOperation::Contains => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation contains is not implemented for field type float",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::EndsWith => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation ends with is not implemented for field type float",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::Equals => {
                                    return Ok(field_value_scalar_float == input_field_value_scalar_float);
                                },
                                ReadInputOperation::GreaterThan => {
                                    return Ok(field_value_scalar_float > input_field_value_scalar_float);
                                },
                                ReadInputOperation::GreaterThanOrEqualTo => {
                                    return Ok(field_value_scalar_float >= input_field_value_scalar_float);
                                },
                                ReadInputOperation::In => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation in is not implemented for field type float",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::LessThan => {
                                    return Ok(field_value_scalar_float < input_field_value_scalar_float);
                                },
                                ReadInputOperation::LessThanOrEqualTo => {
                                    return Ok(field_value_scalar_float <= input_field_value_scalar_float);
                                },
                                ReadInputOperation::StartsWith => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation starts with is not implemented for field type float",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                }
                            };
                        },
                        _ => {
                            return Ok(false);
                        }
                    };
                },
                None => {
                    return Ok(false);
                }
            };
        },
        FieldValue::RelationMany(_) => {
            // TODO we might even want to panic here
            return Ok(false);
        },
        FieldValue::RelationOne(_) => {
            // TODO we might even want to panic here
            return Ok(false);
        }
    }
}

fn field_value_matches_input_for_type_int(
    field_value_scalar_int: &i32,
    input: &ReadInput
) -> Result<bool, Box<dyn Error>> {
    match &input.field_value {
        FieldValue::Scalar(input_field_value_scalar_option) => {
            match input_field_value_scalar_option {
                Some(input_field_value_scalar) => {
                    match input_field_value_scalar {
                        FieldValueScalar::Int(input_field_value_scalar_int) => {
                            match input.input_operation {
                                ReadInputOperation::Contains => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation contains is not implemented for field type int",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::EndsWith => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation ends with is not implemented for field type int",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::Equals => {
                                    return Ok(field_value_scalar_int == input_field_value_scalar_int);
                                },
                                ReadInputOperation::GreaterThan => {
                                    return Ok(field_value_scalar_int > input_field_value_scalar_int);
                                },
                                ReadInputOperation::GreaterThanOrEqualTo => {
                                    return Ok(field_value_scalar_int >= input_field_value_scalar_int);
                                },
                                ReadInputOperation::In => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation in is not implemented for field type int",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::LessThan => {
                                    return Ok(field_value_scalar_int < input_field_value_scalar_int);
                                },
                                ReadInputOperation::LessThanOrEqualTo => {
                                    return Ok(field_value_scalar_int <= input_field_value_scalar_int);
                                },
                                ReadInputOperation::StartsWith => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation starts with is not implemented for field type int",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                }
                            };
                        },
                        _ => {
                            return Ok(false);
                        }
                    };
                },
                None => {
                    return Ok(false);
                }
            };
        },
        FieldValue::RelationMany(_) => {
            // TODO we might even want to panic here
            return Ok(false);
        },
        FieldValue::RelationOne(_) => {
            // TODO we might even want to panic here
            return Ok(false);
        }
    }
}

fn field_value_matches_input_for_type_string(
    field_value_scalar_string: &String,
    input: &ReadInput
) -> Result<bool, Box<dyn Error>> {
    match &input.field_value {
        FieldValue::Scalar(input_field_value_scalar_option) => {
            match input_field_value_scalar_option {
                Some(input_field_value_scalar) => {
                    match input_field_value_scalar {
                        FieldValueScalar::String(input_field_value_scalar_string) => {
                            match input.input_operation {
                                ReadInputOperation::Contains => {
                                    return Ok(field_value_scalar_string.contains(input_field_value_scalar_string));
                                },
                                ReadInputOperation::EndsWith => {
                                    return Ok(field_value_scalar_string.ends_with(input_field_value_scalar_string));
                                },
                                ReadInputOperation::Equals => {
                                    return Ok(field_value_scalar_string == input_field_value_scalar_string);
                                },
                                ReadInputOperation::GreaterThan => {
                                    return Ok(field_value_scalar_string > input_field_value_scalar_string);
                                },
                                ReadInputOperation::GreaterThanOrEqualTo => {
                                    return Ok(field_value_scalar_string >= input_field_value_scalar_string);
                                },
                                ReadInputOperation::In => {
                                    return Err(Box::new(SudodbError {
                                        message: format!(
                                            "{error_prefix}read input operation in is not implemented for field type string",
                                            error_prefix = ERROR_PREFIX
                                        )
                                    }));
                                },
                                ReadInputOperation::LessThan => {
                                    return Ok(field_value_scalar_string < input_field_value_scalar_string);
                                },
                                ReadInputOperation::LessThanOrEqualTo => {
                                    return Ok(field_value_scalar_string <= input_field_value_scalar_string);
                                },
                                ReadInputOperation::StartsWith => {
                                    return Ok(field_value_scalar_string.starts_with(input_field_value_scalar_string));
                                }
                            };
                        },
                        _ => {
                            return Ok(false);
                        }
                    };
                },
                None => {
                    return Ok(false);
                }
            };
        },
        FieldValue::RelationMany(_) => {
            // TODO we might even want to panic here
            return Ok(false);
        },
        FieldValue::RelationOne(_) => {
            // TODO we might even want to panic here
            return Ok(false);
        }
    }
}
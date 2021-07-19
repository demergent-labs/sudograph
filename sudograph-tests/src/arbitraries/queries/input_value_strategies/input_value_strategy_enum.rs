use crate::{
    arbitraries::queries::{
        input_value_strategies::input_value_strategy_nullable::get_input_value_strategy_nullable,
        queries::{
            InputValue,
            MutationType
        }
    },
    utilities::graphql::{
        get_enum_type_from_field,
        get_graphql_type_name,
        is_graphql_type_nullable
    }
};
use graphql_parser::schema::{
    Document,
    Field
};
use proptest::{
    strategy::{
        BoxedStrategy,
        Strategy
    }
};

pub fn get_input_value_strategy_enum(
    graphql_ast: &'static Document<String>,
    field: &'static Field<String>,
    mutation_type: MutationType
) -> BoxedStrategy<InputValue> {
    let nullable = is_graphql_type_nullable(&field.field_type);
    
    let enum_type = get_enum_type_from_field(
        &graphql_ast,
        &field
    ).unwrap();

    let enum_values_len = enum_type.values.len();

    let strategy = (0..enum_values_len - 1).prop_map(move |index| {
        let field_type = get_graphql_type_name(&field.field_type);

        let input_value = serde_json::json!(enum_type.clone().values.get(index).unwrap().name.clone());
        let selection_value = input_value.clone();

        return InputValue {
            field: Some(field.clone()),
            field_name: field.name.to_string(),
            field_type,
            selection: field.name.to_string(),
            nullable,
            input_value,
            selection_value
        };
    }).boxed();

    if nullable == true {
        return get_input_value_strategy_nullable(
            field,
            strategy,
            false,
            false,
            mutation_type,
            serde_json::json!(null)
        );
    }
    else {
        return strategy;
    }
}
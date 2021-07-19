use crate::{
    arbitraries::queries::{
        input_value_strategies::input_value_strategy_nullable::get_input_value_strategy_nullable,
        queries::{
            InputValue,
            MutationType
        }
    },
    utilities::graphql::{
        get_graphql_type_name,
        is_graphql_type_nullable
    }
};
use graphql_parser::schema::Field;
use proptest::{
    prelude::Just,
    strategy::{
        BoxedStrategy,
        Strategy
    }
};

pub fn get_input_value_strategy_date(
    field: &'static Field<String>,
    mutation_type: MutationType
) -> BoxedStrategy<InputValue> {
    let nullable = is_graphql_type_nullable(&field.field_type);
    let strategy = Just(chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true)).prop_map(move |datetime| {
        let field_type = get_graphql_type_name(&field.field_type);
        
        let input_value = serde_json::json!(datetime);
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
pub mod arbitraries {
    pub mod queries {
        pub mod queries;
        pub mod mutation_create;
        pub mod mutation_update;
        pub mod mutation_update_disconnect {
            pub mod mutation_update_disconnect;
            pub mod strategies {
                pub mod strategies;
                pub mod connect;
                pub mod disconnect;
                pub mod check_disconnected_relation;
            }
        }
        pub mod mutation_delete {
            pub mod mutation_delete;
        }
        pub mod input_info_strategies {
            pub mod input_info_strategies;
            pub mod input_info_strategy_blob;
            pub mod input_info_strategy_boolean;
            pub mod input_info_strategy_date;
            pub mod input_info_strategy_enum;
            pub mod input_info_strategy_float;
            pub mod input_info_strategy_id;
            pub mod input_info_strategy_int;
            pub mod input_info_strategy_json;
            pub mod input_info_strategy_nullable;
            pub mod input_info_strategy_relation_many;
            pub mod input_info_strategy_relation_one;
            pub mod input_info_strategy_string;
        }
    }
    pub mod limit {
        pub mod limit_create;
        pub mod limit_read;
    }
    pub mod offset {
        pub mod offset_create;
        pub mod offset_read;
    }
    pub mod order {
        pub mod order_create;
        pub mod order_read;
        pub mod order_input;
    }
    pub mod search {
        pub mod search_create;
        pub mod search_read;
        pub mod search_input;
    }
}
pub mod utilities {
    pub mod assert;
    pub mod graphql;
}
// @generated automatically by Diesel CLI.

diesel::table! {
    records (id) {
        id -> Nullable<Integer>,
        money_type -> Text,
        amount -> Float,
        expense -> Nullable<Text>,
        time -> Text,
    }
}

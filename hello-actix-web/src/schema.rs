// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 64]
        name -> Varchar,
    }
}

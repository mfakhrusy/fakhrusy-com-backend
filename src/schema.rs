table! {
    users (id) {
        id -> Int4,
        email -> Text,
        hashed_password -> Text,
        salt -> Text,
        full_name -> Nullable<Text>,
    }
}

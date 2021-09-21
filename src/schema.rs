table! {
    users (id) {
        id -> Uuid,
        email -> Text,
        password -> Text,
        permissions -> Array<Text>,
    }
}

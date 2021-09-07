table! {
    thermostat_status (id) {
        id -> Int4,
        status -> Bool,
        timestamp -> Timestamp,
    }
}

table! {
    todos (id) {
        id -> Integer,
        title -> Text,
    }
}

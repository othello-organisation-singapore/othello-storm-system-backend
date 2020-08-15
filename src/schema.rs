table! {
    joueurs (id) {
        id -> Int4,
        timestamp -> Timestamp,
        content -> Json,
    }
}

table! {
    tournaments (id) {
        id -> Int4,
        name -> Varchar,
        country -> Varchar,
        creator -> Varchar,
        joueurs_id -> Int4,
        meta_data -> Json,
    }
}

table! {
    users (username) {
        username -> Varchar,
        display_name -> Varchar,
        hashed_password -> Varchar,
        role -> Varchar,
    }
}

joinable!(tournaments -> joueurs (joueurs_id));
joinable!(tournaments -> users (creator));

allow_tables_to_appear_in_same_query!(
    joueurs,
    tournaments,
    users,
);

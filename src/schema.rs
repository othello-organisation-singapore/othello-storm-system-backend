table! {
    tournaments (id) {
        id -> Int4,
        name -> Varchar,
        tournament_type -> Varchar,
        country -> Varchar,
        creator -> Varchar,
        joueurs -> Json,
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

joinable!(tournaments -> users (creator));

allow_tables_to_appear_in_same_query!(
    tournaments,
    users,
);

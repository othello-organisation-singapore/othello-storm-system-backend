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
    tournaments_admin (id) {
        id -> Int4,
        tournament_id -> Int4,
        admin_username -> Varchar,
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
joinable!(tournaments_admin -> tournaments (tournament_id));
joinable!(tournaments_admin -> users (admin_username));

allow_tables_to_appear_in_same_query!(
    tournaments,
    tournaments_admin,
    users,
);

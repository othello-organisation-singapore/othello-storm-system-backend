table! {
    players (id) {
        id -> Int4,
        tournament_id -> Int4,
        joueurs_id -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        country -> Varchar,
        rating -> Int4,
        meta_data -> Json,
    }
}

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

joinable!(players -> tournaments (tournament_id));
joinable!(tournaments -> users (creator));
joinable!(tournaments_admin -> tournaments (tournament_id));
joinable!(tournaments_admin -> users (admin_username));

allow_tables_to_appear_in_same_query!(
    players,
    tournaments,
    tournaments_admin,
    users,
);

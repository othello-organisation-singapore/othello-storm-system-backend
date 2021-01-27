use serde_json::Value;

pub struct GameMatch {
    pub round_id: i32,
    pub black_player_id: i32,
    pub white_player_id: i32,
    pub black_score: i32,
    pub white_score: i32,
    pub meta_data: Value,
}

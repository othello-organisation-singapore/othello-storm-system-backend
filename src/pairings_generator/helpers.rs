use crate::properties::PlayerColor;
use crate::tournament_manager::IResultKeeper;

pub fn get_player_1_color(
    player_1_id: &i32,
    player_2_id: &i32,
    past_results: &Box<dyn IResultKeeper>,
) -> PlayerColor {
    let player_1_black_count = past_results.get_color_count(player_1_id, PlayerColor::Black);
    let player_1_white_count = past_results.get_color_count(player_1_id, PlayerColor::White);
    let player_2_black_count = past_results.get_color_count(player_2_id, PlayerColor::Black);
    let player_2_white_count = past_results.get_color_count(player_2_id, PlayerColor::White);

    if player_1_black_count + player_2_white_count > player_2_black_count + player_1_white_count {
        return PlayerColor::White;
    }
    PlayerColor::Black
}

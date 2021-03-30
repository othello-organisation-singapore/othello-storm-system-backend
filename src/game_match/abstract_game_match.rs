use std::collections::HashMap;
use std::fmt::Debug;

use crate::database_models::MatchRowModel;
use crate::properties::PlayerColor;

pub trait ClonableIGameMatch {
    fn clone_box(&self) -> Box<dyn IGameMatch>;
}

impl<T> ClonableIGameMatch for T
where
    T: 'static + IGameMatch + Clone,
{
    fn clone_box(&self) -> Box<dyn IGameMatch> {
        Box::new(self.clone())
    }
}

pub trait IGameMatch: ClonableIGameMatch + Debug {
    fn is_player_playing(&self, player_id: &i32) -> bool;
    fn get_player_color(&self, player_id: &i32) -> Option<PlayerColor>;
    fn get_players_id(&self) -> (Option<i32>, Option<i32>);
    fn get_opponent_id(&self, player_id: &i32) -> Option<i32>;
    fn calculate_major_score(&self, player_id: &i32) -> f64;
    fn calculate_minor_score(
        &self,
        player_id: &i32,
        major_scores_by_player_ids: &HashMap<i32, f64>,
        brightwell_constant: &f64,
    ) -> f64;
    fn extract_data(&self) -> MatchRowModel;
}

impl Clone for Box<dyn IGameMatch> {
    fn clone(&self) -> Box<dyn IGameMatch> {
        self.clone_box()
    }
}

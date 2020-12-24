use diesel::PgConnection;
use serde_json::{Map, Value};

use crate::database_models::PlayerRowModel;
use crate::errors::ErrorType;
use crate::tournament_manager::Player;

use super::MetaGenerator;

pub struct PlayerMetaGenerator {
    player: PlayerRowModel
}

impl PlayerMetaGenerator {
    pub fn from_player_model(player: PlayerRowModel) -> PlayerMetaGenerator {
        PlayerMetaGenerator { player }
    }

    pub fn from_player(
        player: Player,
        tournament_id: &i32,
        connection: &PgConnection,
    ) -> Result<PlayerMetaGenerator, ErrorType> {
        let player_model = PlayerRowModel::get_from_joueurs_id(
            &player.joueurs_id,
            tournament_id,
            connection,
        )?;
        Ok(PlayerMetaGenerator::from_player_model(player_model))
    }
}

impl MetaGenerator for PlayerMetaGenerator {
    fn generate_meta(&self) -> Map<String, Value> {
        let mut meta = Map::new();
        meta.insert(String::from("id"), Value::from(self.player.id.to_string()));
        meta.insert(
            String::from("joueurs_id"),
            Value::from(self.player.joueurs_id.clone()),
        );
        meta.insert(
            String::from("first_name"),
            Value::from(self.player.first_name.clone()),
        );
        meta.insert(
            String::from("last_name"),
            Value::from(self.player.last_name.clone()),
        );
        meta.insert(
            String::from("country"),
            Value::from(self.player.country.clone()),
        );
        meta.insert(
            String::from("rating"), Value::from(self.player.rating.to_string()),
        );
        meta
    }
}


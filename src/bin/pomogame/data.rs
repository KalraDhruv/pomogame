use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::path::Path;
use crate::player::Player;

const PLAYER_DATA_FILE: &str = "player_data.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerConfig {
    pub player: Player,
}

impl PlayerConfig {
    // If the file doesn't exist, it returns a new PlayerConfig with a default player.
    pub fn extract_from_path(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let expanded_path = shellexpand::tilde(file_path).to_string();
        
        
        // Check if file exists
        if !Path::new(&expanded_path).exists() {
            println!("ERROR: File does not exist at path: {}", expanded_path);
            return Err(format!("File not found at {}", expanded_path).into());
        }
        
        let toml_string = match fs::read_to_string(&expanded_path) {
            Ok(contents) => {
                contents
            },
            Err(e) => {
                println!("ERROR reading file: {}", e);
                return Err(e.into());
            }
        };
        
        // Try to parse TOML
        match toml::from_str(&toml_string) {
            Ok(config) => Ok(config),
            Err(e) => {
                println!("ERROR parsing TOML: {}", e);
                Err(e.into())
            }
        }
    }

    pub fn save_to_path(mut self, file_path: &str) -> Result<(), Box<dyn Error>> {
        // Always update the last active date before saving
        let today = Local::now().date_naive();
        
        // Create a mutable copy to update the date
        self.player.counters.last_active_date = today;
        
        // Convert to TOML and write to file
        let toml_string = toml::to_string(&self)?;
        fs::write(file_path, toml_string)?;
        Ok(())
    }

}
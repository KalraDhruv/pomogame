use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::Error;

// A constant to represent the full value for Stamina and LazyMeter.
const MAX_STAT_VALUE: i32 = 100;

// --- Data Structures ---
#[derive(Serialize, Deserialize, Debug, Default, Clone )]
pub struct Player {
    pub name: String,
    pub level: Level,
    pub stamina: Stamina,
    pub sloth: Sloth,
    pub counters: Counters,
}

impl Player{
    pub fn new()->Self{
        let name= "Player".into();
        let level= Level{
            level : 1,
            level_experience : 0,
        };
        let stamina= Stamina{
            stamina_val : MAX_STAT_VALUE,
        };
        let sloth = Sloth{
            sloth_val : 0,
        };
        let counters = Counters{
            daily_completed_pomodoros: 0,
            daily_rests: 0,
            daily_streak: 0,
            last_active_date: NaiveDate::MIN,
        };
        return Player{
            name,
            level,
            stamina,
            sloth,
            counters,
        }
        

    }
}
// Struct to manage the player's level and total experience.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Level {
    pub level: u32,
    pub level_experience: i64,
}

impl Level {
    /// Calculates the experience required to level up.
    fn lvl_up_experience_required(&self) -> i64 {
        // Corrected calculation for experience required.
        // Rust does not have `**` operator for power, so we use `pow`.
        100 + ((1.375 * (self.level as f64).powi(2) + 5.0 * self.level as f64) as i64)
    }

    pub fn calculate_earned_exp(
        &self,
        user_defined_exp: f64,
        stamina: &Stamina,
        sloth: &Sloth,
    ) -> i64 {
        let stamina_bonus = 100.0 * (0.25 * (stamina.stamina_val as f64 / MAX_STAT_VALUE as f64));
        let sloth_penalty= user_defined_exp * 0.5 * (sloth.sloth_val as f64 / MAX_STAT_VALUE as f64);

        
        let final_exp = (user_defined_exp + stamina_bonus) - sloth_penalty;
        
        if final_exp < 0.0 {
            0
        } else {
            final_exp.round() as i64
        }
    }
    
    /// Adds experience and checks for a level-up.
    pub fn add_experience(&mut self, exp_gained: i64) {
        self.level_experience += exp_gained;
        let experience_required = self.lvl_up_experience_required();
        
        if self.level_experience >= experience_required {
            self.level += 1;
            self.level_experience -= experience_required;
        }
    }
}


// Struct to manage the player's stamina.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Stamina {
    pub stamina_val: i32,
}

impl Stamina {
    /// Resets stamina to its maximum value.
    pub fn reset_stamina(&mut self) {
        self.stamina_val = MAX_STAT_VALUE;
    }

    /// Reduces stamina based on the time overage during a pomodoro session.
    pub fn decrease_stamina(&mut self, time_extra: f64, time_pomodoro: f64) {
        if self.stamina_val == 0 {
            return;
        }

        let ratio = time_extra / time_pomodoro;
        let decrease = if ratio >= 1.0 {
            25
        } else if ratio >= 0.75 {
            18
        } else if ratio >= 0.5 {
            12
        } else if ratio >= 0.25 {
            6
        } else {
            0
        };
        self.stamina_val = self.stamina_val.saturating_sub(decrease);
        if self.stamina_val < 0{
            self.stamina_val = 0;
        }
    }
}

// Struct to manage the player's lazy meter.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Sloth{
    pub sloth_val: i32,
}

impl Sloth{
    /// Resets the lazy meter to its minimum value.
    pub fn reset_sloth(&mut self) {
        self.sloth_val= 0;
    }

    /// Increases the lazy meter based on the time overage during a break.
    pub fn increase_sloth(&mut self, time_extra: f64, time_break: f64) {
        if self.sloth_val== MAX_STAT_VALUE {
            return;
        }

        let ratio = time_extra / time_break;
        let increase = if ratio >= 1.0 {
            25
        } else if ratio >= 0.75 {
            18
        } else if ratio >= 0.5 {
            12
        } else if ratio >= 0.25 {
            6
        } else {
            0
        };

        self.sloth_val= self.sloth_val.saturating_add(increase);
        if self.sloth_val> MAX_STAT_VALUE {
            self.sloth_val= MAX_STAT_VALUE;
        }
    }
}

// Struct to hold the daily counters.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Counters {
    pub daily_completed_pomodoros: u32,
    pub daily_rests: u32,
    pub daily_streak: u32,
    pub last_active_date: NaiveDate,
}

// This allows us to print the player struct in a nicely formatted way.
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Player: {}", self.name)?;
        writeln!(f, "  Level: {}", self.level.level)?;
        writeln!(f, "  Experience: {}", self.level.level_experience)?;
        writeln!(f, "  Stamina: {}", self.stamina.stamina_val)?;
        writeln!(f, "  Sloth: {}", self.sloth.sloth_val)?;
        writeln!(f, "  Daily Pomodoros: {}", self.counters.daily_completed_pomodoros)?;
        writeln!(f, "  Daily Rests: {}", self.counters.daily_rests)?;
        writeln!(f, "  Daily Streak: {}", self.counters.daily_streak)?;
        writeln!(f, "  Last Active Date: {}", self.counters.last_active_date)?;
        Ok(())
    }
}
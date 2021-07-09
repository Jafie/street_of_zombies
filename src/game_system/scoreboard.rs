use std::time::{Instant};


use bevy::{
    prelude::*
};

use crate::{GAME_RESOLUTION_HEIGHT, GAME_RESOLUTION_WIDTH};


static INITIAL_PLAYER_HEALTH: u32 = 5;
static MAX_DIFFICULTY_LEVEL: u32 = 5;
static SECONDS_ELAPSED_BEFORE_NEXT_DIFFICULTY: u32 = 60;


struct ScoreAndInfoInternal {
    score: u32,
    health: u32,
    percent_until_next_level: u32,
    difficulty_level: u32,
    start_time: Instant
}

pub struct ScoreAndInfo {
    score_data: ScoreAndInfoInternal
}


impl ScoreAndInfo {
    pub fn new() -> Self {
        ScoreAndInfo {
            score_data: ScoreAndInfoInternal {score: 0, health: INITIAL_PLAYER_HEALTH, difficulty_level: 0, percent_until_next_level: 0, start_time: Instant::now()}
        }
    }

    pub fn add_to_score(&mut self, score_added: u32) {
        self.score_data.score += score_added;
    }

    pub fn remove_health(&mut self, health_to_remove: u32) {
        let remove_health_result = self.score_data.health.overflowing_sub(health_to_remove);
        match remove_health_result {
            (new_health, false) => self.score_data.health = new_health,
            (_, true) => self.score_data.health = 0
        }
    }

    pub fn update_percent_until_next_level(&mut self) {
        if self.score_data.difficulty_level == MAX_DIFFICULTY_LEVEL {
            return;
        }

        let second_for_next_difficulty_level  = SECONDS_ELAPSED_BEFORE_NEXT_DIFFICULTY as u64;
        let mut percent_elapsed = ((self.score_data.start_time.elapsed().as_secs()*100) / second_for_next_difficulty_level) as u32;

        if percent_elapsed >= 100 {
            self.score_data.start_time = Instant::now();
            self.increase_difficulty_level();
            percent_elapsed = 0;
        }

        self.score_data.percent_until_next_level = percent_elapsed;
    }

    pub fn update_scoarboard_text(&self, text: &mut Text, style: &mut Style) {
        let difficulty_level_list = vec!("EASY", "NORMAL", "HARD", "EXTREME", "STILL OK?", "!YOU ARE GOING TO DIE!");

        let difficulty_text = match difficulty_level_list.get(self.score_data.difficulty_level as usize) {
            Some(difficulty_level) => difficulty_level,
            None => "UNKNOWN"
        };

        if self.is_gameover() {
            self.print_board_game_over(text, style);
        }
        else {
            self.print_board_continue(text, difficulty_text);
        }
    }

    fn print_board_continue(&self, text: &mut Text, difficulty_text: &str) {
        text.sections[0].value = format!("SCORE: {:10}", self.get_score());
        text.sections[1].value = format!("    -  HEALTH: {:2}", self.get_health());
        text.sections[2].value = format!("    -  DIFFICULTY : {:30}  -  {:3}%", difficulty_text, self.get_percent_until_next_difficulty_level());

    }

    fn get_score(&self) -> u32 {
        self.score_data.score
    }

    fn get_health(&self) -> u32 {
        self.score_data.health
    }

    pub fn is_gameover(&self) -> bool {
        self.score_data.health == 0
    }

    pub fn get_difficulty_level(&self) -> u32 {
        self.score_data.difficulty_level
    }

    fn get_percent_until_next_difficulty_level(&self) -> u32 {
        if self.score_data.difficulty_level == MAX_DIFFICULTY_LEVEL {
            return 666;
        }
    
        self.score_data.percent_until_next_level
    }

    fn increase_difficulty_level(&mut self) {
        self.score_data.difficulty_level += 1;

        if self.score_data.difficulty_level > MAX_DIFFICULTY_LEVEL {
            self.score_data.difficulty_level = MAX_DIFFICULTY_LEVEL;
        }
    }

    fn print_board_game_over(&self, text: &mut Text, style: &mut Style) {
        style.position.top =  Val::Px(GAME_RESOLUTION_HEIGHT/2.);
        style.position.left =  Val::Px(GAME_RESOLUTION_WIDTH/4.);
        text.sections[0].value = format!("- GAME OVER -    ");
        text.sections[1].value = format!("Score =  {:10}", self.get_score());
        text.sections[2].value = format!("    - GAME OVER -");

    }
    
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use std::time::{Duration};
    use std::thread;

    #[test]
    fn score_board_add_system_test() {
        let mut player_data = ScoreAndInfo::new();
        player_data.add_to_score(5000);

        assert_eq!(player_data.get_score(), 5000);
    }

    #[test]
    fn health_overflow_test() {
        let mut player_data = ScoreAndInfo::new();
        player_data.remove_health(5000);

        assert_eq!(player_data.get_health(), 0);
    }
    #[test]
    fn health_remove_test() {
        let mut player_data = ScoreAndInfo::new();
        player_data.remove_health(1);

        assert_eq!(player_data.get_health(), INITIAL_PLAYER_HEALTH-1);
    }

    #[test]
    fn increase_difficulty_level_test() {
        let mut player_data = ScoreAndInfo::new();
        player_data.increase_difficulty_level();

        assert_eq!(player_data.get_difficulty_level(), 1);
    }

    #[test]
    fn increase_difficulty_level_to_max_test() {
        let mut player_data = ScoreAndInfo::new();

        for _ in 0..MAX_DIFFICULTY_LEVEL+5 {
            player_data.increase_difficulty_level();
        }

        assert_eq!(player_data.get_difficulty_level(), MAX_DIFFICULTY_LEVEL);
    }

    #[test]
    fn percent_test() {
        // This test can be problematic (Usage of sleep of 1 second). It is header to test if the "Percent" system works
        let mut player_data = ScoreAndInfo::new();

        let one_second = Duration::from_secs(1);
        thread::sleep(one_second);
        player_data.update_percent_until_next_level();

        let percent_for_one_sec = 100 / SECONDS_ELAPSED_BEFORE_NEXT_DIFFICULTY;

        assert_eq!(player_data.get_percent_until_next_difficulty_level() >= percent_for_one_sec, true);
    }

    #[test]
    fn game_over_test() {
        let mut player_data = ScoreAndInfo::new();

        player_data.remove_health(INITIAL_PLAYER_HEALTH);

        assert_eq!(player_data.is_gameover(), true);
    }

    #[test]
    fn game_continue_test() {
        let player_data = ScoreAndInfo::new();

        assert_eq!(player_data.is_gameover(), false);
    }
}

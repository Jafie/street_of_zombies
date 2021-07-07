use std::time::{Instant};


use bevy::{
    prelude::*
};


static PLAYER_INITIAL_LIFE: u32 = 5;
static MAX_DIFFICULTY_LEVEL: u32 = 5;
static SECONDS_ELAPSED_BEFORE_NEXT_DIFFICULTY: u32 = 60;


pub struct ScoreAndInfo {
    score: u32,
    life: u32,
    percent_until_next_level: u32,
    difficulty_level: u32,
    start_time: Instant
}

impl ScoreAndInfo {
    pub fn new() -> Self {
        ScoreAndInfo {score: 0, life: PLAYER_INITIAL_LIFE, difficulty_level: 0, percent_until_next_level: 0, start_time: Instant::now()}
    }

    pub fn add_to_score(&mut self, score_added: u32) {
        self.score += score_added;
    }

    pub fn remove_life(&mut self, life_to_remove: u32) {
        let remove_life_result = self.life.overflowing_sub(life_to_remove);
        match remove_life_result {
            (new_life, false) => self.life = new_life,
            (_, true) => self.life = 0
        }
    }

    pub fn update_percent_until_next_level(&mut self) {
        if self.difficulty_level == MAX_DIFFICULTY_LEVEL {
            return;
        }

        let second_for_next_difficulty_level  = SECONDS_ELAPSED_BEFORE_NEXT_DIFFICULTY as u64;
        let mut percent_elapsed = ((self.start_time.elapsed().as_secs()*100) / second_for_next_difficulty_level) as u32;

        if percent_elapsed >= 100 {
            self.start_time = Instant::now();
            self.increase_difficulty_level();
            percent_elapsed = 0;
        }

        self.percent_until_next_level = percent_elapsed;
    }

    pub fn update_scoarboard_text(&self, text: &mut Text) {
        let difficulty_level_list = vec!("EASY", "NORMAL", "HARD", "ULTRA HARD", "EXTREME", "!YOU WILL DIE!");


        text.sections[0].value = format!("Score: {:9}", self.get_score());
        text.sections[1].value = format!("    -  Life: {:1}", self.get_life());

        let difficulty_text = match difficulty_level_list.get(self.difficulty_level as usize) {
            Some(difficulty_level) => difficulty_level,
            None => "UNKNOWN"
        };

        text.sections[2].value = format!("    -  Difficulty : {:15}  -  {:3}%", difficulty_text, self.get_percent_until_next_difficulty_level());
    }

    fn get_score(&self) -> u32 {
        self.score
    }

    fn get_life(&self) -> u32 {
        self.life
    }

    pub fn get_difficulty_level(&self) -> u32 {
        self.difficulty_level
    }

    fn get_percent_until_next_difficulty_level(&self) -> u32 {
        if self.difficulty_level == MAX_DIFFICULTY_LEVEL {
            return 666;
        }
    
        self.percent_until_next_level
    }

    fn increase_difficulty_level(&mut self) {
        self.difficulty_level += 1;

        if self.difficulty_level > MAX_DIFFICULTY_LEVEL {
            self.difficulty_level = MAX_DIFFICULTY_LEVEL;
        }
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
    fn life_overflow_test() {
        let mut player_data = ScoreAndInfo::new();
        player_data.remove_life(5000);

        assert_eq!(player_data.get_life(), 0);
    }
    #[test]
    fn life_remove_test() {
        let mut player_data = ScoreAndInfo::new();
        player_data.remove_life(1);

        assert_eq!(player_data.get_life(), PLAYER_INITIAL_LIFE-1);
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
        let mut player_data = ScoreAndInfo::new();

        let one_second = Duration::from_secs(1);
        thread::sleep(one_second);
        player_data.update_percent_until_next_level();

        let percent_for_one_sec = 100 / SECONDS_ELAPSED_BEFORE_NEXT_DIFFICULTY;

        assert_eq!(player_data.get_percent_until_next_difficulty_level() >= percent_for_one_sec, true);
    }
}

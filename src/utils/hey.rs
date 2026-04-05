use colored::*;

use crate::{models::User};

pub struct Hey {
    pub prefix: String,
    pub user: User
}


impl Hey {

    pub fn hi(prefix: String, user: User) -> Self { 
        Self {
            prefix,
            user
        } 
    }

    pub fn get_stage_olam(&self) -> String {

        let mut space_name = "".to_string();
        let mut timeline_name = "".to_string();

        if self.user.stage() == "inspace" && !self.user.current_space_hash.is_empty() {
            space_name = match self.user.spaces.iter().find(|s| s.hash == self.user.current_space_hash) {
                Some(e) => e.name.clone(),
                None => "".to_string()
            };
        }

        if self.user.stage() == "intimeline" && !self.user.current_timeline_hash.is_empty() {
            let space = self.user.spaces.iter().find(|s| s.hash == self.user.current_space_hash).unwrap();

            space_name = match self.user.spaces.iter().find(|s| s.hash == self.user.current_space_hash) {
                Some(e) => e.name.clone(),
                None => "".to_string()
            };

            timeline_name = match space.timelines.iter().find(|t| t.hash == self.user.current_timeline_hash) {
                Some(e) => format!(".{}", e.name.clone()),
                None => "".to_string()
            }
        }

        let in_space = format!("{}{}", "~".yellow(), space_name.trim().replace(" ", "").yellow());
        let in_timeline = format!("{}", timeline_name.trim().replace(" ", "").purple());

        let stage = format!("{}{}", in_space, in_timeline);

        stage 

    }

    pub fn hey(&self, msg: &str) {
        println!("{} {} —— {}", format!("({})", self.prefix).blue(), self.get_stage_olam(), msg);
    }

    pub fn tip(&self, msg: &str) {
        println!("——{}—— {}", "?".yellow().bold(), msg.italic().dimmed());
    }

}
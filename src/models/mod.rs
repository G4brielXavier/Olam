
use serde::{ Deserialize, Serialize };


pub type SpaceList = Vec<Space>;
pub type TimelineList = Vec<Timeline>;
pub type EventList = Vec<Event>;


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Event {
    Age(Age),
    Year(Year)
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Space {
    
    pub hash: String,
    pub name: String,
    pub description: String,
    pub created_at: String,

    pub is_favorite: bool,

    pub timelines: TimelineList

}



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Timeline {

    pub hash: String,
    pub name: String,
    pub description: String,

    pub settings: PatternAge,
    pub events: EventList

}



#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PatternAge {

    pub year_name: String,
    pub age_name: String,
    pub year: u32, // 0

    pub max_year: u32, // 1000


}



#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Year {

    pub year: u32,
    pub note: Vec<String>,

}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Age {

    pub title: String,

    pub notes: Vec<String>,
    pub year: Vec<Year>,
    
    pub begin: u32,
    pub end: u32

}






#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {

    pub current_space_hash: String,
    pub current_timeline_hash: String,

    pub spaces: SpaceList

}


impl User {

    pub fn stage(&self) -> String {

        if self.current_timeline_hash.is_empty() && self.current_space_hash.is_empty() {
            return "outspace".to_string();
        }

        if !self.current_timeline_hash.is_empty() {
            return "intimeline".to_string();
        }
        
        if !self.current_space_hash.is_empty() {
            return "inspace".to_string();
        }

        
        "".to_string()
        
    }

}
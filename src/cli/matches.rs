use crate::{

    cli::commands::Commands, models::{ 
        Age, Event, EventList, PatternAge, Space, Timeline, User, Year
    }, utils::{
        fiman::Fiman, get_hash_8, get_timestamp, hey::Hey
    }

};

use colored::*;




pub fn matches(cmd: &Commands, hey: &Hey, fiman: &mut Fiman, user: &mut User) -> Result<(), Box<dyn std::error::Error>> {

    match cmd {

        
        Commands::Settings { title, desc, max, yearname, agename } => {

            let stage = user.stage();

            println!();
            
            match stage.as_str() {

                "outspace" => {

                    hey.hey(&format!("There's nothing here to change."));
                    hey.tip(&format!("{} is used only {} or {}", "olam set".yellow(), "INSPACE".yellow(), "INTIMELINE".yellow()));

                }

                "inspace" => {

                    let space = user.spaces.iter_mut().find(|s| s.hash == user.current_space_hash).unwrap();

                    if let Some(title) = title {
                        
                        space.name = title.to_string();
                        hey.hey(&format!("Space's Name changed to {}", format!("\"{}\"", title).yellow()));

                    }
                    
                    if let Some(desc) = desc {

                        space.description = desc.to_string();
                        hey.hey(&format!("Space's Description changed to {}", format!("\"{}\"", desc).yellow()));

                    }

                    fiman.write(&user, &fiman.user_data_path.clone())?;

                }

                "intimeline" => {

                    let space = user.spaces.iter_mut().find(|s| s.hash == user.current_space_hash).unwrap();
                    let timeline = space.timelines.iter_mut().find(|t| t.hash == user.current_timeline_hash).unwrap();

                    if let Some(title) = title {
                        
                        timeline.name = title.to_string();
                        hey.hey(&format!("Timeline's Name changed to {}", format!("\"{}\"", title).yellow()));

                    }
                    
                    if let Some(desc) = desc {

                        timeline.description = desc.to_string();
                        hey.hey(&format!("Timeline's Description changed to {}", format!("\"{}\"", desc).yellow()));

                    }

                    if let Some(max) = max {

                        let msg = if timeline.settings.max_year > timeline.settings.max_year + *max {
                            "+improved".green()
                        } else {
                            "-reduced".green()
                        };
                        
                        timeline.settings.max_year = *max;
                        
                        hey.hey(&format!("Timeline's limit was {} to {}", msg, max.to_string().yellow()));

                    }

                    if let Some(yearname) = yearname {

                        let old = timeline.settings.year_name.clone();

                        timeline.settings.year_name = yearname.to_string();
                        hey.hey(&format!("{} changed to {}", format!("\"{}\"", old).yellow(), format!("\"{}\"", yearname).yellow()));

                    }
                    
                    if let Some(agename) = agename {

                        let old = timeline.settings.age_name.clone();

                        timeline.settings.age_name = agename.to_string();
                        hey.hey(&format!("{} changed to {}", format!("\"{}\"", old).yellow(), format!("\"{}\"", agename).yellow()));

                    }

                    fiman.write(&user, &fiman.user_data_path.clone())?;

                }

                &_ => todo!()

            }

            println!();

            Ok(())

        }


        Commands::Evoke { name, immerse } => {

            let stage = user.stage();

            println!();

            match stage.as_str() {

                "outspace" => {

                    if !name.is_empty() {
                
                        let name = name.trim().to_string();

                        let immerse_msg = format!("olam immerse \"{}\"", name);
                        let input_hash = format!("{}{}", name, get_timestamp());

                        let hash = get_hash_8(&input_hash.as_bytes());

                        let already_exist = user.spaces.iter().any(|s| s.hash == hash);

                        if !already_exist {
                            
                            let space = Space {
                                hash: hash.clone(),
                                name: name.clone(),
                                description: "".to_string(),
                                created_at: "".to_string(),
                                is_favorite: false,
                                timelines: vec![]
                            };

                            user.spaces.push(space);

                            hey.hey(&format!("Evoking {}...", name.yellow()));
                            hey.tip(&format!("{} has evoked!", name.yellow()));
                            
                            if !*immerse {
                                hey.tip(&format!("Use: {} to {}", immerse_msg.yellow(), "immerse".italic()));
                            }

                            if *immerse {

                                if user.current_space_hash.is_empty() {
                                
                                    println!();

                                    user.current_space_hash = hash;

                                    hey.hey(&format!("Now you are immersed in {}!", name.yellow()));
                                
                                } else {

                                    hey.hey("You are already immersed.");
                                    hey.tip(&format!("Use {}", "olam emerge".yellow()));

                                }

                            }

                            fiman.write(&user, &fiman.user_data_path.clone())?;

                        } else {
                            println!();
                            hey.hey(&format!("Already exist a Space with this name: {}", name));
                            println!();
                        }

                    } else {
                        println!();
                        hey.hey(&format!("Provides the Space's Name"));
                        hey.tip(&format!("use: {}", "olam evoke \"SpaceName\"".yellow()));
                        println!();
                    }

                },

                _ => {
                    println!();
                    hey.hey(&format!("You are {} in a space or timeline.", "immersed".italic()));
                    hey.tip(&format!("use: {} to emerge", "olam emerge".yellow()));
                    println!();
                }

            }

            println!();

            Ok(())
        
        }


        Commands::Efface { hash } => {

            let stage = user.stage();

            println!();

            match stage.as_str() {

                "outspace" => {

                    if !hash.is_empty() {

                        let spaces = user.spaces.clone();

                        if spaces.len() > 0 {

                            let pos_space_effaced = spaces.iter().position(|s| s.hash == hash.trim().to_string()); 

                            if let Some(i) = pos_space_effaced {
                                
                                let space_effaced_name = spaces[i].name.clone();

                                user.spaces.retain(|s| s.hash != hash.trim().to_string());
                                hey.hey(&format!("{} has effaced!", space_effaced_name.yellow()));

                                fiman.write(&user, &fiman.user_data_path.clone())?;

                            } else {
                                println!();
                                hey.hey(&format!("Not found or not exist a Space with this hash: {}", hash.yellow()));
                                hey.tip(&format!("Use: {} to see all your spaces and get the right HASH", "olam ls".yellow()));
                                println!();
                            }

                        } else {
                            println!();
                            hey.hey(&format!("You don't have spaces to immerse."));
                            hey.tip(&format!("Create one using this: {}", "olam evoke \"MySpace\"".yellow()));
                            println!();
                        }

                    } else {
                        println!();
                        hey.hey(&format!("Provide the Space's Hash to find it and immerse"));
                        hey.tip(&format!("Like this: {}", "olam immerse 1234abcd".yellow()));
                        println!();
                    }

                }

                "inspace" => {

                    println!();
                    hey.hey(&format!("To use this command, {} from the Space.", "emerge".yellow()));
                    hey.tip(&format!("Use: {}", "olam em".yellow()));
                    println!();

                }

                "intimeline" => {

                    println!();
                    hey.hey(&format!("To use this command, {} from the Timeline and the Space.", "emerge".yellow()));
                    hey.tip(&format!("Use: {}", "olam em".yellow()));
                    println!();

                }

                &_ => todo!()

            }

            println!();

            Ok(())

        }


        Commands::Immerse { hash } => {

            let stage = user.stage();

            println!();

            match stage.as_str() {

                "outspace" => {

                    if !hash.is_empty() {

                        if !user.current_space_hash.is_empty() {
                            hey.hey(&format!("You are already {} in a space.", "immersed".italic()));
                            hey.tip(&format!("use: {} to emerge from current space", "olam emerge".yellow()));
                            
                            return Ok(())
                        }

                        let spaces = user.spaces.clone();

                        if spaces.len() > 0 {

                            let pos_space_effaced = spaces.iter().position(|s| s.hash == hash.trim().to_string()); 

                            if let Some(i) = pos_space_effaced {
                                
                                let space_effaced_name = spaces[i].name.clone();

                                user.current_space_hash = hash.clone();
                                hey.hey(&format!("Now you are immersed in {}!", space_effaced_name.yellow()));

                                fiman.write(&user, &fiman.user_data_path.clone())?;

                            } else {
                                println!();
                                hey.hey(&format!("Not found or not exist a Space with this hash: {}", hash.yellow()));
                                hey.tip(&format!("Use: {} to see all your spaces and get the right HASH", "olam ls".yellow()));
                                println!();
                            }

                        } else {
                            println!();
                            hey.hey(&format!("You don't have spaces to immerse."));
                            hey.tip(&format!("Create one using this: {}", "olam evoke \"MySpace\"".yellow()));
                            println!();
                        }

                    } else {
                        println!();
                        hey.hey(&format!("Provide the Space's Hash to find it and immerse"));
                        hey.tip(&format!("Like this: {}", "olam immerse 1234abcd".yellow()));
                        println!();
                    }

                }

                "inspace" => {

                    if !hash.is_empty() {

                        if !user.current_timeline_hash.is_empty() {
                            hey.hey(&format!("You are already {} in a timeline.", "immersed".italic()));
                            hey.tip(&format!("use: {} to emerge from current timeline", "olam emerge".yellow()));
                            
                            return Ok(())
                        }

                        let spaces = user.spaces.clone();

                        if spaces.len() > 0 {

                            let space = spaces.iter().find(|s| s.hash == user.current_space_hash); 

                            if let Some(space) = space {
                                
                                let timeline = space.timelines.iter().find(|t| t.hash == *hash);

                                if let Some(timeline) = timeline {

                                    user.current_timeline_hash = timeline.hash.clone();
                                    hey.hey(&format!("Now you are immersed in {}!", timeline.name.yellow()));
    
                                    fiman.write(&user, &fiman.user_data_path.clone())?;
                                
                                } else {
                                    println!();
                                    hey.hey(&format!("Not found or not exist a Timeline with this hash: {}", hash.yellow()));
                                    hey.tip(&format!("Use: {} to see all your timeline and get the right HASH", "olam ls".yellow()));
                                    println!();
                                }


                            } else {
                                println!();
                                hey.hey(&format!("Ocurred a {} error: {}", "FATAL".red().italic(), user.current_space_hash.yellow()));
                                hey.tip(&format!("You are stuck, use {} to force an emerge to {}", "olam outspace".yellow(), "OUTSPACE".yellow()));
                                println!();
                            }

                        } else {
                            println!();
                            hey.hey(&format!("You don't have timelines to immerse."));
                            hey.tip(&format!("Create one using this: {}", "olam add \"MyTimeline\"".yellow()));
                            println!();
                        }

                    } else {
                        println!();
                        hey.hey(&format!("Provide the Timeline's Hash to find it and immerse"));
                        hey.tip(&format!("Like this: {}", "olam immerse 1234abcd".yellow()));
                        println!();
                    }

                }

                _ => {

                }

            }

            println!();
            Ok(())

        }


        Commands::Emerge { out } => {

            let stage = user.stage();

            println!();

            match stage.as_str() {
                
                "outspace" => {

                    hey.hey("There's nowhere to emerge");

                },

                "inspace" => {

                    if *out {

                        user.current_timeline_hash = "".to_string();
                        user.current_space_hash = "".to_string();

                        fiman.write(&user, &fiman.user_data_path.clone())?;

                        hey.hey(&format!("{} You are {} now", "OUT!!".yellow(), "OUTSPACE".yellow()))

                    } else {

                        user.current_space_hash = "".to_string();
                        fiman.write(&user, &fiman.user_data_path.clone())?;
                        
                        hey.hey(&format!("{} You are {} now", "Emerged!".yellow(), "OUTSPACE".yellow()));

                    }

                },

                "intimeline" => {

                    if *out {

                        user.current_timeline_hash = "".to_string();
                        user.current_space_hash = "".to_string();

                        fiman.write(&user, &fiman.user_data_path.clone())?;

                        hey.hey(&format!("{} You are {} now", "OUT!!".yellow(), "OUTSPACE".yellow()))

                    } else {

                        user.current_timeline_hash = "".to_string();
                        fiman.write(&user, &fiman.user_data_path.clone())?;
                        
                        hey.hey(&format!("{} You are {} now", "Emerged!".yellow(), "INSPACE".yellow()))
                    }
                    

                },

                &_ => todo!()

            }

            println!();
            Ok(())

        }


        Commands::Ls => {

            let stage = user.stage();

            println!();

            match stage.as_str() {

                "outspace" => {

                    if user.spaces.len() > 0 {

                        let spaces = user.spaces.clone();
                        
                        for space in spaces.iter() {
                            let description = format!("-- {}", space.description).dimmed();
                            let is_favorite = if space.is_favorite { "◈" } else { "" };

                            println!("• {} {} {} [{} TLs] {}", space.hash.dimmed(), space.name.yellow(), is_favorite.bold().yellow(), space.timelines.len().to_string().yellow(), description);
                        }

                    } else {
                        hey.hey("You don't have spaces to show.");
                        hey.tip(&format!("use: {} to create one", "olam evoke \"myspace\"".yellow()));
                    }
                    
                }

                "inspace" => {

                    let space = user.spaces.iter().find(|s| s.hash == user.current_space_hash);

                    if let Some(spa) = space {

                        if spa.timelines.len() > 0 {

                            for timeline in spa.timelines.iter() {
                                let description = format!("-- {}", timeline.description).dimmed();

                                println!("• {} {} ({}, {}) [{} ev] {}", timeline.hash.dimmed(), format!("\"{}\"", timeline.name.yellow()).yellow(), format!("\"{}\"", timeline.settings.year_name).yellow(), format!("\"{}\"", timeline.settings.age_name).yellow(), timeline.events.len().to_string().yellow(), description);

                            }

                        } else {
                            hey.hey("You don't have timelines to show.");
                            hey.tip(&format!("use: {} to create one", "olam add \"my timeline\"".yellow()));
                        }

                    }

                }

                _ => {

                    println!();
                    hey.hey(&format!("The command {} don't works to {}", "olam ls".yellow(), "INTIMELINE".yellow()));
                    hey.tip(&format!("If you want to preview your timeline, use: {}", "olam timeline".yellow()));
                    println!();

                }

            }

            println!();
            Ok(())

        }


        Commands::Stage => {

            let stage = user.stage();

            println!();
            hey.hey(&format!("{}", stage.to_uppercase().yellow()));
            println!();

            Ok(())
        }


        Commands::Add { name, immerse } => {    

            let stage = user.stage();

            println!();

            match stage.as_str() {

                "inspace" => {

                    if !name.is_empty() {

                        let space = user.spaces.iter_mut().find(|s| s.hash == user.current_space_hash);

                        if let Some(space) = space {

                            let name = name.trim().to_string();

                            let input_hash = format!("{}{}", name, get_timestamp());
                            let hash = get_hash_8(&input_hash.as_bytes());
                            
                            let immerse_msg = format!("olam immerse \"{}\"", hash);

                            let new_timeline = Timeline {

                                hash: hash.clone(),
                                name: name.clone(),
                                description: "".to_string(),
                                settings: PatternAge {
                                    year_name: "Year".to_string(),
                                    age_name: "Age".to_string(),
                                    year: 0,
                                    max_year: 1000,
                                },
                                events: vec![]

                            };

                            space.timelines.push(new_timeline);

                            hey.hey(&format!("Adding {}...", name.yellow()));
                            hey.tip(&format!("{} added!", name.yellow()));
                            hey.tip(&format!("{} has {} years as default", name.yellow(), "1000".yellow()));
                            hey.tip(&format!("use: {} if you want change it", "olam settings --max <years>".yellow()));
                            hey.tip(&format!("If you want change the name \"Year\" to another name, use: {}", "olam settings --yname \"otheryear\"".yellow()));

                            if !*immerse {
                                hey.tip(&format!("Use: {} to {}", immerse_msg.yellow(), "immerse".italic()));
                            }

                            if *immerse {

                                if user.current_timeline_hash.is_empty() {
                                
                                    println!();

                                    user.current_timeline_hash = hash;

                                    hey.hey(&format!("Now you are immersed in {}!", name.yellow()));
                                
                                } else {

                                    hey.hey("You are already immersed.");
                                    hey.tip(&format!("Use {}", "olam emerge".yellow()));

                                }

                            }

                            fiman.write(&user, &fiman.user_data_path.clone())?;

                        } else {
                            hey.hey(&format!("Occurred an error to find the Space"));
                            hey.tip(&format!("Try: {} to solve", "olam em -o"));
                        }

                    } else {
                        println!();
                        hey.hey(&format!("Provides the Timeline's Name"));
                        hey.tip(&format!("Use: {}", "olam add \"TimelineName\"".yellow()));
                        println!();
                    }

                }
                
                _ => {
                    println!();
                    hey.hey(&format!("You are not {} in a Space.", "immersed".italic()));
                    hey.tip(&format!("use: {}", "olam immerse <SPACE_HASH>".yellow()));
                    println!();
                }
            }

            println!();

            Ok(())

        }


        Commands::Expun { hash } => {

            let stage = user.stage();

            println!();

            match stage.as_str() {

                "inspace" => {

                    if !hash.is_empty() {

                        let space = user.spaces.iter_mut().find(|s| s.hash == user.current_space_hash);

                        if let Some(space) = space {

                            let hash  = hash.trim().to_string();
                            let timeline_name = space.timelines.iter().find(|t| t.hash == hash).unwrap().name.clone();

                            space.timelines.retain(|t| t.hash != hash);

                            fiman.write(&user, &fiman.user_data_path.clone())?;

                            hey.hey(&format!("{} expunged!", timeline_name.yellow()));

                        } else {
                            println!("Ocurred an error to find Space.")
                        }

                    } else {
                        println!();
                        hey.hey(&format!("Provides the Timeline's Name"));
                        println!();
                    }

                }
                
                "intimeline" => {

                    println!();
                    hey.hey(&format!("You can not delete a timeline while you are immersed."));
                    hey.tip(&format!("Use: {} to emerge firstly", "olam emerge".yellow()));
                    println!();

                }
                
                _ => {
                    hey.hey(&format!("This command is to remove timelines, but you are not {} in a Space to do it", "immersed ".italic()));
                    hey.tip(&format!("use: {} to immerse in a Space", "olam immerse <SPACE_HASH>".yellow()));
                }
            }

            println!();

            Ok(())

        }


        Commands::Year { age, year, note } => {

            let stage = user.stage();

            println!();

            match stage.as_str() {

                "intimeline" => {

                    if let Some(new_year) = year {

                        if let Some(note) = note {

                            let space = user.spaces.iter_mut().find(|s| s.hash == user.current_space_hash).unwrap();
                            let timeline = space.timelines.iter_mut().find(|t| t.hash == user.current_timeline_hash).unwrap();

                            let events: &mut EventList = &mut timeline.events;

                            // Add a Year to an Age
                            if let Some(age) = age {

                                let age_event = events.iter_mut().find(|e| {
                                    if let Event::Age(e) = e {
                                        e.title == *age
                                    } else {
                                        false
                                    }
                                });  

                                if let Some(age) = age_event {

                                    if let Event::Age(age) = age {

                                        let year_exist = age.year.iter_mut().find(|y| y.year == *new_year);

                                        if let Some(year) = year_exist {

                                            year.note.push(note.to_string());
                                            
                                            hey.hey(&format!("Added {} to {} {}", format!("\"{}\"", note.to_string()).italic().yellow(), timeline.settings.year_name.yellow(), year.year.to_string().yellow()));
                                            
                                            fiman.write(&user, &fiman.user_data_path.clone())?;

                                        } else {

                                            if *new_year >= age.begin as u32 && *new_year <= age.end as u32 {
                                                
                                                let year = Year {
                                                    year: *new_year,
                                                    note: vec![note.to_string()]
                                                };
    
                                                hey.hey(&format!("Added {} {} to {}", timeline.settings.year_name.yellow(), year.year.clone().to_string().yellow(), age.title.yellow()));
    
                                                age.year.push(year);
                                                fiman.write(&user, &fiman.user_data_path.clone())?;

                                            } else {

                                                println!();
                                                hey.hey(&format!("Your new year must be between {} and {}", 0.to_string().yellow(), timeline.settings.max_year.to_string().yellow()));
                                                hey.tip(&format!("If you want change the limit, use: {}", "olam set --max 2026".yellow()));
                                                println!();

                                            }
                                            
                                        }   
                                    
                                    }
                                
                                } else {

                                    // Add a Year to timeline
                                    let exists = events.iter().any(|e| {
                                        if let Event::Year(e) = e {
                                            e.year == *new_year
                                        } else {
                                            false
                                        }
                                    });  

                                    if exists {

                                        let event = events.iter_mut().find(|e| {
                                            if let Event::Year(e) = e {
                                                e.year == *new_year
                                            } else {
                                                false
                                            }
                                        }).unwrap();

                                        if let Event::Year(y) = event {

                                            y.note.push(note.to_string());
                                            
                                            hey.hey(&format!("Added {} to {} {}", format!("\"{}\"", note.to_string()).italic().yellow(), timeline.settings.year_name.yellow(), y.year.to_string().yellow()));
                                            
                                            fiman.write(&user, &fiman.user_data_path.clone())?;
                                        
                                        }

                                    } else {    

                                        if *new_year >= 0 as u32 && *new_year <= timeline.settings.max_year as u32 {

                                            let year = Year {
                                                year: *new_year,
                                                note: vec![note.to_string()]
                                            };

                                            timeline.settings.year = year.year.clone();

                                            hey.hey(&format!("New {} to {} {}", format!("\"{}\"", note.to_string().italic()).italic().yellow(), timeline.settings.year_name.yellow(), year.year.to_string().yellow()));

                                            events.push(Event::Year(year));
                                            fiman.write(&user, &fiman.user_data_path.clone())?;

                                        } else {

                                            println!();
                                            hey.hey(&format!("Your new year must be between {} and {}", 0.to_string().yellow(), timeline.settings.max_year.to_string().yellow()));
                                            hey.tip(&format!("If you want change the limit, use: {}", "olam settings --max 2026".yellow()));
                                            println!();

                                        }
                                        
                                    }                     


                                }

                            } else {

                                // Add a Year to timeline
                                    let exists = events.iter().any(|e| {
                                        if let Event::Year(e) = e {
                                            e.year == *new_year
                                        } else {
                                            false
                                        }
                                    });  

                                    if exists {

                                        let event = events.iter_mut().find(|e| {
                                            if let Event::Year(e) = e {
                                                e.year == *new_year
                                            } else {
                                                false
                                            }
                                        }).unwrap();

                                        if let Event::Year(y) = event {

                                            y.note.push(note.to_string());
                                            
                                            hey.hey(&format!("Added {} to {} {}", format!("\"{}\"", note.to_string()).italic().yellow(), timeline.settings.year_name.yellow(), y.year.to_string().yellow()));
                                            
                                            fiman.write(&user, &fiman.user_data_path.clone())?;
                                        
                                        }

                                    } else {    

                                        if *new_year >= timeline.settings.year as u32 && *new_year <= timeline.settings.max_year as u32 {

                                            let year = Year {
                                                year: *new_year,
                                                note: vec![note.to_string()]
                                            };

                                            timeline.settings.year = year.year.clone();

                                            hey.hey(&format!("New {} to {} {}", format!("\"{}\"", note.to_string().italic()).italic().yellow(), timeline.settings.year_name.yellow(), year.year.to_string().yellow()));

                                            events.push(Event::Year(year));
                                            fiman.write(&user, &fiman.user_data_path.clone())?;

                                        } else {

                                            println!();
                                            hey.hey(&format!("Your new year must be between {} and {}", 0.to_string().yellow(), timeline.settings.max_year.to_string().yellow()));
                                            hey.tip(&format!("If you want change the limit, use: {}", "olam settings --max 2026".yellow()));
                                            println!();

                                        }
                                        
                                    }                     


                            }

                        
                        } else {

                            println!();
                            hey.hey(&format!("Note don't provided"));
                            hey.tip(&format!("Define the note like this: {}", "olam year 435 \"A war broke out\"".yellow()));
                            println!();

                        }
                    
                    } else {

                        println!();
                        hey.hey(&format!("Year don't provided"));
                        hey.tip(&format!("Define the year like this: {}", "olam year 20".yellow()));
                        println!();

                    }

                }

                _ => {

                    println!();
                    hey.hey(&format!("You can't use this command out of a timeline"));
                    hey.tip(&format!("Use: {}", "olam add \"timeline\"".yellow()));
                    println!();

                }

            }

            println!();

            Ok(())

        }

        
        Commands::Age { name, disso, between } => {

            let stage = user.stage();

            println!();

            match stage.as_str() {

                "intimeline" => {

                    if let Some(title) = name {

                        if let Some(disso) = disso {

                            let space = user.spaces.iter_mut().find(|s| s.hash == user.current_space_hash).unwrap();
                            let timeline = space.timelines.iter_mut().find(|t| t.hash == user.current_timeline_hash).unwrap();

                            let events: &mut EventList = &mut timeline.events;

                            let exists = events.iter().any(|e| {
                                if let Event::Age(e) = e {
                                    e.title == *title
                                } else {
                                    false
                                }
                            });  

                            if !exists {

                                if timeline.settings.year + disso <= timeline.settings.max_year {

                                    let last_event = events.last().unwrap();

                                    let last_year = if let Event::Year(y) = last_event {
                                        y.year
                                    } else if let Event::Age(a) = last_event {
                                        a.end
                                    } else {
                                        0
                                    };

                                    let begin = last_year + 1;

                                    let age = Age {
                                        title: title.clone(),
                                        notes: vec![],
                                        year: vec![],
                                        begin: begin.clone(),
                                        end: disso.clone() + timeline.settings.year
                                    };

                                    timeline.settings.year += age.end.clone();

                                    events.push(Event::Age(age));

                                    hey.hey(&format!("The Age {} was added in the {} {} at {}!", title.yellow(), timeline.settings.year_name.yellow(), begin.to_string().yellow(), timeline.name.yellow()));
                                    hey.tip(&format!("{} will be dissolved in {} {}", title.yellow(), timeline.settings.year_name.yellow(), disso.to_string().yellow()));

                                    fiman.write(&user, &fiman.user_data_path.clone())?;

                                } else {

                                    println!();
                                    hey.hey(&format!("{} exceeds the timeline's limit, that is {} {}", disso.to_string().yellow(), timeline.settings.max_year.to_string().yellow(), timeline.settings.year_name.yellow()));
                                    hey.tip(&format!("Try define a year between {} and {}", "0".yellow(), timeline.settings.max_year.to_string().yellow()));
                                    println!();

                                }


                            } else {

                                hey.hey(&format!("You can't have two same age in a timeline"));
                                hey.tip(&format!("Set a different name for each Age."));
                                
                            }                     
                        
                        } else if let Some(between) = between {

                            let space = user.spaces.iter_mut().find(|s| s.hash == user.current_space_hash).unwrap();
                            let timeline = space.timelines.iter_mut().find(|t| t.hash == user.current_timeline_hash).unwrap();

                            let events: &mut EventList = &mut timeline.events;

                            let exists = events.iter().any(|e| {
                                if let Event::Age(e) = e {
                                    e.title == *title
                                } else {
                                    false
                                }
                            });  
                            
                            if exists {

                                let age = events.iter_mut().find(|e| {
                                    if let Event::Age(a) = e {
                                        a.title == *title
                                    } else {
                                        false
                                    }
                                }).unwrap();

                                if let Event::Age(age) = age {

                                    age.notes.push(between.to_string());

                                    hey.hey(&format!("A note was added in the {} at {}!", "between".yellow(), age.title.yellow()));
                                    
                                    fiman.write(&user, &fiman.user_data_path.clone())?;
                                    
                                }

                            } else {

                                hey.hey(&format!("You can't have two same age in a timeline"));
                                hey.tip(&format!("Set a different name for each Age."));
                                
                            }

                        } else {

                            println!();
                            hey.hey(&format!("Dissolvation value or Between Note don't provided"));
                            hey.tip(&format!("Define the disso like this: {}", "olam age \"AgeName\" --disso 100".yellow()));
                            hey.tip(&format!("Or add a note in {}: {}", "between".yellow(), "olam age \"AgeName\" --between \"A new era...\"".yellow()));
                            println!();

                        }

                    } else {

                        println!();
                        hey.hey(&format!("Age's Name don't provided"));
                        hey.tip(&format!("Define the name like this: {}", "olam age \"AgeName\" -d 100".yellow()));
                        println!();

                    }

                }

                _ => {

                    println!();
                    hey.hey(&format!("You can't use this command out of a timeline"));
                    hey.tip(&format!("Use: {}", "olam add \"timeline\"".yellow()));
                    println!();

                }

            }

            println!();

            Ok(())

        }


        Commands::Void { year, age, pop } => {

            let stage = user.stage();

            println!();

            match stage.as_str() {

                "intimeline" => {

                    // Remove an Age or Year
                    let space = user.spaces.iter_mut().find(|s| s.hash == user.current_space_hash).unwrap();
                    let timeline = space.timelines.iter_mut().find(|t| t.hash == user.current_timeline_hash).unwrap();

                    let events: &mut EventList = &mut timeline.events;

                    // olam void 10 --> remove the year 10 from timeline
                    if let Some(year) = year && let None = age {

                        let exists_year = events.iter().any(|e| {
                            if let Event::Year(y) = e {
                                y.year == *year
                            } else {
                                false
                            }
                        });

                        if exists_year {

                            events.retain(|e| {
                                if let Event::Year(e) = e {
                                    e.year != *year
                                } else {
                                    true
                                }
                            });

                            timeline.settings.year -= year;

                            hey.hey(&format!("{} {} removed from {}", timeline.settings.year_name.yellow(), year.to_string().yellow(), timeline.name.yellow()))

                        } else {

                            hey.hey(&format!("{} {} not found in {}", timeline.settings.year_name.yellow(), year.to_string().yellow(), timeline.name.yellow()));
                            hey.tip(&format!("Use: {} to preview", "olam tl".yellow()));

                        }
                    } else if let Some(age) = age {

                        let exists_age = events.iter().any(|e| {
                            if let Event::Age(a) = e {
                                a.title == *age
                            } else {
                                false
                            }
                        });

                        if exists_age {

                            if *pop {

                                // Remove the last note in < Between >

                                let eve = events.iter_mut().find(|e| {
                                    if let Event::Age(a) = e {
                                        a.title == *age 
                                    } else {
                                        false
                                    }
                                }).unwrap();

                                if let Event::Age(a) = eve {

                                    let note_removed = a.notes.pop().unwrap();
                                
                                    hey.hey(&format!("{} removed from {}", format!("\"{}\"", note_removed).yellow(), "< Between >".yellow()));

                                }

                                fiman.write(&user, &fiman.user_data_path.clone())?;
                                println!();

                                return Ok(())
                            }

                            if let Some(year) = year {

                                // Remove a Year in < Event >
                                let eve = events.iter_mut().find(|e| {
                                    if let Event::Age(a) = e {
                                        a.title == *age 
                                    } else {
                                        false
                                    }
                                }).unwrap();

                                if let Event::Age(a) = eve {

                                    let exists = a.year.iter().any(|y| y.year == *year);

                                    if exists {

                                        a.year.retain(|y| y.year != *year);
                                
                                        hey.hey(&format!("{} {} removed from {} in {}", timeline.settings.year_name.yellow(), year.to_string().yellow(), "< Events >".yellow(), a.title.yellow()))

                                    } else {

                                        hey.hey(&format!("{} {} not found in {} from {}", timeline.settings.year_name.yellow(), year.to_string().yellow(), "< Events >".yellow(), a.title.yellow()));
                                        hey.tip(&format!("Use: {} to preview", "olam tl".yellow()));

                                    }

                                }

                                fiman.write(&user, &fiman.user_data_path.clone())?;
                                println!();

                                return Ok(())


                            } else {

                                // Remove an Age
                                let e_age = events.iter().find(|e| {
                                    if let Event::Age(a) = e {
                                        a.title == *age 
                                    } else {
                                        false
                                    }
                                }).unwrap();

                                let age_to_remove_end: u32;

                                if let Event::Age(age) = e_age {
                                    age_to_remove_end = age.end;
                                } else {
                                    age_to_remove_end = 0;
                                }

                                events.retain(|e| {
                                    if let Event::Age(a) = e {
                                        a.title != *age
                                    } else {
                                        true
                                    }
                                });

                                timeline.settings.year -= age_to_remove_end;

                                hey.hey(&format!("{} {} removed from {}", timeline.settings.age_name.yellow(), format!("\"{}\"", age).yellow(), format!("\"{}\"", timeline.name).yellow()))

                            }

                        } else {

                            hey.hey(&format!("{} {} not found in {}", timeline.settings.age_name.yellow(), age.yellow(), timeline.name.yellow()));
                            hey.tip(&format!("Use: {} to preview", "olam tl".yellow()));

                        }

                    }

                    fiman.write(&user, &fiman.user_data_path.clone())?;

                }

                _ => {

                    hey.hey("You can't void nothing out of a timeline.");
                    hey.tip(&format!("Immerse in a timeline to use this {}", "olam void".yellow()));

                }

            }

            println!();

            Ok(())

        }


        Commands::Timeline => {

            let stage = user.stage();

            println!();

            match stage.as_str() {

                "intimeline" => {

                    let space = user.spaces.iter().find(|s| s.hash == user.current_space_hash).unwrap();
                    let timeline = space.timelines.iter().find(|t| t.hash == user.current_timeline_hash).unwrap();
                    
                    let events = timeline.events.clone();
                    let settings = timeline.settings.clone();

                    hey.hey(&format!("Previewing {}", timeline.name.yellow()));

                    let ages: Vec<&Event> = events.iter().filter(|e| matches!(e, Event::Age(..))).collect();

                    hey.tip(&format!("{} {}", settings.year.to_string().yellow(), format!("{}(s)", settings.year_name).yellow()));
                    hey.tip(&format!("{} {}", ages.len().to_string().yellow(), format!("{}(s)", settings.age_name).yellow()));
                    hey.tip(&format!("Max {} {}", settings.max_year.to_string().yellow(), format!("{}(s)", settings.year_name).yellow()));

                    println!();

                    for event in events.iter() {

                        if let Event::Year(year) = event {

                            println!("−–—< {} {} >—–−", settings.year_name.yellow(), year.year.to_string().yellow());
                            println!();
                            for note in year.note.iter() {
                                
                                println!("       {} {}", "•".dimmed(), note.dimmed());
                                
                            }
                            println!();

                        } else if let Event::Age(age) = event {

                            println!("−–—< {} — {} {} >—–−", age.title.blue(), settings.year_name.yellow(), age.begin.to_string().yellow());
                            println!();

                            println!("      < {} - {} notes >", "Between".yellow(), age.notes.len().to_string().yellow());
                            println!();
                            for note in age.notes.iter() {
                                
                                println!("            {} {}", "•".dimmed(), note.dimmed());
                                
                            }
                            
                            println!();
                            println!("      < {} - {} ev >", "Events".yellow(), age.year.len().to_string().yellow());
                            println!();
                            
                            for year in age.year.iter() {

                                println!("            < {} {} >", timeline.settings.year_name.to_string().yellow(), year.year.to_string().yellow());
                                println!();
                                for note in year.note.iter() {
                                    println!("                {} {}", "•".dimmed(), note.dimmed());
                                }

                            }

                            println!();
                            println!("−–—< {} — {} {} >—–−", age.title.blue(), settings.year_name.yellow(), age.end.to_string().yellow());

                        }

                    }

                }

                _ => {

                    println!();
                    hey.hey(&format!("You can't use {} out of a timeline.", "olam timeline".yellow()));
                    hey.tip(&format!("Create and immerse in a timeline {}, to use this", "olam add \"my timeline\" -i".yellow()));
                    println!();

                }

            }

            println!();

            Ok(())

        }



        
    }

}
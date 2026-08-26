// Copyright (C) 2025 hello_wxs <hello_wxs@zohomail.com>
// SPDX-License-Identifier: GPL-3.0-or-later

use rand_pcg::rand_core::{Rng, SeedableRng};
use sha2::Digest;

/// The thinking state of the buddy.
/// Decides the current thought of the buddy.
#[derive(Debug)]
enum Think {
    /// The main thinking state.
    Main,
    /// The dozy state (sleepy).
    Dozy,
    /// The hint state (providing helpful information).
    Hint,
    /// The taunt state (providing humorous comments).
    Taunt,
}

/// The idea struct representing the current state and sayings of the buddy.
/// Includes the current thinking state and the index of the current saying, if any.
#[derive(Debug)]
struct Idea {
    /// The current thinking state of the buddy.
    think: Think,
    /// The index of the current saying, if any.
    say: Option<usize>,
}

/// Images struct holding the buddy's images.
/// Stores the the main, dozy, hint, and taunt images.
#[derive(Debug)]
struct Images {
    /// The main image of the buddy.
    main: String,
    /// The dozy image of the buddy.
    dozy: String,
    /// The hint image of the buddy.
    hint: String,
    /// The taunt image of the buddy.
    taunt: String,
}

/// Helper struct for holding the buddy's sayings.
/// Stores the main, dozy, hint, and taunt sayings.
#[derive(Debug)]
struct Say {
    /// The main sayings of the buddy.
    main: Vec<String>,
    /// The dozy sayings of the buddy.
    dozy: Vec<String>,
    /// The hint sayings of the buddy.
    hint: Vec<String>,
    /// The taunt sayings of the buddy.
    taunt: Vec<String>,
}

/// Holds the traits of the buddy.
/// Stores the atience, wisdom, and sarcasm traits.
#[derive(Debug)]
struct Traits {
    /// The atience trait of the buddy.
    atience: u64,
    /// The wisdom trait of the buddy.
    wisdom: u64,
    /// The sarcasm trait of the buddy.
    sarcasm: u64,
}

/// A Buddy from user_name having a name and traits, with images and sayings.
/// You can use `Buddy::tire`, `Buddy::taunt`, `Buddy::remind` to get a random state.
/// Besides, you can use `Buddy::get_name` to get the buddy's name.
#[derive(Debug)]
pub struct Buddy {
    /// The name of the buddy.
    name: String,
    /// The current idea of the buddy.
    idea: Idea,
    /// The images of the buddy.
    images: Images,
    /// The color of the buddy.
    color: ratatui_core::style::Color,
    /// The sayings of the buddy.
    say: Say,
    /// The traits of the buddy.
    traits: Traits,
    /// Whether the buddy is shiny.
    shiny: bool,
    /// The act interval of the buddy, in seconds since the Unix epoch.
    act_interval: u64,
    /// Latest act time, in seconds since the Unix epoch.
    latest_act_time: u64,
    /// The random number generator used by the buddy.
    rng: rand_pcg::Pcg64,
}

impl Buddy {
    /// Creates a new Buddy from user_name having a name and traits, with images and sayings.
    /// The type and traits are determined by the user's name and a random assest is chosen.
    ///
    /// # Arguments
    ///
    /// * `user_name` - The user's name used to seed the random number generator.
    /// * `buddy_name` - The name of the buddy, if provided. Otherwise, a random name is chosen from the assests.
    ///
    /// # Examples
    ///
    /// ```
    /// let buddy = Buddy::new("user".to_string(), Some("Hei".to_string()), 60);
    /// ```
    pub fn new(user_name: String, buddy_name: Option<String>, act_interval: u64) -> Self {
        // Make buddy assest and shiny from user_name
        let (assest, shiny) = {
            let mut hasher = sha2::Sha256::new();
            hasher.update(
                (user_name + " (yezi-buddy by hello_wxs <hello_wxs@zohomail.com>)").as_bytes(),
            );
            let hash = hasher.finalize();
            let mut seed = [0u8; 32];
            seed.copy_from_slice(&hash[0..32]);
            let mut rng = rand_pcg::Pcg64::from_seed(seed);

            // Select a random buddy assest and is_shiny
            (
                &crate::assest::BUDDIES
                    [(rng.next_u64() % crate::assest::BUDDIES.len() as u64) as usize],
                rng.next_u64() % 64 == 0,
            )
        };

        // Make rng randomly
        let mut rng = rand_pcg::Pcg64::from_seed(rand::random());

        // Create a new Buddy
        Self {
            name: buddy_name.unwrap_or(assest.name.to_string()),
            idea: Idea {
                think: Think::Main,
                say: None,
            },
            images: Images {
                main: assest.image.main.to_string(),
                dozy: assest.image.dozy.to_string(),
                hint: assest.image.hint.to_string(),
                taunt: assest.image.taunt.to_string(),
            },
            color: assest.color,
            traits: Traits {
                atience: rng.next_u64(),
                wisdom: rng.next_u64(),
                sarcasm: rng.next_u64(),
            },
            say: Say {
                main: assest.say.main.split('\n').map(|s| s.to_string()).collect(),
                dozy: assest.say.dozy.split('\n').map(|s| s.to_string()).collect(),
                hint: assest.say.hint.split('\n').map(|s| s.to_string()).collect(),
                taunt: assest
                    .say
                    .taunt
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect(),
            },
            shiny,
            act_interval,
            latest_act_time: 0,
            rng,
        }
    }
    /// Check if the interval since the last tick has elapsed
    fn should_act(&self) -> Result<bool, std::time::SystemTimeError> {
        let current_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        Ok(current_time - self.latest_act_time >= self.act_interval)
    }
    /// Finishing the buddy's act.
    fn finish_act(&mut self) -> Result<(), std::time::SystemTimeError> {
        self.latest_act_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        Ok(())
    }
    /// Change the saying of the buddy to a random one.
    /// It has a 50% chance of setting `say` to `None`.
    fn change_say(&mut self) {
        if self.rng.next_u64().is_multiple_of(2) {
            self.idea.say = Some(self.rng.next_u64() as usize % self.say.main.len());
        } else {
            self.idea.say = None;
        }
    }
    /// Change the idea of the buddy to tired randomly.
    pub fn try_tire(&mut self) -> Result<(), std::time::SystemTimeError> {
        if self.should_act()? {
            if self.rng.next_u64() <= self.traits.atience {
                self.idea.think = Think::Dozy;
            } else {
                self.idea.think = Think::Main;
            }
            self.change_say();
            self.finish_act()?;
        }
        Ok(())
    }
    /// Change the idea of the buddy to taunt randomly.
    pub fn try_taunt(&mut self) -> Result<(), std::time::SystemTimeError> {
        if self.should_act()? {
            if self.rng.next_u64() <= self.traits.sarcasm {
                self.idea.think = Think::Taunt;
            } else {
                self.idea.think = Think::Main;
            }
            self.change_say();
            self.finish_act()?;
        }
        Ok(())
    }
    /// Change the idea of the buddy to reminding randomly.
    pub fn remind(&mut self) -> Result<(), std::time::SystemTimeError> {
        if self.should_act()? {
            if self.rng.next_u64() <= self.traits.wisdom {
                self.idea.think = Think::Hint;
            } else {
                self.idea.think = Think::Main;
            }
            self.change_say();
            self.finish_act()?;
        }
        Ok(())
    }
    /// Returns the name of the buddy.
    pub fn get_name(&self) -> &str {
        &self.name
    }
    /// Returns the current saying of the buddy.
    pub fn get_saying(&self) -> String {
        match self.idea.say {
            Some(index) => match self.idea.think {
                Think::Main => self.say.main[index].clone(),
                Think::Dozy => self.say.dozy[index].clone(),
                Think::Taunt => self.say.taunt[index].clone(),
                Think::Hint => self.say.hint[index].clone(),
            },
            None => String::new(),
        }
    }
    /// Returns the current image of the buddy.
    pub fn get_image(&self) -> String {
        match self.idea.think {
            Think::Main => self.images.main.clone(),
            Think::Dozy => self.images.dozy.clone(),
            Think::Taunt => self.images.taunt.clone(),
            Think::Hint => self.images.hint.clone(),
        }
    }
    /// Returns the color of the buddy.
    pub fn get_color(&self) -> ratatui_core::style::Color {
        if self.shiny {
            ratatui_core::style::Color::Rgb(255, 255, 0)
        } else {
            self.color
        }
    }
}

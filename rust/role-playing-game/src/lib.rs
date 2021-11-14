// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::cmp::Ordering::Less;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn new(level: u32) -> Self {
        Self {
            health: 100,
            mana: if level < 10 { None } else { Some(100) },
            level,
        }
    }

    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(self.level)),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(mut mana) => match mana.cmp(&mana_cost) {
                Less => 0,
                _ => {
                    self.mana = Some(mana - mana_cost);
                    mana_cost * 2
                }
            },
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
        }
    }
}

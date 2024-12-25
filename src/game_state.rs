use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Character {
    pub name: String,
    pub class: String,
    pub level: u32,
    pub experience: f32,
    pub health: f32,
    pub mana: f32,
    pub stats: Stats,
    pub equipment: Equipment,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Stats {
    pub strength: u32,
    pub agility: u32,
    pub intelligence: u32,
    pub stamina: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Equipment {
    pub items: HashMap<String, Item>,
    pub average_item_level: f32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub level: u32,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Inventory {
    pub items: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameState {
    pub character: Character,
    pub stats: Stats,
    pub inventory: Inventory,
    pub equipment: Equipment,
    pub quests: Vec<String>,
    pub reputation: HashMap<String, f32>,
    pub achievements: HashMap<String, bool>,
    pub currency: HashMap<String, u32>,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState {
    pub fn new() -> Self {
        Self {
            character: Character {
                name: "Dummy Character".to_string(),
                class: "Warrior".to_string(),
                level: 60,
                experience: 0.0,
                health: 100.0,
                mana: 100.0,
                stats: Stats {
                    strength: 100,
                    agility: 50,
                    intelligence: 30,
                    stamina: 200,
                },
                equipment: Equipment::default(),
            },
            stats: Stats {
                strength: 100,
                agility: 50,
                intelligence: 30,
                stamina: 200,
            },
            inventory: Inventory {
                items: vec![
                    "Potion".to_string(),
                    "Food".to_string(),
                ],
            },
            equipment: Equipment::default(),
            quests: vec![
                "Dummy Quest 1".to_string(),
                "Dummy Quest 2".to_string(),
            ],
            reputation: HashMap::from([
                ("Faction 1".to_string(), 3000.0),
                ("Faction 2".to_string(), 9000.0),
            ]),
            achievements: HashMap::from([
                ("Achievement 1".to_string(), true),
                ("Achievement 2".to_string(), false),
            ]),
            currency: HashMap::from([
                ("Gold".to_string(), 1000),
                ("Silver".to_string(), 50),
            ]),
        }
    }
    
    #[allow(dead_code)]
    pub fn update_reputation(&mut self, faction: String, value: f32) -> Result<(), Box<dyn std::error::Error>> {
        self.reputation.insert(faction, value);
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn update_equipment(&mut self, slot: String, item: Item) -> Result<(), Box<dyn std::error::Error>> {
        self.character.equipment.items.insert(slot, item);
        self.character.equipment.average_item_level = self.character.equipment.items.values()
            .map(|item| item.level as f32)
            .sum::<f32>() / self.character.equipment.items.len() as f32;
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn update_stats(&mut self, stats: Stats) -> Result<(), Box<dyn std::error::Error>> {
        self.stats = stats;
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn update_level(&mut self, level: u32, exp: f32) -> Result<(), Box<dyn std::error::Error>> {
        self.character.level = level;
        self.character.experience = exp;
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn update_resources(&mut self, health: f32, mana: f32) -> Result<(), Box<dyn std::error::Error>> {
        self.character.health = health;
        self.character.mana = mana;
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn update_character(&mut self, name: String, class: String) -> Result<(), Box<dyn std::error::Error>> {
        self.character.name = name;
        self.character.class = class;
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn add_quest(&mut self, quest: String) -> Result<(), Box<dyn std::error::Error>> {
        self.quests.push(quest);
        Ok(())
    }
    
    #[allow(dead_code)]
    pub fn update_currency(&mut self, currency: String, amount: u32) -> Result<(), Box<dyn std::error::Error>> {
        self.currency.insert(currency, amount);
        Ok(())
    }
} 
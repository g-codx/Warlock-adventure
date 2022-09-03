use std::fs::File;
use ron::de::from_reader;
use crate::prelude::*;
use serde::Deserialize;
use crate::CardAction::Special;
use crate::EnemyType::Lizard;
use crate::EntityType::SkillCard;

pub struct TemplatePlugin;

impl Plugin for TemplatePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, Self::load)
            .init_resource::<Templates>();
    }
}

impl TemplatePlugin {
    fn load(commands: Commands) {
        Templates::load().shuffle(commands);
    }
}


#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub id: usize,
    pub entity_type: EntityType,
    pub level: usize,
    pub name: String,
    pub sprite_index: Option<usize>,
    pub mana_cost: Option<usize>,
    pub value: Option<usize>,
    pub rounds: Option<usize>,
    pub card_action: Option<CardAction>,
    pub sub_sprite_index: Option<usize>,
    pub health: Option<usize>,
    pub attack: Option<usize>,
    pub defense: Option<usize>,
}

#[derive(Clone, Deserialize, Debug, PartialEq, Eq)]
pub enum EntityType {
    Enemy,
    SkillCard,
    Item,
}

#[derive(Clone, Deserialize, Debug, Default)]
pub struct Templates {
    pub entities: Vec<Template>,
}


impl Templates {
    pub fn load() -> Self {
        let file = File::open("assets/Templates.ron").expect("Failed opening file");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn shuffle(&self, mut commands: Commands) {
        let mut skill_cards = Vec::new();
        let mut enemies = Vec::new();
        let mut items = Vec::new();
        self
            .entities
            .iter()
            .for_each(|t| {
                match &t.entity_type {
                    SkillCard => {
                        skill_cards.push(t.clone());
                    },
                    EntityType::Enemy => {
                        enemies.push(t.clone());
                    },
                    EntityType::Item => {
                        items.push(t.clone())
                    }
                    _ => {}
                }
            });

        let types = self.entities
            .iter()
            .map(|t| (t.id, t.entity_type.clone(), t.sprite_index))
            .collect::<Vec<(usize, EntityType, Option<usize>)>>();

        commands.insert_resource(TemplateStorage {
            skill_cards,
            enemies,
            items,
            types
        });
    }
}


pub struct TemplateStorage {
    pub skill_cards: Vec<Template>,
    pub enemies: Vec<Template>,
    pub items: Vec<Template>,
    pub types: Vec<(usize, EntityType, Option<usize>)>,
}


impl TemplateStorage {
    //deprecated
    pub fn roll_card(&self, lvl: usize) -> CardView {
        let card: Vec<CardView> = self.skill_cards
            .iter()
            .filter(|t| t.level == lvl)
            .map(|t| CardView {
                id: t.id,
                level: t.level
            })
            .collect();

        let size = card.len();
        let roll = thread_rng().gen_range(0..size);

        *card.get(roll).unwrap()
    }

    //deprecated
    pub fn roll_item(&self, lvl: usize) -> ItemView {
        let items: Vec<ItemView> = self.items
            .iter()
            .filter(|t| t.level == lvl)
            .map(|t| ItemView {
                id: t.id,
                level: t.level,
            })
            .collect();

        let size = items.len();
        let roll = thread_rng().gen_range(0..size);

        *items.get(roll).unwrap()
    }

    pub fn get_card(&self, id: usize) -> CardView {
        self.skill_cards
            .iter()
            .filter(|t| t.id == id)
            .map(|t| CardView {
                id: t.id,
                level: t.level
            })
            .next()
            .unwrap()
    }

    pub fn get_enemy(&self, enemy_type: EnemyType) -> Option<&Template> {
        match enemy_type {
            Lizard => self.enemies.iter().filter(|e| e.id == 15).next(),
            EnemyType::Medusa => self.enemies.iter().filter(|e| e.id == 16).next(),
            EnemyType::SmallDragon => self.enemies.iter().filter(|e| e.id == 17).next(),
            EnemyType::Gin => self.enemies.iter().filter(|e| e.id == 18).next(),
            EnemyType::BigDragon => self.enemies.iter().filter(|e| e.id == 19).next(),
            EnemyType::Demon => self.enemies.iter().filter(|e| e.id == 20).next(),
        }
    }
}


#[derive(Clone, Deserialize, Debug, Inspectable, Eq, PartialEq)]
pub enum CardAction {
    AttackBuff,
    DefenceBuff,
    HealthBuff,
    ManaBuff,
    Special,
}

impl Default for CardAction {
    fn default() -> Self {
        Special
    }
}

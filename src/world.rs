
use std::ops::Deref;
use crate::player::Player;
pub use crate::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system_to_stage(
                StartupStage::PreStartup, Self::init_item_pull,
            );
    }
}

impl WorldPlugin {
    fn init_item_pull(commands: Commands) {
        ItemPull::create_item_pull(commands);
    }
}


#[derive(Component, Default)]
pub struct ItemPull {
    low_lvl: Vec<usize>,
    middle_lvl: Vec<usize>,
    high_lvl: Vec<usize>,
}

#[derive(Component)]
pub struct WorldEventMarker;

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum WorldEventType {
    Camp,
    Ruins,
    Altar
}

#[derive(Component)]
pub struct WorldEvent {
    pub event_type: WorldEventType,
    pub lvl: usize,
    pub is_visited: bool,
}

impl ItemPull {
    pub fn create_item_pull(mut commands: Commands) {
        let mut pull = Self {
            low_lvl: vec![2, 3, 4, 6, 9, 10, 24, 25, 26],//start with 1,5
            middle_lvl: vec![7, 8, 11, 12, 27, 28, 29, 30],
            high_lvl: vec![13, 14, 15, 16, 21, 22, 23],
        };
        pull.shuffle();

        commands.insert_resource(pull);
    }

    fn shuffle(&mut self) {
        self.low_lvl.shuffle(&mut thread_rng());
        self.middle_lvl.shuffle(&mut thread_rng());
        self.high_lvl.shuffle(&mut thread_rng());
    }

    pub fn get_item(&mut self, lvl: usize) -> Option<usize> {
        match lvl {
            1 => self.low_lvl.pop(),
            2 => self.middle_lvl.pop(),
            3 => self.high_lvl.pop(),
            _ => None
        }
    }
}

pub fn add_reward(reward: &Reward, player: &mut Player) {
    match reward.entity_type {
        EntityType::SkillCard => player.add_in_deck(reward.item_id, reward.item_lvl),
        EntityType::Item => player.add_in_bag(reward.item_id, reward.item_lvl),
        _ => {}
    }
}

pub struct Reward {
    pub item_id: usize,
    pub entity_type: EntityType,
    pub sprite_index: Option<usize>,
    pub item_lvl: usize
}

pub fn get_reward_template(
    template_storage: &Res<TemplateStorage>,
    mut lvl: usize,
    items: &mut ItemPull
) -> Option<Reward> {

    if lvl == 0 {
        lvl = thread_rng().gen_range(1..=3);
    }

    let template = if let Some(id) = items.get_item(lvl) {
        template_storage.types
            .iter()
            .find(|t| t.0 == id)
    } else {
        None
    };

    template.map(|template_| Reward {
            item_id: template_.0,
            entity_type: template_.1.clone(),
            sprite_index: template_.2,
            item_lvl: lvl
        })
}

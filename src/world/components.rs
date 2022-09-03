use crate::CardAction::Special;
use crate::prelude::*;


#[derive(Debug)]
pub struct UpdateEvent(pub bool);

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum WorldEventType {
    Camp,
    Ruins,
    Altar
}

#[derive(Component)]
pub struct Backpack;

#[derive(Component)]
pub struct BagExit;

#[derive(Component)]
pub struct SkillPackExit;

#[derive(Component)]
pub struct SkillPack;

#[derive(Component)]
pub struct Next;

#[derive(Component)]
pub struct WorldInterface;

#[derive(Component)]
pub struct BagInterface;

#[derive(Component)]
pub struct SkillPackInterface;

#[derive(Component)]
pub struct CombatDeckSpell;

#[derive(Component)]
pub struct WorldReward;

#[derive(Component)]
pub struct AcceptRewardButton;

#[derive(Component)]
pub struct WorldEventMarker;

#[derive(Component)]
pub struct Town;

#[derive(Component)]
pub struct MoveDiceText;

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EncounterTracker {
    pub timer: Timer,
}

#[derive(Clone, Copy, Default, Inspectable)]
pub struct ItemView {
    pub id: usize,
    pub level: usize,
}

#[derive(Clone, Copy, Default, Inspectable, Component)]
pub struct CardView {
    pub id: usize,
    pub level: usize,
}

#[derive(Component, Debug)]
pub struct MoveDice {
    pub value: isize,
    pub can_roll: bool,
}

#[derive(Component)]
pub struct WorldEvent {
    pub event_type: WorldEventType,
    pub lvl: usize,
    pub is_visited: bool,
}

#[derive(Component, Default)]
pub struct ItemPull {
    low_lvl: Vec<usize>,
    middle_lvl: Vec<usize>,
    high_lvl: Vec<usize>,
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

#[derive(Component, Inspectable)]
pub struct Item {
    pub id: usize,
    pub lvl: usize,
    pub name: String,
    pub sprite_index: Option<usize>,
    pub value: usize,
    pub buff_type: CardAction
}

#[derive(Component, Inspectable, Clone)]
pub struct Card {
    pub id: usize,
    pub mana_cost: usize,
    pub name: String,
    pub value: usize,
    pub card_action: CardAction,
    pub sprite_index: usize,
    pub rounds: usize,
    pub is_used: bool,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            id: 0,
            mana_cost: 0,
            name: "".to_string(),
            value: 0,
            card_action: Special,
            sprite_index: 0,
            rounds: 1,
            is_used: false,
        }
    }
}

pub struct Reward {
    pub item_id: usize,
    pub entity_type: EntityType,
    pub sprite_index: Option<usize>,
    pub item_lvl: usize
}
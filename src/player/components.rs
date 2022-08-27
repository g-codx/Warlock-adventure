use bevy::utils::HashMap;
use crate::CardAction::Special;
use crate::prelude::*;


#[derive(Debug)]
pub struct UpdateEvent(pub bool);

#[derive(Component, Debug)]
pub struct MoveDice {
    pub value: isize,
    pub can_roll: bool,
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

#[derive(Component, Inspectable, Default)]
pub struct Player {
    pub is_selected: bool,
    pub combat_deck: Vec<CardView>,
    pub deck: Vec<CardView>,
    pub items_bag: Vec<ItemView>,
    pub item_build: ItemBuild
}

impl Player {
    pub fn add_spell(&mut self, spell: CardView) {
        self.combat_deck.push(spell)
    }

     pub fn remove(&mut self, id: usize) {
        let index = self.combat_deck
            .iter()
            .position(|s| s.id == id)
            .unwrap();
        self.combat_deck.remove(index);
    }

    pub fn can_add(&self, lvl: usize) -> bool {
        match lvl {
            1 => self.spell_count(lvl) < 3,
            2 => self.spell_count(lvl) < 2,
            _ => self.spell_count(lvl) == 0
        }
    }

    fn spell_count(&self, lvl: usize) -> usize {
        self.combat_deck
            .iter()
            .filter(|s| s.level == lvl)
            .count()
    }

    pub fn add_in_deck(&mut self, id: usize, level: usize) {
        let card_view = CardView {
            id,
            level
        };
        self.deck.push(card_view);
    }

    pub fn add_in_bag(&mut self, id: usize, level: usize) {
        let item_view = ItemView {
            id,
            level
        };
        self.items_bag.push(item_view);
    }
}

#[derive(Component, Inspectable, Default)]
pub struct ItemBuild {
    pub defense: Option<(usize, usize)>,//(id, value)
    pub attack: Option<(usize, usize)>,
    pub mana: Option<(usize, usize)>,
    pub health: Option<(usize, usize)>,
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

#[derive(Component, Inspectable)]
pub struct Item {
    pub id: usize,
    pub lvl: usize,
    pub name: String,
    pub sprite_index: Option<usize>,
    pub value: usize,
    pub buff_type: CardAction
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

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EncounterTracker {
    pub timer: Timer,
}

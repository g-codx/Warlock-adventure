use crate::prelude::*;

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

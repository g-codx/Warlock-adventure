use crate::EnemyType::{BigDragon, Demon, Gin, Lizard, Medusa, SmallDragon};
use crate::player::Card;
use crate::prelude::*;

#[derive(Component, Inspectable)]
pub struct CombatStats {
    pub health: isize,
    pub attack: isize,
    pub defense: isize,
    pub max_health: isize,
}

#[derive(Component, Inspectable)]
pub struct Enemy {
    pub enemy_type: EnemyType,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct CombatManager {
    pub round: usize,
    pub damage: isize,
    pub mana_poll: isize,
    pub defense: isize,
    pub permanent_damage_buff: isize,
    pub permanent_defense_buff: isize,
    pub permanent_mana_buff: isize,
    pub can_roll_mana: bool,
    pub can_roll_attack: bool,
    pub skip_round: bool,
    pub enemy_skip_round: bool,
    pub timer: Timer,
    pub enemy_death: bool,
    pub player_death: bool,
    pub enemy_lvl: usize,
}

impl CombatManager {
    pub fn if_can_cast(&mut self, mana_cost: usize, is_used: bool) -> bool {
        println!("{}", is_used);
        if (self.mana_poll - mana_cost as isize >= 0) && !is_used {
            self.mana_poll = self.mana_poll - mana_cost as isize;
            println!("SUCCESS! Mana pull = {}", self.mana_poll);
            true
        } else {
            println!("FAIL! Mana pull = {}, Mana cost = {}", self.mana_poll, mana_cost);
            false
        }
    }

    pub fn is_range_buff(card: &Card) -> bool {
        card.rounds == 0
    }

    pub fn print(&self) {
        println!("round:{}, damage:{}, mana poll{}, permanent_damage_buff:{},\
         permanent_defense_buff:{}, permanent_mana_buff:{}",
                 self.round,self.damage,self.mana_poll,self.permanent_damage_buff,
                 self.permanent_defense_buff,self.permanent_mana_buff);
    }
}

impl Default for CombatManager {
    fn default() -> Self {
        Self {
            round: 1,
            damage: 0,
            mana_poll: 0,
            defense: 0,
            permanent_damage_buff: 0,
            permanent_defense_buff: 0,
            permanent_mana_buff: 0,
            can_roll_mana: true,
            can_roll_attack: true,
            skip_round: false,
            enemy_skip_round: false,
            timer: Timer::from_seconds(2., true),
            enemy_death: false,
            player_death: false,
            enemy_lvl: 0,
        }
    }
}

#[derive(Debug)]
pub struct FightEvent {
    pub target: Entity,
    pub damage_amount: isize,
    pub next_state: CombatState,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Inspectable, Component)]
pub enum EnemyType {
    Lizard,
    Medusa,
    Gin,
    SmallDragon,
    BigDragon,
    Demon
}

impl EnemyType {
    pub fn get_type(char_type: char) -> EnemyType {
        match char_type {
            'm' => Lizard,
            'M' => Medusa,
            'd' => SmallDragon,
            'j' => Gin,
            'D' => BigDragon,
            's' => Demon,
            _ => Demon
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct EncounterEvent(pub EnemyType);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum CombatState {
    PreState,
    PlayerTurn,
    PlayerAttack,
    EnemyTurn,
    EnemyAttack,
    EnemyDeath,
    Finalize,
    End,
}


#[derive(Component)]
pub struct AttackDice;

#[derive(Component)]
pub struct ManaDice;

#[derive(Component)]
pub struct AttackButton;

#[derive(Component)]
pub struct HeroSpellButton;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct AttackText;

#[derive(Component)]
pub struct DefenseText;

#[derive(Component)]
pub struct ManaText;

#[derive(Component)]
pub struct RoundText;

#[derive(Component)]
pub struct PlayerMarker;

#[derive(Component)]
pub struct EnemyMarker;

#[derive(Component, Inspectable, Default)]
pub struct Selected {
    pub selected: bool,
}

#[derive(Component)]
pub struct CombatEndButton;

#[derive(Component)]
pub struct BottomItems;

#[derive(Component)]
pub struct TopItems;

#[derive(Component)]
pub struct Battleground;


use std::cell::RefCell;
use std::rc::Rc;

use super::classes::Attacker;

pub enum DamageType {
    Default,
    Blunt,
    Slash,
    Stab,
    Bullet,
    Blast,
    Magic,
    Burn,
    Shock,
}

pub struct Damage {
    pub amount: i32,
    pub damage_type: DamageType,
    pub dodgeable: bool,
    pub defensible: bool,
    pub source: Rc<RefCell<dyn Attacker>>,
}

pub enum DamageResult {
    Default(i32),
    Success(i32),
    Dodge(i32),
    Defend(i32),
    Miss(i32),
}

pub trait Damageable {
    fn process_damage(&mut self, damage: &Damage) -> DamageResult;
    fn get_hp(&self) -> i32;
    fn set_hp(&mut self, amount: i32);
}

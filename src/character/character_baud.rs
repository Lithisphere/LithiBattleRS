use crate::classes::classes::{Attacker, Entity};
use crate::classes::damage::{Damage, DamageResult, DamageType, Damageable};
use crate::classes::moves::Move;
use std::cell::RefCell;
use std::rc::Rc;
struct MovePunch {}

impl MovePunch {
    fn new() -> Self {
        Self {}
    }
}
impl Move for MovePunch {
    fn use_move(
        &mut self,
        user: Rc<RefCell<dyn Attacker>>,
        target: &mut dyn Damageable,
    ) -> DamageResult {
        let dmg = Damage {
            amount: 80,
            damage_type: DamageType::Blunt,
            dodgeable: true,
            defensible: true,
            source: user,
        };
        target.process_damage(&dmg)
    }
}
pub struct CharacterBaud<'a> {
    HP: i32,
    NAME: &'a str,
    STR: i32,
    DEX: i32,
    DEF: i32,
    SPD: i32,
    moves: Vec<Rc<RefCell<dyn Move>>>,
}

impl CharacterBaud<'_> {
    pub fn new() -> Self {
        let a: Vec<Rc<RefCell<dyn Move>>> = vec![Rc::new(RefCell::new(MovePunch::new()))];
        Self {
            HP: 4000,
            NAME: "Baud",
            STR: 80,
            DEX: 80,
            DEF: 30,
            SPD: 100,
            moves: a,
        }
    }
}

impl Damageable for CharacterBaud<'_> {
    fn process_damage(&mut self, damage: &Damage) -> DamageResult {
        let remaining_damage = damage.amount - self.DEF;

        if remaining_damage <= 0 {
            return DamageResult::Defend(damage.amount);
        }
        self.HP -= remaining_damage;
        DamageResult::Success(remaining_damage)
    }
    fn get_hp(&mut self) -> i32 {
        self.HP
    }
    fn set_hp(&mut self, amount: i32) {
        self.HP = amount
    }
}

impl Entity for CharacterBaud<'_> {
    fn get_name(&mut self) -> &str {
        self.NAME
    }

    fn get_str(&mut self) -> i32 {
        self.STR
    }
    fn set_str(&mut self, amount: i32) {
        self.STR = amount
    }

    fn get_dex(&mut self) -> i32 {
        self.DEX
    }
    fn set_dex(&mut self, amount: i32) {
        self.DEX = amount
    }

    fn get_def(&mut self) -> i32 {
        self.DEF
    }
    fn set_def(&mut self, amount: i32) {
        self.DEF = amount
    }

    fn get_spd(&mut self) -> i32 {
        self.SPD
    }
    fn set_spd(&mut self, amount: i32) {
        self.SPD = amount
    }
}

impl Attacker for CharacterBaud<'_> {
    fn get_moves(&mut self) -> Vec<Rc<RefCell<dyn Move>>> {
        self.moves.clone()
    }
}

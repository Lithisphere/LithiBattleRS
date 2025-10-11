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
    fn get_name(&mut self) -> &str {
        "Punch"
    }
}

struct MoveLaserGun {}

impl MoveLaserGun {
    fn new() -> Self {
        Self {}
    }
}
impl Move for MoveLaserGun {
    fn use_move(
        &mut self,
        user: Rc<RefCell<dyn Attacker>>,
        target: &mut dyn Damageable,
    ) -> DamageResult {
        let dmg = Damage {
            amount: 160,
            damage_type: DamageType::Burn,
            dodgeable: true,
            defensible: true,
            source: user,
        };
        target.process_damage(&dmg)
    }
    fn get_name(&mut self) -> &str {
        "Laser Gun"
    }
}
pub struct CharacterBaud {
    HP: i32,
    NAME: String,
    STR: i32,
    DEX: i32,
    DEF: i32,
    SPD: i32,
    moves: Vec<Rc<RefCell<dyn Move>>>,
}

impl<'a> CharacterBaud {
    pub fn new() -> Self {
        let a: Vec<Rc<RefCell<dyn Move>>> = vec![
            Rc::new(RefCell::new(MovePunch::new())),
            Rc::new(RefCell::new(MoveLaserGun::new())),
        ];
        Self {
            HP: 4000,
            NAME: String::from("Baud"),
            STR: 80,
            DEX: 80,
            DEF: 30,
            SPD: 100,
            moves: a,
        }
    }
}

impl Damageable for CharacterBaud {
    fn process_damage(&mut self, damage: &Damage) -> DamageResult {
        let remaining_damage = damage.amount - self.DEF;

        if remaining_damage <= 0 {
            return DamageResult::Defend(damage.amount);
        }
        self.HP -= remaining_damage;
        DamageResult::Success(remaining_damage)
    }
    fn get_hp(&self) -> i32 {
        self.HP
    }
    fn set_hp(&mut self, amount: i32) {
        self.HP = amount
    }
}

impl<'a> Entity for CharacterBaud {
    fn get_name(&self) -> &str {
        &self.NAME
    }

    fn set_name(&mut self, name: &str) {
        self.NAME.clear();
        self.NAME.push_str(name);
    }

    fn get_str(&self) -> i32 {
        self.STR
    }
    fn set_str(&mut self, amount: i32) {
        self.STR = amount
    }

    fn get_dex(&self) -> i32 {
        self.DEX
    }
    fn set_dex(&mut self, amount: i32) {
        self.DEX = amount
    }

    fn get_def(&self) -> i32 {
        self.DEF
    }
    fn set_def(&mut self, amount: i32) {
        self.DEF = amount
    }

    fn get_spd(&self) -> i32 {
        self.SPD
    }
    fn set_spd(&mut self, amount: i32) {
        self.SPD = amount
    }
    fn to_display(&self) -> String {
        format!(
            "{} HP:{}, [STR:{}, DEX:{}, DEF:{}, SPD:{}]",
            self.get_name(),
            self.get_hp(),
            self.get_str(),
            self.get_dex(),
            self.get_def(),
            self.get_spd()
        )
    }
}

impl Attacker for CharacterBaud {
    fn get_moves(&self) -> Vec<Rc<RefCell<dyn Move>>> {
        self.moves.clone()
    }
}

// impl CharacterBaud {
//     fn as_damageable(self: Box<Self>) -> Box<dyn Damageable> {
//         self
//     }
// }

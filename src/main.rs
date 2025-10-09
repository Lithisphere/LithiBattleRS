mod character;
mod classes;
use std::rc::Rc;

use std::cell::RefCell;

use character::character_baud::CharacterBaud;
use classes::damage::{Damage, DamageType, Damageable};
fn main() {
    let mut baud = Rc::new(RefCell::new(CharacterBaud::new()));
    let dmg = Damage {
        amount: 80,
        damage_type: DamageType::Bullet,
        dodgeable: false,
        defensible: false,
        source: baud.clone(),
    };

    println!("{}", baud.borrow_mut().get_hp());
    baud.borrow_mut().process_damage(&dmg);
    println!("{}", baud.borrow_mut().get_hp());
}

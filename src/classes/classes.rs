use super::damage::Damageable;
use super::moves::Move;
use std::cell::RefCell;
use std::rc::Rc;
pub trait Entity: Damageable {
    fn get_name(&mut self) -> &str;

    fn get_str(&mut self) -> i32;
    fn set_str(&mut self, amount: i32);

    fn get_dex(&mut self) -> i32;
    fn set_dex(&mut self, amount: i32);

    fn get_def(&mut self) -> i32;
    fn set_def(&mut self, amount: i32);

    fn get_spd(&mut self) -> i32;
    fn set_spd(&mut self, amount: i32);
}

pub trait Attacker: Entity {
    fn get_moves(&mut self) -> Vec<Rc<RefCell<dyn Move>>>;
}

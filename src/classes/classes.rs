use super::damage::Damageable;
use super::moves::Move;
use core::fmt;
use std::cell::RefCell;
use std::rc::Rc;

pub trait Entity: Damageable {
    fn get_name(&self) -> &str;
    fn set_name<'a>(&mut self, name: &str);

    fn get_str(&self) -> i32;
    fn set_str(&mut self, amount: i32);

    fn get_dex(&self) -> i32;
    fn set_dex(&mut self, amount: i32);

    fn get_def(&self) -> i32;
    fn set_def(&mut self, amount: i32);

    fn get_spd(&self) -> i32;
    fn set_spd(&mut self, amount: i32);

    fn to_display(&self) -> String;
}
// pub trait Damageable {
//     fn hp(&self) -> i32;
//     fn set_hp(&mut self, v: i32);
// }

// Object-safe shim that exposes &mut dyn Damageable
pub trait AsDamageable {
    fn as_damageable_mut(&mut self) -> &mut dyn Damageable;
}

// Blanket impl: works for any concrete type that is Damageable
impl<T: Damageable + Sized> AsDamageable for T {
    fn as_damageable_mut(&mut self) -> &mut dyn Damageable {
        self
    }
}

pub trait Attacker: Entity + AsDamageable {
    fn get_moves(&self) -> Vec<Rc<RefCell<dyn Move>>>;

    // Go straight to &mut dyn Damageable as well
    fn as_damageable(&mut self) -> &mut dyn Damageable {
        self.as_damageable_mut()
    }
}

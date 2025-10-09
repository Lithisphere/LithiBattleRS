use std::cell::RefCell;
use std::rc::Rc;

use crate::classes::classes::Attacker;

use super::damage::{DamageResult, Damageable};
pub trait Move {
    fn use_move(
        &mut self,
        user: Rc<RefCell<dyn Attacker>>,
        target: &mut dyn Damageable,
    ) -> DamageResult;
}

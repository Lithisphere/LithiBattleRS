mod character;
mod classes;
use std::alloc::GlobalAlloc;
use std::io::Write;
use std::{io, rc::Rc};

use std::cell::{RefCell, RefMut};

use character::character_baud::CharacterBaud;
use classes::damage::{Damage, DamageType, Damageable};

use crate::classes::damage::DamageResult;
use crate::classes::{
    classes::{Attacker, Entity},
    moves::Move,
};
fn main() {
    let baud = Rc::new(RefCell::new(CharacterBaud::new()));
    let duab = Rc::new(RefCell::new(CharacterBaud::new()));
    duab.borrow_mut().set_name("Duab");
    let players: Vec<Rc<RefCell<dyn Attacker>>> = vec![baud, duab];

    // let dmg = Damage {
    //     amount: 80,
    //     damage_type: DamageType::Bullet,
    //     dodgeable: false,
    //     defensible: false,
    //     source: baud.clone(),
    // };
    let mut turns = 0;
    loop {
        let turn_index: usize = turns % players.len();
        let target_index: usize = (turns + 1) % players.len();
        let mut user_input = String::new(); // Declare a mutable String to store the input
        print!("{}> ", players[turn_index].borrow().get_name());
        let flush_result = io::stdout().flush();
        match flush_result {
            Ok(_) => {}
            Err(_) => {}
        }
        let inp_result = io::stdin() // Get a handle to the standard input stream
            .read_line(&mut user_input); // Read a line of input and append it to `user_input` // Handle potential errors during input

        match inp_result {
            Ok(len) => {}
            Err(e) => {
                println!("Invalid input.");
                continue;
            }
        }
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for (i, item) in players.iter().enumerate() {
            println!("{}", item.borrow().to_display());
        }
        // The `read_line` method includes the newline character, so `trim()` is used to remove it.
        let trimmed_input = user_input.trim();
        let parsed_input: Result<i32, _> = trimmed_input.parse();
        let r#move: Rc<RefCell<dyn Move>>;
        match parsed_input {
            Ok(val) => {
                if (val as usize) < players[turn_index].borrow_mut().get_moves().len() {
                    r#move = players[turn_index].borrow_mut().get_moves()[val as usize].clone();
                } else {
                    println!("Invalid input.");
                    continue;
                }
            }
            Err(e) => {
                println!("Invalid input.");
                continue;
            }
        }
        // Prepare the user (attacker). If use_move expects &mut Rc<...>, make it mutable:
        let user_attacker: Rc<RefCell<dyn Attacker>> = players[target_index].clone();
        let mut res: DamageResult;
        // Now borrow player 0 again to act as the target and upcast to Damageable.
        match {
            let mut p0_mut = players[turn_index].borrow_mut(); // RefMut<dyn Attacker>
            let target_damageable: &mut dyn Damageable = p0_mut.as_damageable_mut();

            // Call the move. Adjust the signature here to match your trait exactly:
            let res: DamageResult = r#move
                .borrow_mut()
                .use_move(user_attacker, target_damageable);
            res
            // match res
        } {
            DamageResult::Default(_) => {}
            DamageResult::Success(dmg) => {
                println!(
                    "{} used {} and dealt {} {} damage.",
                    players[turn_index].borrow_mut().get_name(),
                    r#move.borrow_mut().get_name(),
                    players[target_index].borrow_mut().get_name(),
                    dmg
                )
            }
            DamageResult::Dodge(_) => {
                println!(
                    "{} tried to use {}, but {} dodged.",
                    players[turn_index].borrow_mut().get_name(),
                    r#move.borrow_mut().get_name(),
                    players[target_index].borrow_mut().get_name()
                )
            }
            DamageResult::Defend(_) => {
                println!(
                    "{} tried to use {}, but {} defended themselves.",
                    players[turn_index].borrow_mut().get_name(),
                    r#move.borrow_mut().get_name(),
                    players[target_index].borrow_mut().get_name()
                )
            }
            DamageResult::Miss(_) => {
                println!(
                    "{} tried to use {}, but missed.",
                    players[turn_index].borrow_mut().get_name(),
                    r#move.borrow_mut().get_name()
                )
            }
        }
        if players[target_index].borrow().get_hp() < 0 {
            println!(
                "Winner: {}! (Player {})",
                players[turn_index].borrow().get_name(),
                turn_index + 1
            );
            break;
        }
        turns += 1;
    }
}

use std::rc::Rc;
enum DamageType {
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

struct Damage {
    amount: i32,
    damage_type: DamageType,
    dodgeable: bool,
    defensible: bool,
}

enum DamageResult {
    Default(i32),
    Success(i32),
    Dodge(i32),
    Defend(i32),
    Miss(i32),
}

trait Damageable {
    fn process_damage(&mut self, damage: &Damage) -> DamageResult;
    fn get_hp(&mut self) -> i32;
    fn set_hp(&mut self, amount: i32);
}

trait Entity: Damageable {
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

trait Move {
    fn use_move(&mut self, user: &mut dyn Entity, target: &mut dyn Damageable) -> DamageResult;
}

// #[derive(Debug, Clone, Copy)]
struct MovePunch {}

impl MovePunch {
    fn new() -> Self {
        Self {}
    }
}

impl Move for MovePunch {
    fn use_move(&mut self, user: &mut dyn Entity, target: &mut dyn Damageable) -> DamageResult {
        let dmg = Damage {
            amount: 80,
            damage_type: DamageType::Blunt,
            dodgeable: true,
            defensible: true,
        };
        target.process_damage(&dmg)
    }
}

trait Attacker: Entity {
    fn get_moves(&mut self) -> Vec<Rc<dyn Move>>;
}

struct CharacterBaud<'a> {
    HP: i32,
    NAME: &'a str,
    STR: i32,
    DEX: i32,
    DEF: i32,
    SPD: i32,
    moves: Vec<Rc<dyn Move>>,
}

impl CharacterBaud<'_> {
    fn new() -> Self {
        let a: Vec<Rc<dyn Move>> = vec![Rc::new(MovePunch::new())];
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
    fn get_moves(&mut self) -> Vec<Rc<dyn Move>> {
        self.moves.clone()
    }
}

fn main() {
    let dmg = Damage {
        amount: 80,
        damage_type: DamageType::Bullet,
        dodgeable: false,
        defensible: false,
    };

    let mut baud = CharacterBaud::new();
    println!("{}", baud.get_hp());
    baud.process_damage(&dmg);
    println!("{}", baud.get_hp());
}

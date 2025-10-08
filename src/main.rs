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
    fn process_damage(&self, damage: &Damage) -> DamageResult;
    fn get_hp(&self) -> i32;
    fn set_hp(&self, amount: i32);
}

trait Entity: Damageable {
    fn get_name(&self) -> str;

    fn get_str(&self) -> i32;
    fn set_str(&self, amount: i32) -> i32;

    fn get_dex(&self) -> i32;
    fn set_dex(&self, amount: i32) -> i32;

    fn get_def(&self) -> i32;
    fn set_def(&self, amount: i32) -> i32;

    fn get_spd(&self) -> i32;
    fn set_spd(&self, amount: i32) -> i32;
}

trait Move {
    fn use_move(&self, user: &dyn Entity, target: &dyn Entity) -> DamageResult;
}

struct MovePunch {}

impl MovePunch {
    fn use_move(&self, user: &dyn Entity, target: &dyn Entity) -> DamageResult {
        let dmg = Damage {
            amount: 80,
            damage_type: DamageType::Blunt,
            dodgeable: true,
            defensible: true,
        };
        target.process_damage(&dmg);
    }
}

trait Attacker: Entity {
    fn get_moves(&self) -> Vec<Box<dyn Move>>;
}

struct CharacterBaud {
    HP: i32,
    NAME: str,
    STR: i32,
    DEX: i32,
    DEF: i32,
    SPD: i32,
}

impl CharacterBaud {
    fn new() -> Self {
        Self {
            HP: 4000,
            NAME: "Baud",
            STR: 80,
            DEX: 80,
            DEF: 30,
            SPD: 100,
        }
    }
}

impl Damageable for CharacterBaud {
    fn process_damage(&mut self, damage: &Damage) -> DamageResult {
        let mut remaining_damage = damage.amount - self.DEF;

        if (remaining_damage <= 0) {
            return DamageResult::Defend(damage.amount);
        }
        self.HP -= remaining_damage;
        DamageResult::Success(remaining_damage)
    }
}

fn main() {
    let dmg = Damage {
        amount: 80,
        damage_type: DamageType::Bullet,
        dodgeable: false,
        defensible: false,
    };
}

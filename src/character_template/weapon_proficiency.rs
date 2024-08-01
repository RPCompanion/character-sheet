
use super::common::Requirements;

pub struct WeaponProficiency {
    pub categories: Vec<WeaponCategory>
}

pub struct WeaponCategory {
    pub category: String,
    pub weapons: Vec<Weapon>
}

pub struct Weapon {
    pub weapon: String,
    pub required: Option<Requirements>
}
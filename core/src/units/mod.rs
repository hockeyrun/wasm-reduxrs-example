pub mod humans;

use cgmath::Vector2;
use std::collections::HashMap;

pub enum UnitType {
    Worker,
    Soldat,
    Rifler,
}

pub enum UnitAttackType {
    Weak,
    Normal,
    Heavy,
}

pub enum UnitWeaponType {
    Weak,
    Normal,
    Heavy,
}

pub enum UnitArmorType {
    Weak,
    Normal,
    Heavy,
}

pub enum UnitAbilityIds {
    Repair,
    Collect,
    Attack,
    Cast,
}

pub struct UnitAbility {
    pub id: UnitAbilityIds,
    pub name: String,
}

pub struct Unit {
    pub id: String,
    pub level: i32,
    pub cost: i32,
    pub food: i32,
    pub armor: i32,
    pub attack: i32,
    pub hp: i32,
    pub cooldown: i32,
    pub template: String,
    pub can_attack_air: bool,
    pub can_attack_ground: bool,
    pub position: Vector2<i32>,
    pub unit_type: UnitType,
    pub attack_type: UnitAttackType,
    pub weapon_type: UnitWeaponType,
    pub armor_type: UnitArmorType,
    pub abilities: HashMap<String, UnitAbility>,
    pub active_ability: Option<UnitAbility>,
}

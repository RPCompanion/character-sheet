
pub struct Template {
    pub name: String,
    pub version: [u8; 3]
}

pub struct Skill {
    pub name: String,
    pub value: i32
}

pub struct Attribute {
    pub name: String,
    pub value: i32,
    pub skills: Option<Vec<Skill>>
}

pub struct CharacterSheet {
    pub name: String,
    pub template: Template,
    pub description: Option<String>,
    pub health: i32,
    pub armor_class: i32,
    
    pub weapon_proficiencies: Vec<String>,
    pub perks: Option<Vec<String>>,    
    pub attributes: Vec<Attribute>
}
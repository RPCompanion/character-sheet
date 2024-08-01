
pub struct AttributeModifier {
    pub name: String,
    pub modifier: i32,
}

pub struct SkillModifier {
    pub name: String,
    pub modifier: i32,
}

pub struct AttributeRequirement {
    pub name: String,
    pub greater_than_or_equal_to: i32,
}

pub struct SkillRequirement {
    pub name: String,
    pub greater_than_or_equal_to: i32,
}

pub struct Requirements {
    pub perks: Option<Vec<String>>,
    pub attributes: Option<Vec<AttributeRequirement>>,
    pub skills: Option<Vec<SkillRequirement>>
}
use super::common::Requirements;

pub struct Skill {
    pub name: String,
    pub description: String
}

pub struct Attribute {
    pub name: String,
    pub description: String,
    pub skills: Vec<Skill>,
    pub required: Option<Requirements>
}
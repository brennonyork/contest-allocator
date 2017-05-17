enum RoleType {
    ChiefJudge,
    Judge,
    AssistantJudge,
    Recorder,
    BoundaryJudge,
    Runner,
    Starter,
}

pub struct Role {
    role: RoleType,
    name: String,
    dependencies: Vec<String>,
}

pub fn generate_roles() -> Vec<Role> {
    let r: Role = Role { role: RoleType::ChiefJudge,
                         name: "Steve".to_string(),
                         dependencies: vec!["a".to_string()], };
    vec![r]
}

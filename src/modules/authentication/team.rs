#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub parent_team_id: Option<String>,
    pub is_default: bool,
}

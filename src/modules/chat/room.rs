#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub participant_ids: Vec<String>,
}

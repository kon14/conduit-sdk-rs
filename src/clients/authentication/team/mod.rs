mod add_team_members;
mod create_team;
mod delete_team;
mod get_team;
mod remove_team_members;

use super::proto;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub parent_team_id: Option<String>,
    pub is_default: bool,
}

impl From<proto::Team> for Team {
    fn from(grpc_res: proto::Team) -> Self {
        let parent_team_id = if grpc_res.parent_team.trim().is_empty() {
            None
        } else {
            Some(grpc_res.parent_team)
        };
        Team {
            id: grpc_res.id,
            name: grpc_res.name,
            parent_team_id,
            is_default: grpc_res.is_default,
        }
    }
}

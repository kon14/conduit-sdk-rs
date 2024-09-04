mod change_user_pass;
mod create_user;
mod delete_user;

pub enum NewPassword {
    Key(String),
    Random,
}

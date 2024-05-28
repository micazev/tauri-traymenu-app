// src/utils.rs
pub fn get_logged_in_user_name() -> String {
  whoami::realname()
}

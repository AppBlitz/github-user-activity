use commands::core::api::get_user;

fn main() {
    let user_name: String = String::from("AppBlitz");
    get_user(user_name);
}

use crate::models::user::User;
use rocket_contrib::json::Json;

#[get("/<id>")]
pub fn user(id: u64) -> Json<User> {
    let user = User {
        id,
        name: "John Doe".to_string(),
    };
    Json(user)
}
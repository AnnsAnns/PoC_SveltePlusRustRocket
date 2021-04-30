use rocket::http::RawStr;
use serde::Deserialize;
use http::Request;

use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use serde_json::json;

#[get("/get/<id>")]
pub fn get(id: &RawStr) -> JsonValue {
    let id: String = id.to_string();

    let request = Request::builder()
        .uri(format!("https://api.minako.moe/detail/{}", id))
        .header("tomGER Test App", "v0-dev")
        .body(())
        .unwrap();

    json!(request.body())
}
use crate::state::State;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::content::RawHtml;
use rocket::serde::json::{json, Value};
use std::sync::{Arc, RwLock};

struct App {
    state: Arc<RwLock<State>>,
}

struct User {
    id: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let id = request
            .cookies()
            .get("user")
            .map(|cookie| cookie.value().to_string())
            .unwrap_or_default();
        Outcome::Success(User { id })
    }
}

#[rocket::get("/balance")]
fn balance(app: &rocket::State<App>, user: User) -> Value {
    let state = app.state.read().unwrap();
    let balance = state.balance_of(&user.id).unwrap_or_default();
    json!({ "balance": balance as u32 })
}

#[rocket::get("/pods/list")]
fn pods(app: &rocket::State<App>, user: User) -> Option<Value> {
    let state = app.state.read().unwrap();
    let pods = state.pods_of(&user.id);
    Some(json!({ "pods": pods }))
}

#[rocket::post("/pods/add?<image>&<cmd>&<name>")]
fn add_pod(
    app: &rocket::State<App>,
    user: User,
    name: String,
    image: String,
    cmd: String,
) -> Value {
    let mut state = app.state.write().unwrap();
    let result = state.add_pod(user.id, image, cmd, name);
    match result {
        Ok(pod_id) => json!({ "pod_id": pod_id }),
        Err(e) => json!({ "error": e.to_string() }),
    }
}

#[rocket::post("/pods/start?<id>")]
fn start_pod(app: &rocket::State<App>, user: User, id: u128) -> Value {
    let state = app.state.read().unwrap();
    let pod = state.get_pod(id).ok_or("Pod not found");
    match pod {
        Ok(pod) => {
            if pod.owner != user.id {
                return json!({ "error": "You don't own this pod" });
            }
            pod.start();
            json!({ "status": "ok" })
        }
        Err(err) => json!({ "error": err }),
    }
}

#[rocket::post("/recharge?<amount>")]
fn recharge(app: &rocket::State<App>, user: User, amount: u128) -> Value {
    let mut state = app.state.write().unwrap();
    state.recharge(&user.id, amount);
    json!({ "status": "ok" })
}

#[rocket::post("/pods/stop?<id>")]
fn stop_pod(app: &rocket::State<App>, user: User, id: u128) -> Value {
    let state = app.state.read().unwrap();
    let pod = state.get_pod(id).ok_or("Pod not found");
    match pod {
        Ok(pod) => {
            if pod.owner != user.id {
                return json!({ "error": "You don't own this pod" });
            }
            pod.stop();
            json!({ "status": "ok" })
        }
        Err(err) => json!({ "error": err }),
    }
}

// A single page app to display and manage the pods
#[rocket::get("/")]
fn index() -> RawHtml<String> {
    RawHtml(std::fs::read_to_string("static/index.html").unwrap())
}

pub(crate) async fn http_serve(state: Arc<RwLock<State>>) -> Result<(), rocket::Error> {
    use rocket::fs::{FileServer, Options};

    let app = App { state };
    let static_files = FileServer::new("static", Options::Missing);
    rocket::build()
        .mount(
            "/",
            rocket::routes![balance, pods, add_pod, start_pod, stop_pod, recharge, index,],
        )
        .mount("/static", static_files)
        .manage(app)
        .launch()
        .await
        .map(|_| ())
}

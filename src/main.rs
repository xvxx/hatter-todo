use {hatter::Env, vial::prelude::*};

mod db;
use db::{RequestWithTodos, TodoDB};

routes! {
    GET "/" => list;
    POST "/" => create;
    POST "/check/:id" => check;
}

fn check(req: Request) -> impl Responder {
    if let Some(id) = req.arg("id") {
        let id = id.parse().unwrap_or(0);
        if id < req.todos().len() {
            req.todos().check(id);
        }
    }
}

fn list(req: Request) -> vial::Result<String> {
    let mut env = Env::new();
    env.set("todos", req.todos().all());
    let body = env.render(&asset::to_string("hat/list.hat")?).unwrap();
    env.set("body", body);
    Ok(env.render(&asset::to_string("hat/layout.hat")?).unwrap())
}

fn create(req: Request) -> Option<Response> {
    let todo = req.form("todo")?;
    req.todos().push(todo.to_string());
    Some(Response::redirect_to("/"))
}

fn main() {
    asset_dir!("assets");
    use_state!(TodoDB::new());
    run!().unwrap();
}

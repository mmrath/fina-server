use actix::prelude::*;
use util::{establish_connection_pool,DbConnectionPool};


use super::core::config;
use actix::prelude::*;
use actix_web::{middleware, server, App};


pub struct ServiceActor {
    pub pool: DbConnectionPool,
}

impl Actor for ServiceActor {
    type Context = SyncContext<Self>;
}


/// State with DbExecutor address
pub struct AppState {
    pub service_actor: Addr<Syn, ServiceActor>,
}





pub fn run() {
    let sys = actix::System::new("diesel-example");

    // Start http server
    let server = server::new(move || create_app())
        .bind("127.0.0.1:8080")
        .unwrap()
        .start();

    let _ = sys.run();
}

pub fn create_app() -> App<AppState> {
    let app = App::with_state(create_state());
    config_app(app)
}

pub fn create_state() -> AppState {
    let pool = establish_connection_pool();
    let addr = SyncArbiter::start(3, move || ServiceActor { pool: pool.clone() });

    AppState {
        service_actor: addr.clone(),
    }
}

pub fn config_app(app: App<AppState>) -> App<AppState> {
    let app = app.prefix("/api").middleware(middleware::Logger::default());

    super::core::config(app)
}

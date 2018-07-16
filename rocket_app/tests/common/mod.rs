use parking_lot::Mutex;

static DB_LOCK: Mutex<()> = Mutex::new(());

macro_rules! run_test {
    (| $client:ident, $conn:ident | $block:expr) => {{
        ::std::env::set_var(
        "DATABASE_URL",
        "postgres://billac:billac@localhost/billacdb",
    );
        let _lock = DB_LOCK.lock();
        let (rocket, db) = fina_app_lib::rocket();
        let $client = Client::new(rocket).expect("Rocket client");
        let $conn = db.expect("failed to get database connection for testing");
        $block
    }};
}

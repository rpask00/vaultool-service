pub mod prod {
    use std::time::Duration;
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
    pub const TIMEOUT: Duration = Duration::from_millis(200);
}

pub mod test {
    use std::time::Duration;
    pub const TEST_DB_PREFIX: &str = "test_";
    pub const APP_ADDRESS: &str = "127.0.0.1:0";
    pub const TIMEOUT: Duration = Duration::from_millis(200);
}

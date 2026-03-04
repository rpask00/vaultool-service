pub mod prod {
    use std::time::Duration;
    pub const APP_ADDRESS: &str = "0.0.0.0:3000";
    pub const TIMEOUT: Duration = Duration::from_millis(200);
}

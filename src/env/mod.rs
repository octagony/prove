pub mod enviroment {
    use std::env;
    pub fn read_env() -> String {
        let read_user = env::var("USER").unwrap();
        let format_path = format!("/home/{}/.config", read_user);
        format_path
    }
}

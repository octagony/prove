pub mod enviroment {
    use std::env;
    use std::path::Path;

    pub fn read_env() -> String {
        let read_user = std::env::var("USER").unwrap();
        let format_path = format!("/home/{}/.config", read_user);
        format_path
    }
}

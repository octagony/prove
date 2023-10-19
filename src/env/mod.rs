pub mod enviroment {
    use std::env;
    pub fn read_env() -> String {
        //TODO: Implement different path readings for the configuration folder
        let read_user = env::var("USER").expect("Unable to get data about $USER variable");
        let format_path = format!("/home/{}/.config", read_user);
        format_path
    }
}

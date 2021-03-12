use dotenv_parser::parse_dotenv;
use std::fs;

pub fn parse() {
    if let Ok(env_file) = fs::read_to_string(".env") {
        if let Ok(envs) = parse_dotenv(&env_file) {
            for (k, v) in envs {
                std::env::set_var(k, v);
            }
        }
    }
}

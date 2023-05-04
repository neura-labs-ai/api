use std::collections::HashMap;
use env_file_reader::read_file;

pub struct Environment {
    pub mongodb_uri: String,
    // The super-key is the direct bypass key for the API. Used for internal API's
    pub super_key: String,
}

fn load_env_data() -> anyhow::Result<HashMap<String, String>> {
    let env_variables = read_file(".env").expect("Failed to read .env file");

    let mut env_data = HashMap::new();

    for (key, value) in env_variables {
        env_data.insert(key, value);
    }

    Ok(env_data)
}

// Easy access to the environment variables
pub fn env() -> anyhow::Result<Environment> {
    let env_data = load_env_data().unwrap();

    let mongodb_uri = match env_data.get("MONGODB_URI") {
        Some(uri) => uri,
        None => "mongodb://localhost:27017",
    };
    let super_key = match env_data.get("SUPER_KEY") {
        Some(key) => key,
        None => return Err(anyhow::anyhow!("SUPER_KEY not found in .env")),
    };

    Ok(Environment {
        mongodb_uri: mongodb_uri.to_string(),
        super_key: super_key.to_string(),
    })
}

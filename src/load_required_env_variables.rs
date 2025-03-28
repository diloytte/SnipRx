use std::env;

#[derive(Debug)]
pub struct EnvVariables {
    pub api_id: i32,
    pub api_hash: String,
    pub phone_number: String,
    pub password: String,
}

pub fn load_required_env_variables() -> Result<EnvVariables, Box<dyn std::error::Error>> {
    let api_id: i32 = env::var("API_ID")?.parse()?;
    let api_hash: String = env::var("API_HASH")?;
    let phone_number: String = env::var("PHONE_NUMBER")?;
    let password: String = env::var("PASSWORD")?;

    Ok(EnvVariables {
        api_id,
        api_hash,
        phone_number,
        password,
    })
}

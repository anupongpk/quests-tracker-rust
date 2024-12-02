use anyhow::{Ok, Result};

use super::{
    config_model::{AdventurersSecret, Database, DotEnvyCongfig, GuildCommandersSecret, Server},
    stage::Stage,
};

pub fn load() -> Result<DotEnvyCongfig> {
    dotenvy::dotenv().ok();

    let server = Server {
        port: std::env::var("SERVER_PORT")
            .expect("SEVER_PORT is invalid")
            .parse()?,
        body_limit: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
        timeout: std::env::var("SERVER_TIMEOUT")
            .expect("SERVER_TIMEOUT is invalid")
            .parse()?,
    };

    let database = Database {
        url: std::env::var("DATABASE_URL").expect("DATABASE_URL is invalid"),
    };

    Ok(DotEnvyCongfig { server, database })
}

pub fn get_stage() -> Stage {
    dotenvy::dotenv().ok();

    let stage_str = std::env::var("STAGE").unwrap_or("".to_string());
    Stage::try_from(&stage_str).unwrap_or_default()
}

pub fn get_adventurers_secrt_env() -> Result<AdventurersSecret> {
    Ok(AdventurersSecret {
        secret: std::env::var("JWT_ADVENTURER_SECRET").expect("JWT_ADVENTURER_SECRET is invalid"),
        refresh_secret: std::env::var("JWT_ADVENTURER_REFRESH_SECRET")
            .expect("JWT_ADVENTURER_REFRESH_SECRET is invalid"),
    })
}

pub fn get_guild_commanders_secrt_env() -> Result<GuildCommandersSecret> {
    Ok(GuildCommandersSecret {
        secret: std::env::var("JWT_GUILD_COMMANDER_SECRET")
            .expect("JWT_GUILD_COMMANDER_SECRET is invalid"),
        refresh_secret: std::env::var("JWT_GUILD_COMMANDER_REFRESH_SECRET")
            .expect("JWT_GUILD_COMMANDER_REFRESH_SECRET is invalid"),
    })
}

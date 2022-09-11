
#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

fn main() -> Result<(), config::ConfigError> {
    let mut settings = config::Config::default();
      let Settings{database, application_port}: Settings = {
        settings.merge(config::File::with_name("configuration"))?;
        settings.try_into()?
      };
      
      println!("{}", database.connection_string());
      println!("{}", application_port);

      Ok(())
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}



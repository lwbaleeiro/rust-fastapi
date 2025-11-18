use once_cell::sync::Lazy;
use std::env;

// Static garante que a função de configuração seja executada apenas uma vez durante o tempo de vida do programa.
// Lazy é usado para inicializar a configuração de forma preguiçosa, ou seja, somente quando for acessada pela primeira vez.
pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::from_env().unwrap_or_else(|err| {
        tracing::error!("FATAL! Failed to load config: {}", err);
        std::process::exit(1);
    })
});

#[derive(Debug)] // Permite imprimir a estrutura para depuração.
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

// Implementação do Config, a função from_env lê as variáveis de ambiente e cria uma instância de Config.
impl Config {
    fn from_env() -> Result<Self, env::VarError> {
        // ? operador propaga erros automaticamente, se qualquer uma das variáveis de ambiente estiver faltando, 
        // a criação da Config falhará imediatamente com um erro claro
        Ok(Self{
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            port: env::var("PORT")?
                .parse::<u16>()
                .expect("PORT must be a valid u16"),
        })
    }
}
use crate::{
    config::{Config, CONFIG},
    db::DbPool,
};

pub struct Context {
    pool: DbPool,
    config: Config,
}

impl Context {
    pub fn create(pool: DbPool, config: Config) -> Context {
        Context { pool, config }
    }
    pub fn pool(&self) -> &DbPool {
        &self.pool
    }
    pub fn config(&self) -> &'static Config {
        &CONFIG
    }
}

impl Clone for Context {
    fn clone(&self) -> Self {
        Context {
            pool: self.pool.clone(),
            config: self.config.clone(),
        }
    }
}

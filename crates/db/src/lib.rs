use clorinde::{deadpool_postgres, tokio_postgres};
use std::str::FromStr;

pub fn create_pool(database_url: &str) -> deadpool_postgres::Pool {
    let config = tokio_postgres::Config::from_str(database_url).unwrap();
    let manager = deadpool_postgres::Manager::new(config, tokio_postgres::NoTls);
    deadpool_postgres::Pool::builder(manager).build().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use clorinde::queries::hero::*;
    use clorinde::queries::player::{create_player};
    

    #[tokio::test]
    async fn load_heroes() {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = create_pool(&database_url);
        let conn = pool.get().await.unwrap();
        let heroes = get_heroes().bind(&conn).all().await.unwrap();
        dbg!(heroes);
    }

    #[tokio::test]
    async fn create_player_test() {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = create_pool(&database_url);
        let conn = pool.get().await.unwrap();
        let hero = create_player().bind(&conn, &"TEST".to_string()).all().await.unwrap();
        dbg!(hero);
    }
}

use db::create_pool;

#[cfg(test)]
mod tests {
    use super::*;
    use clorinde::queries::game_client;
    use clorinde::queries::hero_client;
    use clorinde::queries::player_client;
    use clorinde::queries::team_client;

    #[tokio::test]
    async fn player_workflow_test() {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = create_pool(&database_url);
        let conn = pool.get().await.unwrap();
        let player = player_client::create_player()
            .bind(&conn, &"TEST".to_string())
            .one()
            .await
            .unwrap();
        assert!(player.display_name == "TEST".to_string());

        let read_player = player_client::get_player_by_id()
            .bind(&conn, &player.id)
            .one()
            .await
            .unwrap();
        assert!(read_player.id == player.id);
        assert!(read_player.display_name == "TEST".to_string());

        let players = player_client::get_players().bind(&conn).all().await.unwrap();
        assert!(!players.is_empty());

        // Clean up
        let deleted_player = player_client::delete_player_by_id()
            .bind(&conn, &player.id)
            .one()
            .await
            .unwrap();

        assert!(deleted_player.id == player.id);
    }

    #[tokio::test]
    async fn hero_workflow_test() {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = create_pool(&database_url);
        let conn = pool.get().await.unwrap();
        let heroes = hero_client::get_heroes().bind(&conn).all().await.unwrap();
        assert!(!heroes.is_empty());
    }

    #[tokio::test]
    async fn team_workflow_test() {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = create_pool(&database_url);
        let conn = pool.get().await.unwrap();

        let teams = team_client::get_teams().bind(&conn).all().await.unwrap();
        assert!(teams.is_empty());
    }

    #[tokio::test]
    async fn game_workflow_test() {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = create_pool(&database_url);
        let conn = pool.get().await.unwrap();

        let games = game_client::get_games().bind(&conn).all().await.unwrap();
        assert!(games.is_empty());
    }
}
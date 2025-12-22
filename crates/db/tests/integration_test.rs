use db::create_pool;

#[cfg(test)]
mod tests {
    use super::*;
    use clorinde::queries::GameClient;
    use clorinde::queries::HeroClient;
    use clorinde::queries::PlayerClient;
    use clorinde::queries::TeamClient;

    #[tokio::test]
    async fn player_workflow_test() {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = create_pool(&database_url);
        let conn = pool.get().await.unwrap();
        let player = PlayerClient::create_player()
            .bind(&conn, &"TEST".to_string())
            .one()
            .await
            .unwrap();
        assert!(player.display_name == "TEST".to_string());

        let read_player = PlayerClient::get_player_by_id()
            .bind(&conn, &player.id)
            .one()
            .await
            .unwrap();
        assert!(read_player.id == player.id);
        assert!(read_player.display_name == "TEST".to_string());

        let players = PlayerClient::get_players().bind(&conn).all().await.unwrap();
        assert!(!players.is_empty());

        // Clean up
        let deleted_player = PlayerClient::delete_player_by_id()
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
        let heroes = HeroClient::get_heroes().bind(&conn).all().await.unwrap();
        assert!(!heroes.is_empty());
    }

    #[tokio::test]
    async fn team_workflow_test() {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = create_pool(&database_url);
        let conn = pool.get().await.unwrap();

        let teams = TeamClient::get_teams().bind(&conn).all().await.unwrap();
        assert!(teams.is_empty());
    }

    #[tokio::test]
    async fn game_workflow_test() {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = create_pool(&database_url);
        let conn = pool.get().await.unwrap();

        let games = GameClient::get_games().bind(&conn).all().await.unwrap();
        assert!(games.is_empty());
    }
}
--: BestOfMatchGame()

--! get_best_of_match_games : BestOfMatchGame
SELECT match_id, game_id, game_order
FROM tagteam.best_of_match_game;

--! add_game_to_best_of_match : BestOfMatchGame
INSERT INTO tagteam.best_of_match_game (match_id, game_id, game_order)
VALUES (:match_id, :game_id, (SELECT COALESCE(MAX(game_order), 0) + 1
                      FROM tagteam.best_of_match_game
                      WHERE match_id = :match_id))
RETURNING match_id, game_id, game_order;
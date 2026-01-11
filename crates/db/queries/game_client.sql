--: Game()

--! get_games : Game
SELECT id, team_one_id, team_two_id, winner_id, loser_id, win_condition_id, created_at
FROM tagteam.game;

--! get_game_by_id : Game
SELECT id, team_one_id, team_two_id, winner_id, loser_id, win_condition_id, created_at
FROM tagteam.game
WHERE id = :id;

--! create_game : Game
INSERT INTO tagteam.game (team_one_id, team_two_id, winner_id, loser_id, win_condition_id)
VALUES (:team_one_id, :team_two_id, :winner_id, :loser_id, :win_condition_id)
RETURNING id, team_one_id, team_two_id, winner_id, loser_id, win_condition_id, created_at;
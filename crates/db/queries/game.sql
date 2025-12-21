--: Game()

--! get_games : Game
SELECT id, team_one_id, team_two_id, winner_id, loser_id, win_condition_id, created_at
FROM tagteam.game;
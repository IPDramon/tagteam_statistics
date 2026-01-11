--: BestOfMatch()

--! get_best_of_matches : BestOfMatch
SELECT id, winner_id, loser_id, created_at
FROM tagteam.best_of_match;

--! get_best_of_match_by_id : BestOfMatch
SELECT id, winner_id, loser_id, created_at
FROM tagteam.best_of_match
WHERE id = :id; 

--! create_best_of_match : BestOfMatch
INSERT INTO tagteam.best_of_match (winner_id, loser_id)
VALUES (NULL, NULL)
RETURNING id, winner_id, loser_id, created_at;

--! update_best_of_match : BestOfMatch
UPDATE tagteam.best_of_match
SET winner_id = :winner_id, loser_id = :loser_id
WHERE id = :id
RETURNING id, winner_id, loser_id, created_at;
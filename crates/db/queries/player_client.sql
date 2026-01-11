--: Player()

--! get_players : Player
SELECT id, display_name, created_at
FROM tagteam.player;

--! get_player_by_id : Player
SELECT id, display_name, created_at
FROM tagteam.player
WHERE id = :id;

--! create_player : Player
INSERT INTO tagteam.player (display_name)
VALUES (:display_name)
RETURNING id, display_name, created_at;

--! delete_player_by_id: Player
DELETE FROM tagteam.player
WHERE id = :id
RETURNING id, display_name, created_at;
--: Team()

--! get_teams : Team
SELECT id, left_partner_id, right_partner_id, player_id, created_at
FROM tagteam.team;

--! get_team_by_id : Team
SELECT id, left_partner_id, right_partner_id, player_id, created_at
FROM tagteam.team
WHERE id = :id;

--! create_team : Team
INSERT INTO tagteam.team (id, left_partner_id, right_partner_id, player_id)
VALUES (gen_random_uuid(), :left_partner_id, :right_partner_id, :player_id)
RETURNING id, left_partner_id, right_partner_id, player_id, created_at;
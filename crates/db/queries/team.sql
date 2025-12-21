--: Team()

--! get_teams : Team
SELECT id, left_partner_id, right_partner_id, created_at
FROM tagteam.team;
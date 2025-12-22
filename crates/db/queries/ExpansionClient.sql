--: Expansion()

--! get_expansions : Expansion
SELECT id, title, created_at
FROM tagteam.expansion;

--! get_expansion_by_id : Expansion
SELECT id, title, created_at
FROM tagteam.expansion
WHERE id = :id;
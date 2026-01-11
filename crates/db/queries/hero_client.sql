--: Hero()

--! get_heroes : Hero
SELECT id, display_name, expansion_id, created_at
FROM tagteam.hero;

--! get_hero_by_id : Hero
SELECT id, display_name, expansion_id, created_at
FROM tagteam.hero
WHERE id = :id;


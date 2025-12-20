-- migrate:up
INSERT INTO tagteam.expansion (id, title, created_at) VALUES
    (gen_random_uuid(), 'Base Game', CURRENT_TIMESTAMP);

INSERT INTO tagteam.hero (id, display_name, expansion_id, base_power, base_health, created_at)
VALUES
    (gen_random_uuid(), 'Warrior', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), 5, 10, CURRENT_TIMESTAMP),
    (gen_random_uuid(), 'Mage', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), 4, 8, CURRENT_TIMESTAMP),
    (gen_random_uuid(), 'Rogue', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), 6, 7, CURRENT_TIMESTAMP),
    (gen_random_uuid(), 'Cleric', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), 3, 12, CURRENT_TIMESTAMP);

-- migrate:down
DELETE FROM tagteam.hero
WHERE expansion_id = (SELECT id FROM tagteam.expansion WHERE title = 'Base Game');
DELETE FROM tagteam.expansion WHERE title = 'Base Game';


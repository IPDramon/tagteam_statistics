-- migrate:up
INSERT INTO tagteam.expansion (id, title)
VALUES (gen_random_uuid(), 'Base Game');

INSERT INTO tagteam.win_condition (id, code)
VALUES 
(gen_random_uuid(), 'ko'),
(gen_random_uuid(), 'shango-fire');

-- migrate:down
DELETE FROM tagteam.win_condition
WHERE code IN ('ko', 'shango-fire');
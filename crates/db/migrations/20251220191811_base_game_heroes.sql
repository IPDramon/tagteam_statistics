-- migrate:up
INSERT INTO tagteam.expansion (title, created_at) VALUES
    ('Base Game', CURRENT_TIMESTAMP);

INSERT INTO tagteam.hero (display_name, expansion_id, created_at)
VALUES
    ('Jeanne', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), CURRENT_TIMESTAMP),
    ('Die wilde Meute', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), CURRENT_TIMESTAMP),
    ('Bödvar', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), CURRENT_TIMESTAMP),
    ('Shango', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), CURRENT_TIMESTAMP),
    ('Maman Brijit', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), CURRENT_TIMESTAMP),
    ('Mephisto', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), CURRENT_TIMESTAMP),
    ('Das Waldvolk', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), CURRENT_TIMESTAMP),
    ('Mordred', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), CURRENT_TIMESTAMP),
    ('Milady', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), CURRENT_TIMESTAMP),
    ('Der Golem', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), CURRENT_TIMESTAMP),
    ('Wong Fei-Hung', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), CURRENT_TIMESTAMP),
    ('Ching Shih', (SELECT id FROM tagteam.expansion WHERE title = 'Base Game'), CURRENT_TIMESTAMP);

INSERT INTO tagteam.health_bar (hero_id, health, created_at)
VALUES
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Jeanne'), 18, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Die wilde Meute'), 5, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Bödvar'), 11, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Bödvar'), 15, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Shango'), 15, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Maman Brijit'), 16, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mephisto'), 13, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Das Waldvolk'), 3, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Das Waldvolk'), 4, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Das Waldvolk'), 5, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mordred'), 19, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Milady'), 16, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Der Golem'), 25, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Wong Fei-Hung'), 17, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Ching Shih'), 14, CURRENT_TIMESTAMP);

INSERT INTO tagteam.strength_bar (hero_id, strength, created_at)
VALUES
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Jeanne'), 1, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Die wilde Meute'), 1, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Bödvar'), 3, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Shango'), 0, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Maman Brijit'), 2, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mephisto'), 1, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Das Waldvolk'), 0, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mordred'), 0, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Milady'), 1, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Der Golem'), 1, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Wong Fei-Hung'), 2, CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Ching Shih'), 2, CURRENT_TIMESTAMP);

INSERT INTO tagteam.card (hero_id, title, created_at)
VALUES
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Jeanne'), 'Dieu Permier Servi', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Jeanne'), 'Hand der Gerechtigkeit', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Jeanne'), 'Schwert des Hl. Michael', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Jeanne'), 'Heiliger Schild', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Jeanne'), 'Göttliche Vision', CURRENT_TIMESTAMP),

    ((SELECT id FROM tagteam.hero WHERE display_name = 'Die wilde Meute'), 'Korrupter Sheriff', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Die wilde Meute'), 'Schlüssel zum Waffenlager', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Die wilde Meute'), 'Doc für alle Fälle', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Die wilde Meute'), 'Zurück ins Lager', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Die wilde Meute'), 'Die persönliche Wumme des Sheriffs', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Die wilde Meute'), 'Bestechung', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Die wilde Meute'), 'Nich zum Reden hier', CURRENT_TIMESTAMP),

    ((SELECT id FROM tagteam.hero WHERE display_name = 'Bödvar'), 'Wutanfall', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Bödvar'), 'Raserei!', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Bödvar'), 'Rune der Verwandlung', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Bödvar'), 'Rune der Stärke', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Bödvar'), 'Blutsbande', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Bödvar'), 'Kraft schöpfen', CURRENT_TIMESTAMP),

    ((SELECT id FROM tagteam.hero WHERE display_name = 'Shango'), 'Brenne!', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Shango'), 'Donnerstein', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Shango'), 'Flammenwand', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Shango'), 'Ausbrennen', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Shango'), 'Blitzschlag', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Shango'), 'Feueratem', CURRENT_TIMESTAMP),
    
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Maman Brijit'), 'Feuerpunsch', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Maman Brijit'), 'Du gehörst mir!', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Maman Brijit'), 'Unheiliges Ritual', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Maman Brijit'), 'Ewige Jugend', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Maman Brijit'), 'Aderlass', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Maman Brijit'), 'Seelenopfer', CURRENT_TIMESTAMP),

    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mephisto'), 'Zwillingsschlangen', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mephisto'), 'Die Schlange der Leere', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mephisto'), 'Seelenernte', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mephisto'), 'Die Schlange der Tiefe', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mephisto'), 'Höllenschlund', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mephisto'), 'Zorn der Verdammten', CURRENT_TIMESTAMP),

    ((SELECT id FROM tagteam.hero WHERE display_name = 'Das Waldvolk'), 'Alles ist vergänglich', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Das Waldvolk'), 'Geistersturm', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Das Waldvolk'), 'Kreis des Lebens', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Das Waldvolk'), 'Verstricken', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Das Waldvolk'), 'Beschwörung der Rotkappe', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Das Waldvolk'), 'Beschwörung des Baumhirten', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Das Waldvolk'), 'Beschwörung des Leshi', CURRENT_TIMESTAMP),

    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mordred'), 'Dunkle Macht', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mordred'), 'Todesstoß', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mordred'), 'Tückischer Konter', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mordred'), 'Versteckter Dolch', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Mordred'), 'Schattenmantel', CURRENT_TIMESTAMP),

    ((SELECT id FROM tagteam.hero WHERE display_name = 'Milady'), 'Todschick', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Milady'), 'Auge um Auge', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Milady'), 'Was dich nicht umbringt, macht mich stärker', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Milady'), 'Wunden lecken', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Milady'), 'So vorhersehbar', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Milady'), 'Für gute Freunde ...', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Milady'), 'Schachmatt', CURRENT_TIMESTAMP),

    ((SELECT id FROM tagteam.hero WHERE display_name = 'Der Golem'), 'Die Unschuldigen beschützen', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Der Golem'), 'Wiedergeburt', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Der Golem'), 'Lehmfaust', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Der Golem'), 'Selbstopfer', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Der Golem'), 'Zusammenflicken', CURRENT_TIMESTAMP),

    ((SELECT id FROM tagteam.hero WHERE display_name = 'Wong Fei-Hung'), 'In der Ruhe liegt die Kraft', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Wong Fei-Hung'), 'Innere Balance', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Wong Fei-Hung'), 'Das Chi fokussieren', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Wong Fei-Hung'), 'Tigerklaue', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Wong Fei-Hung'), 'Kraftbrücke', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Wong Fei-Hung'), 'Lähmender Schlag', CURRENT_TIMESTAMP),

    ((SELECT id FROM tagteam.hero WHERE display_name = 'Ching Shih'), 'Schrecken der Meere', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Ching Shih'), 'Setzt die Segel!', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Ching Shih'), 'Geheimer Hafen', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Ching Shih'), 'Ausmanövriert', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Ching Shih'), 'Entertrupp', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Ching Shih'), 'Schwarzpulver-Trunk', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Ching Shih'), 'Kraftvolle Parade', CURRENT_TIMESTAMP),
    ((SELECT id FROM tagteam.hero WHERE display_name = 'Ching Shih'), 'Die Rache der Rotsegel-Flotte', CURRENT_TIMESTAMP)
;


INSERT INTO tagteam.win_condition (dependant_hero_id, code, created_at)
VALUES
(NULL, 'ko', CURRENT_TIMESTAMP),
((SELECT id FROM tagteam.hero WHERE display_name = 'Shango'), 'shango-fire', CURRENT_TIMESTAMP),
((SELECT id FROM tagteam.hero WHERE display_name = 'Das Waldvolk'), 'waldvolk-all_ghosts', CURRENT_TIMESTAMP);

-- migrate:down
DELETE FROM tagteam.expansion WHERE title = 'Base Game';


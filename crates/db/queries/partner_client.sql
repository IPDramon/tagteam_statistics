--: Partner()

--! get_partners : Partner
SELECT id, hero_id, created_at
FROM tagteam.partner;

--! get_partner_by_id : Partner
SELECT id, hero_id, created_at
FROM tagteam.partner
WHERE id = :id;

--! create_partner : Partner
INSERT INTO tagteam.partner (hero_id)
VALUES (:hero_id)
RETURNING id, hero_id, created_at;
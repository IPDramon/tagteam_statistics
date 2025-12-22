--: Partner()

--! get_partners : Partner
SELECT id, hero_id, final_power, final_health, created_at
FROM tagteam.partner;

--! get_partner_by_id : Partner
SELECT id, hero_id, final_power, final_health, created_at
FROM tagteam.partner
WHERE id = :id;

--! create_partner : Partner
INSERT INTO tagteam.partner (id, hero_id, final_power, final_health)
VALUES (gen_random_uuid(), :hero_id, :final_power, :final_health)
RETURNING id, hero_id, final_power, final_health, created_at;
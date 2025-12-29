--: Card()

--! get_cards_of_hero : Card
SELECT id, title
FROM tagteam.card
WHERE hero_id = :hero_id;

--! get_cards_of_team() : Card
SELECT card.id, card.title
FROM tagteam.card card 
  INNER JOIN 
    (SELECT * FROM tagteam.deck WHERE team_id = :team_id) deck
  ON deck.card_id = card.id
ORDER BY deck.card_order ASC;


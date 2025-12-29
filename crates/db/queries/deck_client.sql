--: Deck()

--! add_card_to_deck() : Deck
INSERT INTO tagteam.deck (team_id, card_id, card_order)
VALUES
  (:team_id, :card_id, :card_order)
RETURNING team_id, card_id, card_order;
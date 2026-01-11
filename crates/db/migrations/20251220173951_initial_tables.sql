-- migrate:up
CREATE SCHEMA IF NOT EXISTS tagteam;

CREATE TABLE tagteam.expansion (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


CREATE TABLE tagteam.hero (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    display_name VARCHAR(255) NOT NULL,
    expansion_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (expansion_id) REFERENCES tagteam.expansion(id) ON DELETE CASCADE
);

CREATE TABLE tagteam.health_bar (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    hero_id UUID NOT NULL,
    health INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (hero_id) REFERENCES tagteam.hero(id) ON DELETE CASCADE
);

CREATE TABLE tagteam.strength_bar (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    hero_id UUID NOT NULL,
    strength INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (hero_id) REFERENCES tagteam.hero(id) ON DELETE CASCADE
);


CREATE TABLE tagteam.player (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    display_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tagteam.partner (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    hero_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (hero_id) REFERENCES tagteam.hero(id) ON DELETE CASCADE
);

CREATE TABLE tagteam.final_health (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    partner_id UUID NOT NULL,
    health_bar_id UUID NOT NULL,
    health INTEGER,
    FOREIGN KEY (partner_id) REFERENCES tagteam.partner(id) ON DELETE CASCADE,
    FOREIGN KEY (health_bar_id) REFERENCES tagteam.health_bar(id) ON DELETE CASCADE
);

CREATE FUNCTION tagteam.validate_final_health_hero() RETURNS TRIGGER AS $$
BEGIN
    IF (SELECT hero_id FROM tagteam.health_bar WHERE id = NEW.health_bar_id) !=
       (SELECT hero_id FROM tagteam.partner WHERE id = NEW.partner_id) THEN
        RAISE EXCEPTION 'health_bar and partner must reference the same hero';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER final_health_hero_check
BEFORE INSERT OR UPDATE ON tagteam.final_health
FOR EACH ROW
EXECUTE FUNCTION tagteam.validate_final_health_hero();

CREATE TABLE tagteam.final_strength (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    partner_id UUID NOT NULL,
    strength_bar_id UUID NOT NULL,
    strength INTEGER,
    FOREIGN KEY (partner_id) REFERENCES tagteam.partner(id) ON DELETE CASCADE,
    FOREIGN KEY (strength_bar_id) REFERENCES tagteam.strength_bar(id) ON DELETE CASCADE
);

CREATE FUNCTION tagteam.validate_final_strength_hero() RETURNS TRIGGER AS $$
BEGIN
    IF (SELECT hero_id FROM tagteam.strength_bar WHERE id = NEW.strength_bar_id) !=
       (SELECT hero_id FROM tagteam.partner WHERE id = NEW.partner_id) THEN
        RAISE EXCEPTION 'strength_bar and partner must reference the same hero';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER final_strength_hero_check
BEFORE INSERT OR UPDATE ON tagteam.final_strength
FOR EACH ROW
EXECUTE FUNCTION tagteam.validate_final_strength_hero();

CREATE TABLE tagteam.card (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    hero_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (hero_id) REFERENCES tagteam.hero(id) ON DELETE CASCADE
);

CREATE TABLE tagteam.team (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    left_partner_id UUID NOT NULL,
    right_partner_id UUID NOT NULL,
    player_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (left_partner_id) REFERENCES tagteam.partner(id) ON DELETE CASCADE,
    FOREIGN KEY (right_partner_id) REFERENCES tagteam.partner(id) ON DELETE CASCADE,
    FOREIGN KEY (player_id) REFERENCES tagteam.player(id) ON DELETE CASCADE
);

CREATE TABLE tagteam.deck (
    team_id UUID NOT NULL,
    card_id UUID NOT NULL,
    card_order INTEGER NOT NULL,
    FOREIGN KEY (card_id) REFERENCES tagteam.card(id) ON DELETE CASCADE,
    FOREIGN KEY (team_id) REFERENCES tagteam.team(id) ON DELETE CASCADE,
    PRIMARY KEY (team_id, card_order, card_id)
);

CREATE TABLE tagteam.win_condition (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dependant_hero_id UUID,
    code VARCHAR(100) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (dependant_hero_id) REFERENCES tagteam.hero(id) ON DELETE CASCADE
);

CREATE TABLE tagteam.best_of_match (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    winner_id UUID REFERENCES tagteam.player(id) DEFERRABLE INITIALLY DEFERRED,
    loser_id UUID REFERENCES tagteam.player(id) DEFERRABLE INITIALLY DEFERRED,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tagteam.game (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    team_one_id UUID NOT NULL,
    team_two_id UUID NOT NULL,
    winner_id UUID NOT NULL,
    loser_id UUID NOT NULL,
    win_condition_id UUID NOT NULL,
    num_rounds INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (team_one_id) REFERENCES tagteam.team(id) ON DELETE CASCADE,
    FOREIGN KEY (team_two_id) REFERENCES tagteam.team(id) ON DELETE CASCADE,
    FOREIGN KEY (winner_id) REFERENCES tagteam.player(id) ON DELETE CASCADE,
    FOREIGN KEY (loser_id) REFERENCES tagteam.player(id) ON DELETE CASCADE,
    FOREIGN KEY (win_condition_id) REFERENCES tagteam.win_condition(id) ON DELETE CASCADE
);

CREATE TABLE tagteam.best_of_match_game (
    match_id UUID NOT NULL,
    game_id UUID NOT NULL,
    game_order INTEGER NOT NULL,
    PRIMARY KEY (match_id, game_id),
    FOREIGN KEY (match_id) REFERENCES tagteam.best_of_match(id) ON DELETE CASCADE,
    FOREIGN KEY (game_id) REFERENCES tagteam.game(id) ON DELETE CASCADE
);

-- migrate:down
DROP TABLE IF EXISTS tagteam.best_of_match_game;
DROP TABLE IF EXISTS tagteam.best_of_match;
DROP TABLE IF EXISTS tagteam.game;
DROP TABLE IF EXISTS tagteam.win_condition;
DROP TABLE IF EXISTS tagteam.deck;
DROP TABLE IF EXISTS tagteam.team;
DROP TABLE IF EXISTS tagteam.card;
DROP TRIGGER IF EXISTS final_strength_hero_check ON tagteam.final_strength;
DROP FUNCTION IF EXISTS tagteam.validate_final_strength_hero();
DROP TRIGGER IF EXISTS final_health_hero_check ON tagteam.final_health;
DROP FUNCTION IF EXISTS tagteam.validate_final_health_hero();
DROP TABLE IF EXISTS tagteam.final_health;
DROP TABLE IF EXISTS tagteam.final_strength;
DROP TABLE IF EXISTS tagteam.partner;
DROP TABLE IF EXISTS tagteam.player;
DROP TABLE IF EXISTS tagteam.health_bar;
DROP TABLE IF EXISTS tagteam.strength_bar;
DROP TABLE IF EXISTS tagteam.hero;
DROP TABLE IF EXISTS tagteam.expansion;
DROP SCHEMA IF EXISTS tagteam;

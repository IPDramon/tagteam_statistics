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
    base_power INTEGER NOT NULL,
    base_health INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (expansion_id) REFERENCES tagteam.expansion(id)
);

CREATE TABLE tagteam.player (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    display_name VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE tagteam.partner (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    hero_id UUID NOT NULL,
    final_power INTEGER NOT NULL,
    final_health INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (hero_id) REFERENCES tagteam.hero(id)
);

CREATE TABLE tagteam.team (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    left_partner_id UUID NOT NULL,
    right_partner_id UUID NOT NULL,
    player_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (left_partner_id) REFERENCES tagteam.partner(id),
    FOREIGN KEY (right_partner_id) REFERENCES tagteam.partner(id),
    FOREIGN KEY (player_id) REFERENCES tagteam.player(id)
);

CREATE TABLE tagteam.win_condition (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
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
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (team_one_id) REFERENCES tagteam.team(id),
    FOREIGN KEY (team_two_id) REFERENCES tagteam.team(id),
    FOREIGN KEY (winner_id) REFERENCES tagteam.player(id),
    FOREIGN KEY (loser_id) REFERENCES tagteam.player(id),
    FOREIGN KEY (win_condition_id) REFERENCES tagteam.win_condition(id)
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
DROP TABLE IF EXISTS tagteam.team;
DROP TABLE IF EXISTS tagteam.partner;
DROP TABLE IF EXISTS tagteam.player;
DROP TABLE IF EXISTS tagteam.hero;
DROP SCHEMA IF EXISTS tagteam;
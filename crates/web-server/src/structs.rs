use chrono::{DateTime, Utc};

struct Expansion {
    id: String,
    title: String,
    created_at: DateTime<Utc>,
}

struct Hero {
    id: String,
    display_name: String,
    expansion: String,
    base_power: u32,
    base_health: u32,
    created_at: DateTime<Utc>,
}

struct Partner<'a> {
    id: String,
    hero: &'a Hero,
    final_power: u32,
    final_health: u32,
    created_at: DateTime<Utc>,
}

struct Player {
    id: String,
    display_name: String,
    created_at: DateTime<Utc>,
}

struct Team<'a> {
    id: String,
    left_partner: &'a Partner<'a>,
    right_partner: &'a Partner<'a>,
    created_at: DateTime<Utc>,
}

struct WinCondition {
    id: String,
    title: String,
    description: String,
    created_at: DateTime<Utc>,
}

struct Game<'a> {
    id: String,
    team_one: &'a Team<'a>,
    team_two: &'a Team<'a>,
    winner: &'a Player,
    loser: &'a Player,
    win_condition: &'a WinCondition,
    created_at: DateTime<Utc>,
}

struct BestOfMatchGame<'a> {
    game: &'a Game<'a>,
    best_of_match: &'a BestOfMatch<'a>, 
    order: u32,
}

struct BestOfMatch<'a> {
    id: String,
    games: Vec<&'a Game<'a>>,
    winner: &'a Player,
    loser: &'a Player,
    created_at: DateTime<Utc>,
}

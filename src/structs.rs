struct Hero {
    uuid: String,
    name: String,
    expansion: String,
    base_power: u32,
    base_health: u32,
}

struct Partner<'a> {
    uuid: String,
    hero: &'a Hero,
    final_power: u32,
    final_health: u32,
}

struct Player {
    uuid: String,
    name: String,
}

struct Team<'a> {
    uuid: String,
    left_partner: &'a Partner<'a>,
    right_partner: &'a Partner<'a>,
}

struct WinCondition {
    uuid: String,
    title: String,
    description: String,
}

struct Game<'a> {
    uuid: String,
    team_one: &'a Team<'a>,
    team_two: &'a Team<'a>,
    winner: &'a Player,
    loser: &'a Player,
    win_condition: &'a WinCondition,
}

struct Match<'a> {
    uuid: String,
    games: Vec<&'a Game<'a>>,
    winner: &'a Player,
    loser: &'a Player,
}

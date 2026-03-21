use db::mutation::Mutation;
use db::query::Query;
use rand::Rng;
use sea_orm::DatabaseConnection;

pub enum Command {
    Hell,
    Honk,
    Ping,
    DogFonts,
    MoreMoles,
    // TODO: implement once per day limit
    ShinyRoll,
    // TODO: add Team X Badge to bag
    AMBeef,
    Followage,
    TIDRoll,
    SIDRoll,
    // TODO: The next four need to actually roll and store a PID
    // TODO: MarkCheck needs to add an item to the bad
    MarkCheck,
    WhaleRoll,
    // TODO: implement
    TileRoll,
    MoleRoll,
    TIDLookup,
    SIDLookup,
}

impl Command {
    pub fn find(trigger: &str) -> Option<Self> {
        match trigger {
            s if !s.starts_with("!") => None,
            s if s.starts_with("!he'll") => Some(Command::Hell),
            s if s.starts_with("!honk") => Some(Command::Honk),
            s if s.starts_with("!ping") => Some(Command::Ping),
            s if s.starts_with("!dogfonts") => Some(Command::DogFonts),
            s if s.starts_with("!more moles") => Some(Command::MoreMoles),
            s if s.starts_with("!tidroll") => Some(Command::TIDRoll),
            s if s.starts_with("!tid") => Some(Command::TIDLookup),
            s if s.starts_with("!sidroll") => Some(Command::SIDRoll),
            s if s.starts_with("!sid") => Some(Command::SIDLookup),
            s if s.starts_with("!ambeef") => Some(Command::AMBeef),
            s if s.starts_with("!followage") => Some(Command::Followage),
            s if s.starts_with("!shinyroll") => Some(Command::ShinyRoll),
            s if s.starts_with("!markcheck") => Some(Command::MarkCheck),
            s if s.starts_with("!moleroll") => Some(Command::MoleRoll),
            s if s.starts_with("!whaleroll") => Some(Command::WhaleRoll),
            _ => None,
        }
    }

    pub fn get_id(&self) -> i64 {
        match self {
            Command::Hell => 1,
            Command::Honk => 2,
            Command::Ping => 3,
            Command::DogFonts => 4,
            Command::MoreMoles => 5,
            Command::ShinyRoll => 6,
            Command::AMBeef => 7,
            Command::Followage => 8,
            Command::TIDRoll => 9,
            Command::SIDRoll => 10,
            Command::MarkCheck => 11,
            Command::WhaleRoll => 12,
            Command::TileRoll => 13,
            Command::MoleRoll => 14,
            Command::TIDLookup => 15,
            Command::SIDLookup => 16,
        }
    }

    pub async fn run(
        &self,
        _body: String,
        chatter_id: i64,
        broadcaster_id: &str,
        conn: DatabaseConnection,
    ) -> Option<String> {
        // Store the fact that a command was ran
        let command_id = self.get_id();
        let conn_clone = conn.clone();
        tokio::spawn(async move {
            let _ = Mutation::inc_chatter_command_count(&conn_clone, chatter_id, command_id)
                .await
                .inspect_err(|e| tracing::warn!("Failed to count chatter command: {e}"));
        });

        match self {
            Command::Hell => Some("yeag".into()),
            Command::Honk => Some("honk 🪿".into()),
            Command::Ping => Some("pong".into()),
            Command::DogFonts => Some("he'll yeag".into()),
            Command::MoreMoles => Some("more holes".into()),
            Command::TIDRoll => run_tidroll(chatter_id, &conn).await,
            Command::TIDLookup => run_tidlookup(chatter_id, &conn).await,
            Command::AMBeef => run_ambeef().await,
            Command::Followage => run_followage().await,
            Command::SIDRoll => run_sidroll(chatter_id, &conn).await,
            Command::SIDLookup => Some("Your new SID is *****".into()),
            Command::ShinyRoll => run_shinyroll(broadcaster_id).await,
            Command::MarkCheck => run_markcheck().await,
            Command::MoleRoll => run_moleroll().await,
            Command::WhaleRoll => run_whaleroll().await,
            _ => todo!(),
        }
    }
}

async fn run_tidlookup(chatter_id: i64, conn: &DatabaseConnection) -> Option<String> {
    let Ok(Some(chatter)) = Query::find_chatter_by_id(conn, chatter_id).await else {
        return Some("Something went wrong, go yell at Fex".into());
    };

    return Some(format!("Your current TID is {:05}", chatter.tid));
}

async fn run_tidroll(chatter_id: i64, conn: &DatabaseConnection) -> Option<String> {
    let num = rand::rng().random_range(0..65536);

    let Ok(_) = Mutation::set_chatter_tid(conn, chatter_id, num).await else {
        return Some("Something went wrong, go yell at Fex".into());
    };

    return Some(format!("Your new TID is {:05}", num));
}

async fn run_sidroll(chatter_id: i64, conn: &DatabaseConnection) -> Option<String> {
    let num = rand::rng().random_range(0..65536);

    let Ok(_) = Mutation::set_chatter_sid(conn, chatter_id, num).await else {
        return Some("Something went wrong, go yell at Fex".into());
    };

    return Some(format!("Your new SID is *****"));
}

async fn run_ambeef() -> Option<String> {
    let name = match rand::rng().random_range(1..3) {
        1 => "ShinyCatherine",
        _ => "CannedWolfMeat",
    };
    return Some(format!(
        "You are team {} (unless you reroll, I'm a chatbot not your boss)",
        name
    ));
}

async fn run_followage() -> Option<String> {
    let mut num = rand::rng().random_range(1..31536000);
    if rand::rng().random_range(1..10) == 5 {
        num += rand::rng().random_range(31536000..315360000);
    }
    let mut years = 0;
    while num >= 31536000 {
        years += 1;
        num -= 31536000;
    }
    let mut months = 0;
    while num >= 2628000 {
        months += 1;
        num -= 2628000;
    }
    let mut days = 0;
    while num >= 86400 {
        days += 1;
        num -= 86400;
    }
    return Some(format!(
        "You have been following for {} years, {} months, and {} days",
        years, months, days
    ));
}

async fn run_shinyroll(broadcaster_id: &str) -> Option<String> {
    if broadcaster_id == "861073341" {
        return None;
    } // Yarnity
    if broadcaster_id == "29002848" {
        return None;
    } // Kwikpanik
    let num = rand::rng().random_range(1..8193);
    return Some(format!("You rolled {:04}", num));
}

async fn run_markcheck() -> Option<String> {
    let t = match rand::rng().random_range(1..101) {
        1 => "a Destiny Mark yarnie1Hype",
        2 => "an Itemfinder Mark yarnie1Hype",
        3 => "a Gourmand Mark yarnie1Hype",
        4 => "a Jumbo Mark yarnie1Hype",
        5 => "a Mightiest Mark yarnie1Hype",
        6 => "a Jumbo Mark yarnie1Hype",
        7 => "a Partner Mark yarnie1Hype",
        8 => "a Titan Mark yarnie1Hype",
        9 => "an Alpha Mark yarnie1Hype",
        10 => "a Lunchtime Mark yarnie1Hype",
        11 => "a Sleepy-Time Mark yarnie1Yawn",
        12 => "a Dusk Mark yarnie1Hype",
        13 => "a Dawn Mark yarnie1Hype",
        14 => "a Cloudy Mark yarnie1Hype",
        15 => "a Rainy Mark yarnie1Hype",
        16 => "a Stormy Mark yarnie1Hype",
        17 => "a Snowy Mark yarnie1Hype",
        18 => "a Blizzard Mark yarnie1Hype",
        19 => "a Sandstorm Mark yarnie1Hype",
        20 => "a Misty Mark yarnie1Hype",
        21 => "a Rare Mark yarnie1Hype",
        22 => "an Uncommon Mark yarnie1Hype",
        23 => "a Rowdy Mark yarnie1Hype",
        24 => "an Absent-Minded Mark yarnie1Hype",
        25 => "a Jittery Mark yarnie1Hype",
        26 => "an Excited Mark yarnie1Hype",
        27 => "a Charismatic Mark yarnie1Hype",
        28 => "a Calmness Mark yarnie1Hype",
        29 => "an Intense Mark yarnie1Hype",
        30 => "a Zoned-Out Mark yarnie1Hype",
        31 => "a Joyful Mark yarnie1Hype",
        32 => "an Angry Mark yarnie1Hype",
        33 => "a Smiley Mark yarnie1Hype",
        34 => "a Teary Mark yarnie1Cry",
        35 => "an Upbeat Mark yarnie1Hype",
        36 => "a Peeved Mark yarnie1Hype",
        37 => "an Intellectual Mark yarnie1Hype",
        38 => "a Ferocious Mark yarnie1Hype",
        39 => "a Crafty Mark yarnie1Hype",
        40 => "a Scowling Mark yarnie1Hype",
        41 => "a Kindly Mark yarnie1Hype",
        42 => "a Flustered Mark yarnie1Hype",
        43 => "a Pumped-Up Mark yarnie1Hype",
        44 => "a Zero Energy Mark yarnie1Hype",
        45 => "a Prideful Mark yarnie1Hype",
        46 => "an Unsure Mark yarnie1Hype",
        47 => "a Humble Mark yarnie1Hype",
        48 => "a Thorny Mark yarnie1Hype",
        49 => "a Vigor Mark yarnie1Hype",
        50 => "a Slump Mark yarnie1Hype",
        _ => "no mark yarnie1Cry",
    };

    return Some(format!("You found {}", t));
}

async fn run_moleroll() -> Option<String> {
    let msg = match rand::rng().random_range(1..11) {
        1 => {
            let t = match rand::rng().random_range(1..13) {
                1 => "an everstone",
                2 => "a sun stone",
                3 => "a moon stone",
                4 => "a fire stone",
                5 => "a water stone",
                6 => "a thunder stone",
                7 => "a leaf stone",
                8 => "a shiny stone",
                9 => "a dusk stone",
                10 => "a dawn stone",
                11 => "a hard stone",
                _ => "an oval stone",
            };

            format!("You got {}", t)
        }
        2 | 3 | 4 | 5 | 6 => {
            let t = match rand::rng().random_range(1..18) {
                1 => "a fire",
                2 => "a water",
                3 => "an electric",
                4 => "a grass",
                5 => "an ice",
                6 => "a fighting",
                7 => "a poison",
                8 => "a ground",
                9 => "a flying",
                10 => "a psychic",
                11 => "a bug",
                12 => "a rock",
                13 => "a ghost",
                14 => "a dragon",
                15 => "a dark",
                16 => "a steel",
                _ => "a normal",
            };

            format!("You got {} gem", t)
        }
        _ => {
            if rand::rng().random_range(1..8193) == 8192 {
                "✨✨✨You got a PINK mole✨✨✨".into()
            } else {
                "You got a mole".into()
            }
        }
    };
    return Some(msg);
}

async fn run_whaleroll() -> Option<String> {
    let mins = rand::rng().random_range(1..35);
    tokio::time::sleep(std::time::Duration::from_secs(mins)).await;

    if rand::rng().random_range(1..8193) == 8192 {
        return Some("✨✨✨You found a PINK whale✨✨✨".into());
    }

    return Some("You found a whale".into());
}

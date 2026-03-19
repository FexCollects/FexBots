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
    MarkCheck, // todo
    WhaleRoll, // todo
    TileRoll, // todo
    MoleRoll, // todo
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
        conn: &DatabaseConnection,
    ) -> Option<String> {
        // Store the fact that a command was ran
        if let Err(_) = Mutation::inc_chatter_command_count(conn, chatter_id, self.get_id()).await {
            tracing::warn!("Failed to count chatter command");
        }

        match self {
            Command::Hell => Some("yeag".into()),
            Command::Honk => Some("honk 🪿".into()),
            Command::Ping => Some("pong".into()),
            Command::DogFonts => Some("he'll yeag".into()),
            Command::MoreMoles => Some("more holes".into()),
            Command::TIDRoll => run_tidroll(chatter_id, conn).await,
            Command::TIDLookup => run_tidlookup(chatter_id, conn).await,
            Command::AMBeef => run_ambeef().await,
            Command::Followage => run_followage().await,
            Command::SIDRoll => run_sidroll(chatter_id, conn).await,
            Command::SIDLookup => Some("Your new SID is *****".into()),
            Command::ShinyRoll => run_shinyroll(broadcaster_id).await,
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
    return Some(format!("You have been following for {} years, {} months, and {} days", years, months, days));
}

async fn run_shinyroll(broadcaster_id: &str) -> Option<String> {
    if broadcaster_id == "861073341" { return None; } // Yarnity
    if broadcaster_id == "29002848" { return None; } // Kwikpanik
    let num = rand::rng().random_range(1..8193);
    return Some(format!("You rolled {:04}", num));
}

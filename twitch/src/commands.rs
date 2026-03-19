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
    ShinyRoll,
    AMBeef,
    Followage,
    TIDRoll,
    SIDRoll,
    MarkCheck,
    WhaleRoll,
    TileRoll,
    MoleRoll,
    TIDLookup,
}

impl Command {
    pub fn find(trigger: &str) -> Option<Self> {
        match trigger {
            s if s.starts_with("!he'll") => Some(Command::Hell),
            s if s.starts_with("!honk") => Some(Command::Honk),
            s if s.starts_with("!ping") => Some(Command::Ping),
            s if s.starts_with("!dogfonts") => Some(Command::DogFonts),
            s if s.starts_with("!more moles") => Some(Command::MoreMoles),
            s if s.starts_with("!tidroll") => Some(Command::TIDRoll),
            s if s.starts_with("!tid") => Some(Command::TIDLookup),
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
        }
    }

    pub async fn run(
        &self,
        _body: String,
        chatter_id: i64,
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

/*
    if shiny_roll_enabled && msg.starts_with("!shinyroll") {
    if msg.starts_with("!ambeef") {
    if msg.starts_with("!followage") {
    if msg.starts_with("!tidroll") {
    if msg.starts_with("!sidroll") {
    if msg.starts_with("!markcheck") {
    if msg.starts_with("!whaleroll") {
    if msg.starts_with("!tileroll ") {
    if msg.starts_with("!moleroll") {
*/

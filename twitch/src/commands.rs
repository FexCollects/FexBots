use chrono::{Duration, Utc};
use db::mutation::Mutation;
use db::query::Query;
use rand::prelude::*;
use sea_orm::DatabaseConnection;

static MOVES: &'static [&'static str] = &[
    "POUND",
    "KARATE CHOP",
    "DOUBLE SLAP",
    "COMET PUNCH",
    "MEGA PUNCH",
    "PAY DAY",
    "FIRE PUNCH",
    "ICE PUNCH",
    "THUNDER PUNCH",
    "SCRATCH",
    "VICE GRIP",
    "GUILLOTINE",
    "RAZOR WIND",
    "SWORDS DANCE",
    "CUT",
    "GUST",
    "WING ATTACK",
    "WHIRLWIND",
    "FLY",
    "BIND",
    "SLAM",
    "VINE WHIP",
    "STOMP",
    "DOUBLE KICK",
    "MEGA KICK",
    "JUMP KICK",
    "ROLLING KICK",
    "SAND ATTACK",
    "HEADBUTT",
    "HORN ATTACK",
    "FURY ATTACK",
    "HORN DRILL",
    "TACKLE",
    "BODY SLAM",
    "WRAP",
    "TAKE DOWN",
    "THRASH",
    "DOUBLE-EDGE",
    "TAIL WHIP",
    "POISON STING",
    "TWINEEDLE",
    "PIN MISSILE",
    "LEER",
    "BITE",
    "GROWL",
    "ROAR",
    "SING",
    "SUPERSONIC",
    "SONIC BOOM",
    "DISABLE",
    "ACID",
    "EMBER",
    "FLAMETHROWER",
    "MIST",
    "WATER GUN",
    "HYDRO PUMP",
    "SURF",
    "ICE BEAM",
    "BLIZZARD",
    "PSYBEAM",
    "BUBBLE BEAM",
    "AURORA BEAM",
    "HYPER BEAM",
    "PECK",
    "DRILL PECK",
    "SUBMISSION",
    "LOW KICK",
    "COUNTER",
    "SEISMIC TOSS",
    "STRENGTH",
    "ABSORB",
    "MEGA DRAIN",
    "LEECH SEED",
    "GROWTH",
    "RAZOR LEAF",
    "SOLAR BEAM",
    "POISON POWDER",
    "STUN SPORE",
    "SLEEP POWDER",
    "PETAL DANCE",
    "STRING SHOT",
    "DRAGON RAGE",
    "FIRE SPIN",
    "THUNDER SHOCK",
    "THUNDERBOLT",
    "THUNDER WAVE",
    "THUNDER",
    "ROCK THROW",
    "EARTHQUAKE",
    "FISSURE",
    "DIG",
    "TOXIC",
    "CONFUSION",
    "PSYCHIC",
    "HYPNOSIS",
    "MEDITATE",
    "AGILITY",
    "QUICK ATTACK",
    "RAGE",
    "TELEPORT",
    "NIGHT SHADE",
    "MIMIC",
    "SCREECH",
    "DOUBLE TEAM",
    "RECOVER",
    "HARDEN",
    "MINIMIZE",
    "SMOKESCREEN",
    "CONFUSE RAY",
    "WITHDRAW",
    "DEFENSE CURL",
    "BARRIER",
    "LIGHT SCREEN",
    "HAZE",
    "REFLECT",
    "FOCUS ENERGY",
    "BIDE",
    "METRONOME",
    "MIRROR MOVE",
    "SELF-DESTRUCT",
    "EGG BOMB",
    "LICK",
    "SMOG",
    "SLUDGE",
    "BONE CLUB",
    "FIRE BLAST",
    "WATERFALL",
    "CLAMP",
    "SWIFT",
    "SKULL BASH",
    "SPIKE CANNON",
    "CONSTRICT",
    "AMNESIA",
    "KINESIS",
    "SOFT-BOILED",
    "HI JUMP KICK",
    "GLARE",
    "DREAM EATER",
    "POISON GAS",
    "BARRAGE",
    "LEECH LIFE",
    "LOVELY KISS",
    "SKY ATTACK",
    "TRANSFORM",
    "BUBBLE",
    "DIZZY PUNCH",
    "SPORE",
    "FLASH",
    "PSYWAVE",
    "SPLASH",
    "ACID ARMOR",
    "CRABHAMMER",
    "EXPLOSION",
    "FURY SWIPES",
    "BONEMERANG",
    "REST",
    "ROCK SLIDE",
    "HYPER FANG",
    "SHARPEN",
    "CONVERSION",
    "TRI ATTACK",
    "SUPER FANG",
    "SLASH",
    "SUBSTITUTE",
    "STRUGGLE",
    "SKETCH",
    "TRIPLE KICK",
    "THIEF",
    "SPIDER WEB",
    "MIND READER",
    "NIGHTMARE",
    "FLAME WHEEL",
    "SNORE",
    "CURSE",
    "FLAIL",
    "CONVERSION 2",
    "AEROBLAST",
    "COTTON SPORE",
    "REVERSAL",
    "SPITE",
    "POWDER SNOW",
    "PROTECT",
    "MACH PUNCH",
    "SCARY FACE",
    "FAINT ATTACK",
    "SWEET KISS",
    "BELLY DRUM",
    "SLUDGE BOMB",
    "MUD-SLAP",
    "OCTAZOOKA",
    "SPIKES",
    "ZAP CANNON",
    "FORESIGHT",
    "DESTINY BOND",
    "PERISH SONG",
    "ICY WIND",
    "DETECT",
    "BONE RUSH",
    "LOCK-ON",
    "OUTRAGE",
    "SANDSTORM",
    "GIGA DRAIN",
    "ENDURE",
    "CHARM",
    "ROLLOUT",
    "FALSE SWIPE",
    "SWAGGER",
    "MILK DRINK",
    "SPARK",
    "FURY CUTTER",
    "STEEL WING",
    "MEAN LOOK",
    "ATTRACT",
    "SLEEP TALK",
    "HEAL BELL",
    "RETURN",
    "PRESENT",
    "FRUSTRATION",
    "SAFEGUARD",
    "PAIN SPLIT",
    "SACRED FIRE",
    "MAGNITUDE",
    "DYNAMIC PUNCH",
    "MEGAHORN",
    "DRAGON BREATH",
    "BATON PASS",
    "ENCORE",
    "PURSUIT",
    "RAPID SPIN",
    "SWEET SCENT",
    "IRON TAIL",
    "METAL CLAW",
    "VITAL THROW",
    "MORNING SUN",
    "SYNTHESIS",
    "MOONLIGHT",
    "HIDDEN POWER",
    "CROSS CHOP",
    "TWISTER",
    "RAIN DANCE",
    "SUNNY DAY",
    "CRUNCH",
    "MIRROR COAT",
    "PSYCH UP",
    "EXTREME SPEED",
    "ANCIENT POWER",
    "SHADOW BALL",
    "FUTURE SIGHT",
    "ROCK SMASH",
    "WHIRLPOOL",
    "BEAT UP",
    "FAKE OUT",
    "UPROAR",
    "STOCKPILE",
    "SPIT UP",
    "SWALLOW",
    "HEAT WAVE",
    "HAIL",
    "TORMENT",
    "FLATTER",
    "WILL-O-WISP",
    "MEMENTO",
    "FACADE",
    "FOCUS PUNCH",
    "SMELLING SALT",
    "FOLLOW ME",
    "NATURE POWER",
    "CHARGE",
    "TAUNT",
    "HELPING HAND",
    "TRICK",
    "ROLE PLAY",
    "WISH",
    "ASSIST",
    "INGRAIN",
    "SUPERPOWER",
    "MAGIC COAT",
    "RECYCLE",
    "REVENGE",
    "BRICK BREAK",
    "YAWN",
    "KNOCK OFF",
    "ENDEAVOR",
    "ERUPTION",
    "SKILL SWAP",
    "IMPRISON",
    "REFRESH",
    "GRUDGE",
    "SNATCH",
    "SECRET POWER",
    "DIVE",
    "ARM THRUST",
    "CAMOUFLAGE",
    "TAIL GLOW",
    "LUSTER PURGE",
    "MIST BALL",
    "FEATHER DANCE",
    "TEETER DANCE",
    "BLAZE KICK",
    "MUD SPORT",
    "ICE BALL",
    "NEEDLE ARM",
    "SLACK OFF",
    "HYPER VOICE",
    "POISON FANG",
    "CRUSH CLAW",
    "BLAST BURN",
    "HYDRO CANNON",
    "METEOR MASH",
    "ASTONISH",
    "WEATHER BALL",
    "AROMATHERAPY",
    "FAKE TEARS",
    "AIR CUTTER",
    "OVERHEAT",
    "ODOR SLEUTH",
    "ROCK TOMB",
    "SILVER WIND",
    "METAL SOUND",
    "GRASS WHISTLE",
    "TICKLE",
    "COSMIC POWER",
    "WATER SPOUT",
    "SIGNAL BEAM",
    "SHADOW PUNCH",
    "EXTRASENSORY",
    "SKY UPPERCUT",
    "SAND TOMB",
    "SHEER COLD",
    "MUDDY WATER",
    "BULLET SEED",
    "AERIAL ACE",
    "ICICLE SPEAR",
    "IRON DEFENSE",
    "BLOCK",
    "HOWL",
    "DRAGON CLAW",
    "FRENZY PLANT",
    "BULK UP",
    "BOUNCE",
    "MUD SHOT",
    "POISON TAIL",
    "COVET",
    "VOLT TACKLE",
    "MAGICAL LEAF",
    "WATER SPORT",
    "CALM MIND",
    "LEAF BLADE",
    "DRAGON DANCE",
    "ROCK BLAST",
    "SHOCK WAVE",
    "WATER PULSE",
    "DOOM DESIRE",
    "PSYCHO BOOST",
];

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
    TileRoll,
    MoleRoll,
    TIDLookup,
    SIDLookup,
    Metronome,
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
            s if s.starts_with("!tileroll") => Some(Command::TileRoll),
            s if s.starts_with("!metronome") => Some(Command::Metronome),
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
            Command::Metronome => 17,
        }
    }

    pub async fn run(
        &self,
        body: String,
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
            Command::TileRoll => run_tileroll(body).await,
            Command::Metronome => run_metronome().await,
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
    let mut rng = rand::rng();

    let names = vec![
        "ShinyCatherine",
        "CannedWolfMeat",
    ];
    let Some(name) = names.choose(&mut rng) else {
        return Some("Something went wrong, go yell at Fex".into());
    };

    return Some(format!(
        "You are team {} (unless you reroll, I'm a chatbot not your boss)",
        name
    ));
}

async fn run_followage() -> Option<String> {
    let mut rng = rand::rng();
    let mut num = rng.random_range(1..31536000);
    if rng.random_range(1..10) == 5 {
        num += rng.random_range(31536000..315360000);
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
    let mut rng = rand::rng();
    let choices = vec![
        (1, "a Destiny Mark yarnie1Hype"),
        (1, "an Itemfinder Mark yarnie1Hype"),
        (1, "a Gourmand Mark yarnie1Hype"),
        (1, "a Jumbo Mark yarnie1Hype"),
        (1, "a Mightiest Mark yarnie1Hype"),
        (1, "a Jumbo Mark yarnie1Hype"),
        (1, "a Partner Mark yarnie1Hype"),
        (1, "a Titan Mark yarnie1Hype"),
        (1, "an Alpha Mark yarnie1Hype"),
        (1, "a Lunchtime Mark yarnie1Hype"),
        (1, "a Sleepy-Time Mark yarnie1Yawn"),
        (1, "a Dusk Mark yarnie1Hype"),
        (1, "a Dawn Mark yarnie1Hype"),
        (1, "a Cloudy Mark yarnie1Hype"),
        (1, "a Rainy Mark yarnie1Hype"),
        (1, "a Stormy Mark yarnie1Hype"),
        (1, "a Snowy Mark yarnie1Hype"),
        (1, "a Blizzard Mark yarnie1Hype"),
        (1, "a Sandstorm Mark yarnie1Hype"),
        (1, "a Misty Mark yarnie1Hype"),
        (1, "a Rare Mark yarnie1Hype"),
        (1, "an Uncommon Mark yarnie1Hype"),
        (1, "a Rowdy Mark yarnie1Hype"),
        (1, "an Absent-Minded Mark yarnie1Hype"),
        (1, "a Jittery Mark yarnie1Hype"),
        (1, "an Excited Mark yarnie1Hype"),
        (1, "a Charismatic Mark yarnie1Hype"),
        (1, "a Calmness Mark yarnie1Hype"),
        (1, "an Intense Mark yarnie1Hype"),
        (1, "a Zoned-Out Mark yarnie1Hype"),
        (1, "a Joyful Mark yarnie1Hype"),
        (1, "an Angry Mark yarnie1Hype"),
        (1, "a Smiley Mark yarnie1Hype"),
        (1, "a Teary Mark yarnie1Cry"),
        (1, "an Upbeat Mark yarnie1Hype"),
        (1, "a Peeved Mark yarnie1Hype"),
        (1, "an Intellectual Mark yarnie1Hype"),
        (1, "a Ferocious Mark yarnie1Hype"),
        (1, "a Crafty Mark yarnie1Hype"),
        (1, "a Scowling Mark yarnie1Hype"),
        (1, "a Kindly Mark yarnie1Hype"),
        (1, "a Flustered Mark yarnie1Hype"),
        (1, "a Pumped-Up Mark yarnie1Hype"),
        (1, "a Zero Energy Mark yarnie1Hype"),
        (1, "a Prideful Mark yarnie1Hype"),
        (1, "an Unsure Mark yarnie1Hype"),
        (1, "a Humble Mark yarnie1Hype"),
        (1, "a Thorny Mark yarnie1Hype"),
        (1, "a Vigor Mark yarnie1Hype"),
        (1, "a Slump Mark yarnie1Hype"),
        (50, "no mark yarnie1Cry"),
    ];

    let Ok((_,choice)) = choices.choose_weighted(&mut rng, |item| { item.0 }) else {
        return Some("Something went wrong, go yell at Fex".into());
    };

    return Some(format!("You found {}", choice));
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
    let mins = rand::rng().random_range(1..35) * 60;
    tokio::time::sleep(std::time::Duration::from_secs(mins)).await;

    if rand::rng().random_range(1..8193) == 8192 {
        return Some("✨✨✨You found a PINK whale✨✨✨".into());
    }

    return Some("You found a whale".into());
}

fn get_daily_tile() -> u32 {
    let date_time = Utc::now() - Duration::hours(6);
    let formatted = format!("{}", date_time.format("%Y%m%d"));
    let seed = formatted.parse::<u64>().unwrap();
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    rng.random::<u32>() % 100 + 1
}

async fn run_tileroll(body: String) -> Option<String> {
    let Some(tile) = body.strip_prefix("!tileroll ") else {
        return Some("try !tileroll 999".into());
    };

    let Ok(tile) = tile.parse::<u32>() else {
        return Some("try !tileroll 999".into());
    };

    if tile < 1 || tile > 100 {
        return Some("tile must be between 1 and 100!".into());
    }

    let rng_num = rand::rng().random_range(1..101);
    let daily = get_daily_tile();
    tracing::warn!("Daily tile: {daily}");
    let pokemon = if tile == daily {
        match rng_num {
            1..=50 => "feebas",
            51..=60 => "tentacool",
            61..=90 => "magikarp",
            _ => "carvanha",
        }
    } else {
        match rng_num {
            1..=20 => "tentacool",
            21..=80 => "magikarp",
            _ => "carvanha",
        }
    };

    let shiny = rand::rng().random_range(1..8193);
    let resp = if shiny == 8192 {
        format!("✨✨✨You reeled in a SHINY {}✨✨✨", pokemon)
    } else {
        format!("You reeled in a {}", pokemon)
    };

    return Some(resp);
}

async fn run_metronome() -> Option<String> {
    let idx = rand::rng().random_range(0..MOVES.len());
    return Some(format!("You used {}!", MOVES[idx]));
}

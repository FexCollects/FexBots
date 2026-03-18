pub type CmdRunFn = fn(String) -> Option<String>;

pub trait BotCommand {
    const TRIGGER: &str;
    fn run(command: String) -> Option<String>;
}

macro_rules! simple_response_command {
    ($cmd_name:ident, $trigger:literal, $response:literal) => {
        pub struct $cmd_name;
        impl BotCommand for $cmd_name {
            const TRIGGER: &str = $trigger;
            fn run(_command: String) -> Option<String> {
                return Some($response.into());
            }
        }
    };
}

simple_response_command!(PingCommand, "!ping", "pong");
simple_response_command!(DogFontsCommand, "!dogfonts", "he'll yeag");
simple_response_command!(MoreMolesCommand, "!more moles", "more holes");

// TODO: Create a Chatters table and a Commands table
// Record an entry anytime someone runs a command

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

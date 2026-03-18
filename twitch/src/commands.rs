pub type CmdRunFn = fn(String) -> Option<String>;

pub trait BotCommand {
    const TRIGGER: &str;
    fn run(command: String) -> Option<String>;
}

pub struct PingCommand;
impl BotCommand for PingCommand {
    const TRIGGER: &str = "!ping";
    fn run(_command: String) -> Option<String> {
        return Some("pong".into());
    }
}

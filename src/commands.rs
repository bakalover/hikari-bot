use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Поддерживаемые команды:")]
pub(crate) enum Command {
    #[command(description = "справка по командам")]
    Help,
    #[command(description = "поиск слова/кандзи")]
    Search(String),
    #[command(description = "начать игру в ширитори")]
    Shiritory,
}

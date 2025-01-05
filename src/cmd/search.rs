use teloxide::prelude::*;

use crate::search::ISearch;

pub(super) async fn search<TSearch: ISearch>(
    bot: Bot,
    msg: Message,
    query: &str,
) -> ResponseResult<()> {
    Ok(())
}

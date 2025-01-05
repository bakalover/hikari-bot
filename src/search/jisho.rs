use super::ISearch;

pub(crate) struct Jisho {}
impl ISearch for Jisho {
    fn search(_query: &str) -> Result<(), ()> {
        Ok(())
    }
}

// let reply = reqwest::get(format!(
//     "https://jisho.org/api/v1/search/words?keyword={}",
//     query
// ))
// .await?
// .text()
// .await
// .unwrap();

// let truncated_message = if reply.len() > 4096 {
//     &reply[..4096]
// } else {
//     &reply
// };

// bot.send_message(msg.chat.id, truncated_message)
//     .message_thread_id(msg.thread_id.unwrap())
//     .await?;

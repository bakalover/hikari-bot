use super::ISearch;

pub(crate) struct JMDict {}
impl ISearch for JMDict {
    fn search(_query: &str) -> Result<(), ()> {
        Ok(())
    }
}

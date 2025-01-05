pub(crate) mod jisho;
pub(crate) mod jmdict;
pub(crate) trait ISearch {
    fn search(_query: &str) -> Result<(), ()> {
        Ok(())
    }
}

use std::io::Result;

pub trait Backend {
    /// pushes the data to the backend
    ///
    /// `push` should handle how the data is stored, ie: double linkedlist...etc
    fn push(&mut self, key: &[u8], data: &[u8]) -> Result<()>;

    /// gets the data associated with key provided from the backend
    ///
    /// `fetch` should handle how the data is fetched
    fn fetch(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;
}

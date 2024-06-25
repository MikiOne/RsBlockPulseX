
mod store;
mod error;

pub trait Store {
fn get(&self, key: &str) -> Option<String>;

}



/// A struct for interfacing with the PokeAPI REST API.
pub struct PokeAPIClient {

}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum APIError {
    ResourceNotFound
}

impl PokeAPIClient {
    pub fn init() -> Self {
        Self {}
    }

    pub fn retrieve_pokemon_by_id(_id: usize) {
    }
}

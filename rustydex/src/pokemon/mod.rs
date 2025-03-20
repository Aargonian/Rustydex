mod species;
pub mod evolution;
pub mod item;
pub mod moves;

pub use species::*;

/// Stub enum for gender
#[derive(Debug, Copy, Clone)]
pub enum PokemonGender {
    Male,
    Female,
}

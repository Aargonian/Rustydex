use crate::types::Type;

use super::evolution::Evolution;

const DEFAULT_CAPACITY: usize = 2048;

/// Simple newtype for Pokemon Species Index.
#[derive(Debug, Copy, Clone, Default)]
pub struct PokemonSpecies(u32);

/// Simple type for pokemon typing.
#[derive(Debug, Copy, Clone, Default)]
pub struct Typing {
    primary: Type,
    secondary: Option<Type>,
}

/// Simple newtype for height and weight, which are scaled integers
#[derive(Debug, Copy, Clone, Default)]
pub struct ScaledInteger(u32);

/// Simple newtype for abilities. Temporary.
#[derive(Debug, Clone, Default)]
pub struct Ability(String);

#[derive(Debug, Copy, Clone, Default)]
pub struct BaseStats {
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub sp_attack: u8,
    pub sp_defense: u8,
    pub speed: u8,
}

impl BaseStats {
    pub fn total(&self) -> u16 {
        (self.hp as u16)
            + (self.attack as u16)
            + (self.defense as u16)
            + (self.sp_attack as u16)
            + (self.sp_defense as u16)
            + (self.speed as u16)
    }
}

pub struct MiscSpeciesData {
    species_descriptor: String,
    height: ScaledInteger,
    weight: ScaledInteger,
}

/// This structure contains all the data necessary for the context of the application as it relates to the pokemon the
/// application uses. It acts as a "context" which contains all the data for pokemon in an arbitrary pokedex. Usually
/// this is used for the "national dex" which contains data for all pokemon of all generations, but it can be
/// arbitrarily built as well.
#[derive(Debug, Clone)]
pub struct Pokedex {
    /*
     * Implementation Details:
     *
     * This struct is a "Struct of Arrays (SoA)" as a manner of speaking. In theory, we could have
     * an Array of Structs (AoS) of Pokemon Species, but it is not generally common that a user of
     * the pokedex is interested in ALL the data for a given species. Usually, they want to do
     * something like iterate over base stats, abilities, egg groups, or evolution lines. Therefore,
     * it makes more sense for a "Species" to simply be an index that can be used to lookup the
     * relevant data, instead of stuffing all the data into a single Species struct.
     */
    species: Vec<PokemonSpecies>,
    base_stats: Vec<BaseStats>,
    types: Vec<(Type, Option<Type>)>,
    abilities: Vec<(Ability, Option<Ability>, Option<Ability>)>,
    evolutions: Vec<Option<Evolution>>,
}

impl Default for Pokedex {
    fn default() -> Self {
        Self {
            species: Vec::with_capacity(DEFAULT_CAPACITY),
            base_stats: Vec::with_capacity(DEFAULT_CAPACITY),
            types: Vec::with_capacity(DEFAULT_CAPACITY),
            abilities: Vec::with_capacity(DEFAULT_CAPACITY),
            evolutions: Vec::with_capacity(DEFAULT_CAPACITY),
        }
    }
}

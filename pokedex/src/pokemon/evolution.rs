use super::{item::Item, moves::PokemonMove, PokemonGender, PokemonSpecies};

/// An enum for the various days/times/blocks that are relevant for various pokemon, items, etc.
/// Date and Time in pokemon games can get complicated since every game has differences in how it
/// handles date and time, how precisely date and time need to be represented, and how closely date
/// and time map to the real world. This is an incomplete, but hopefully close enough,
/// approximation.
///
/// As most events in pokemon don't tend to care about the precise minute, they can usually be
/// broken down into "during the day", "during the night", "between 6pm and 9pm", "on a Friday",
/// etc. This enum can be combined multiple times to get something more precise like "Between 6pm
/// and 12pm on a Friday".
#[derive(Debug, Copy, Clone)]
pub enum PokemonTime {
    Daytime,
    Nighttime,
    DayOfWeek(u8),
    BetweenHours(u8, u8),
    BetweenMinutes(u8, u8),
}

/// Simple enum for the different restrictions on evolution
#[derive(Debug, Clone)]
pub enum EvolutionRequirement {
    SpecificTime(PokemonTime),
    CurrentlyInArea(String),
    Friendship(u8),
    HoldItem(Item),
    KnowMove(PokemonMove),
    Gender(PokemonGender),
    Other(String),
}

/// Simple enum for the different methods of evolution
#[derive(Debug, Clone, Default)]
pub enum EvolutionMethod {
    #[default]
    LevelUp,
    UseItem(Item),
    Trade,
}


#[derive(Debug, Clone)]
pub struct Evolution {
    from: PokemonSpecies,
    to: PokemonSpecies,
    method: EvolutionMethod,
}

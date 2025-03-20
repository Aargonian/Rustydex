# The Pokemon.json File Format

The Pokemon.json file (and potential other future JSON files) serve as the actual initial data the pokedex uses when
constructing all the data structures at startup. The layout of this file is described below. For the most part, the file
is not hand-authored, and instead most of the data is either directly retrieved or derived from PokeAPI data. As
explained in the [Rationale](#Rationale) section however, the data is by no means bound to only contain (or explicitly
match) the data found in PokeAPI.

## Rationale

Although currently all application data for the pokedex comes courtesy of pokeapi, I've decided to make our own custom
JSON format for the data used by the pokedex. The data in this file still primarily comes from PokeAPI, but undergoes
a conversion process to match the expected format. Right now, this is done manually, but in the future there will be a
conversion process that can do this automatically when changes to the PokeAPI data are detected. Using our own file and
custom format accomplishes a number of goals:

1. Independence from changes to the PokeAPI JSON format. In case the format for PokeAPI ever changes, or even in the
   case that there may be an error in the format, we do not need to immediately change anything about our own data.
2. More convenient data format layout. By using our own custom format, we are free to adapt it in the future to more
   closely match what the code actually works with, easing the development process for new features or working on bugs.
3. No need to interface with pokeapi directly. By the very nature of this being a pokedex application, it will require
   almost all the data related to pokemon species that PokeAPI provides. While PokeAPI encourages cacheing on request, in
   the case of our application it will almost certainly be better if we just bundle all the data for the app rather than
   potentially make many requests to PokeAPI. Likewise, when the app checks for an update the data can come from the
   github repository or a separate server we manage, or even a separate data source the end user provides in the future.
4. Ability to add our own data, or data from other sources, without having to explicitly add more application code. A
   format we control means that in the future, we can simply add any additional data we desire to the files, or even
   pull data from sources other than PokeAPI.

## Format

The file contains a top-level JSON object which contains a little metadata about all the pokemon it contains. This object
primarily contains the array of PokemonSpecies objects.


### Pokedex Root Object Layout
The fields of the top-level object are as follows:

- version: A semver string denoting the current version of the data.
- update: An additional integer that can be considered a part of the version. It denotes the specific build number for
  this version. This value is incremented anytime the data in the file is updated. The application will primarily use
  this and the version string to determine if this version of the file is newer or older than the version it currently
  uses.
- pokemon_count: A non-zero integer value for the number of unique PokemonSpecies entries in the "pokemon" list.
- pokemon: An array of PokemonSpecies objects.

### PokemonSpecies Object Layout

The following fields are defined for PokemonSpecies:
- id - The national dex id number associated with this species.
- name - The primary name used for the species.
- generation - A list of generations this species is available in.
- type1 - The primary Type associated with this species.
- type2 - The secondary Type associated with this species.
- height - A scaled integer representing the height of the pokemon in meters. The last two digits are the decimal
           fraction component of the height, so the height can be considered as being scaled by a factor of 100.
- weight - A scaled integer representing the weight of the pokemon in kilograms. The last two digits are the decimal
           fraction component of the weight, so the weight can be considered as being scaled by a factor of 100.


```json
{
  id: number,
  name:
}
```

[
]

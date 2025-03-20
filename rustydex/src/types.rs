#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
pub enum SpecialType {
    #[default]
    Unknown, // '???' type, Only present in older generations
    Bird,    // Only present in gens 1 and 2 as an unused type value.
    Stellar, // Only present in gen 9 for the tera stellar type.
}

#[derive(Debug, Copy, Clone, Default, PartialEq, Hash)]
pub enum Type {
    Bug,
    Dark,
    Dragon,
    Electric,
    Fairy,
    Fighting,
    Fire,
    Flying,
    Ghost,
    Grass,
    Ground,
    Ice,

    #[default]
    Normal,

    Poison,
    Psychic,
    Rock,
    Steel,
    Water,
    None,
}

pub const TYPES: [Type; 19] = [
    Type::Bug,
    Type::Dark,
    Type::Dragon,
    Type::Electric,
    Type::Fairy,
    Type::Fighting,
    Type::Fire,
    Type::Flying,
    Type::Ghost,
    Type::Grass,
    Type::Ground,
    Type::Ice,
    Type::Normal,
    Type::Poison,
    Type::Psychic,
    Type::Rock,
    Type::Steel,
    Type::Water,
    Type::None,
];

// pub struct TypeInfo {
//     pub ptype: Type,
//     pub weaknesses: &'static [Type],
//     pub strengths: &'static [Type],
//     pub immunities: &'static [Type],
//     pub unhittables: &'static [Type],
// }

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DoubleEffectiveness {
    Immune,
    Neutral,
    Effective,
    Resisted,
    DoubleEffective,
    DoubleResisted,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Effectiveness {
    Immune,
    Neutral,
    Effective,
    Resisted,
}

impl Effectiveness {
    #[inline]
    const fn into_double_effectiveness(self) -> DoubleEffectiveness {
        match self {
            Effectiveness::Neutral => DoubleEffectiveness::Neutral,
            Effectiveness::Immune => DoubleEffectiveness::Immune,
            Effectiveness::Effective => DoubleEffectiveness::Effective,
            Effectiveness::Resisted => DoubleEffectiveness::Resisted,
        }
    }
}

impl From<Effectiveness> for DoubleEffectiveness {
    fn from(val: Effectiveness) -> Self {
        val.into_double_effectiveness()
    }
}

// Constants
const BUG_WEAKNESSES: [Type; 3] = [Type::Rock, Type::Fire, Type::Flying];
const DARK_WEAKNESSES: [Type; 3] = [Type::Bug, Type::Fairy, Type::Fighting];
const DRAGON_WEAKNESSES: [Type; 3] = [Type::Dragon, Type::Fairy, Type::Ice];
const ELECTRIC_WEAKNESSES: [Type; 1] = [Type::Ground];
const FAIRY_WEAKNESSES: [Type; 2] = [Type::Poison, Type::Steel];
const FIGHTING_WEAKNESSES: [Type; 3] = [Type::Fairy, Type::Flying, Type::Psychic];
const FIRE_WEAKNESSES: [Type; 3] = [Type::Ground, Type::Rock, Type::Water];
const FLYING_WEAKNESSES: [Type; 3] = [Type::Electric, Type::Ice, Type::Rock];
const GHOST_WEAKNESSES: [Type; 2] = [Type::Dark, Type::Ghost];
const GRASS_WEAKNESSES: [Type; 5] = [Type::Bug, Type::Fire, Type::Flying, Type::Ice, Type::Poison];
const GROUND_WEAKNESSES: [Type; 3] = [Type::Grass, Type::Ice, Type::Water];
const ICE_WEAKNESSES: [Type; 4] = [Type::Fighting, Type::Fire, Type::Rock, Type::Steel];
const NORMAL_WEAKNESSES: [Type; 1] = [Type::Fighting];
const POISON_WEAKNESSES: [Type; 2] = [Type::Ground, Type::Psychic];
const PSYCHIC_WEAKNESSES: [Type; 3] = [Type::Bug, Type::Dark, Type::Ghost];
const ROCK_WEAKNESSES: [Type; 5] = [Type::Fighting, Type::Grass, Type::Ground, Type::Steel, Type::Water];
const STEEL_WEAKNESSES: [Type; 3] = [Type::Fighting, Type::Fire, Type::Ground];
const WATER_WEAKNESSES: [Type; 2] = [Type::Electric, Type::Grass];
const NONE_WEAKNESSES: [Type; 0] = [];

impl Type {
    pub const fn double_effectiveness(attacker: &Self, defender_one: &Self, defender_two: &Self) -> DoubleEffectiveness {
        let eff_one = Self::single_effectiveness(attacker, defender_one);

        if (*defender_one as usize) == (*defender_two as usize) {
            return eff_one.into_double_effectiveness();
        }

        let eff_two = Self::single_effectiveness(attacker, defender_two);

        match (eff_one, eff_two) {
            // Immunity overrides everything
            (Effectiveness::Immune, _) | (_, Effectiveness::Immune) => DoubleEffectiveness::Immune,

            (Effectiveness::Neutral, Effectiveness::Neutral) => DoubleEffectiveness::Neutral,
            (Effectiveness::Neutral, Effectiveness::Effective) => DoubleEffectiveness::Effective,
            (Effectiveness::Neutral, Effectiveness::Resisted) => DoubleEffectiveness::Resisted,

            (Effectiveness::Effective, Effectiveness::Neutral) => DoubleEffectiveness::Effective,
            (Effectiveness::Effective, Effectiveness::Effective) => DoubleEffectiveness::DoubleEffective,
            (Effectiveness::Effective, Effectiveness::Resisted) => DoubleEffectiveness::Neutral,

            (Effectiveness::Resisted, Effectiveness::Neutral) => DoubleEffectiveness::Resisted,
            (Effectiveness::Resisted, Effectiveness::Effective) => DoubleEffectiveness::Neutral,
            (Effectiveness::Resisted, Effectiveness::Resisted) => DoubleEffectiveness::DoubleResisted,
        }
    }

    pub const fn single_effectiveness(attacker: &Self, defender: &Self) -> Effectiveness {
        match (attacker, defender) {
            // None Type
            (Self::None, _) => Effectiveness::Neutral,

            // Bug Matchups
            (Self::Bug, Self::Dark) => Effectiveness::Effective,
            (Self::Bug, Self::Fairy) => Effectiveness::Resisted,
            (Self::Bug, Self::Fighting) => Effectiveness::Resisted,
            (Self::Bug, Self::Fire) => Effectiveness::Resisted,
            (Self::Bug, Self::Flying) => Effectiveness::Resisted,
            (Self::Bug, Self::Ghost) => Effectiveness::Resisted,
            (Self::Bug, Self::Grass) => Effectiveness::Effective,
            (Self::Bug, Self::Poison) => Effectiveness::Resisted,
            (Self::Bug, Self::Psychic) => Effectiveness::Effective,
            (Self::Bug, Self::Steel) => Effectiveness::Resisted,
            (Self::Bug, _) => Effectiveness::Neutral,

            // Dark Matchups
            (Self::Dark, Self::Dark) => Effectiveness::Resisted,
            (Self::Dark, Self::Fairy) => Effectiveness::Resisted,
            (Self::Dark, Self::Fighting) => Effectiveness::Resisted,
            (Self::Dark, Self::Ghost) => Effectiveness::Effective,
            (Self::Dark, Self::Psychic) => Effectiveness::Effective,
            (Self::Dark, _) => Effectiveness::Neutral,

            // Dragon Matchups
            (Self::Dragon, Self::Dragon) => Effectiveness::Effective,
            (Self::Dragon, Self::Fairy) => Effectiveness::Immune,
            (Self::Dragon, Self::Steel) => Effectiveness::Resisted,
            (Self::Dragon, _) => Effectiveness::Neutral,

            // Electric Matchups
            (Self::Electric, Self::Dragon) => Effectiveness::Resisted,
            (Self::Electric, Self::Electric) => Effectiveness::Resisted,
            (Self::Electric, Self::Flying) => Effectiveness::Effective,
            (Self::Electric, Self::Grass) => Effectiveness::Resisted,
            (Self::Electric, Self::Ground) => Effectiveness::Immune,
            (Self::Electric, Self::Water) => Effectiveness::Effective,
            (Self::Electric, _) => Effectiveness::Neutral,

            // Fairy Matchups
            (Self::Fairy, Self::Dark) => Effectiveness::Effective,
            (Self::Fairy, Self::Dragon) => Effectiveness::Effective,
            (Self::Fairy, Self::Fighting) => Effectiveness::Effective,
            (Self::Fairy, Self::Fire) => Effectiveness::Resisted,
            (Self::Fairy, Self::Poison) => Effectiveness::Resisted,
            (Self::Fairy, Self::Steel) => Effectiveness::Resisted,
            (Self::Fairy, _) => Effectiveness::Neutral,

            // Fighting Matchups
            (Self::Fighting, Self::Bug) => Effectiveness::Resisted,
            (Self::Fighting, Self::Dark) => Effectiveness::Effective,
            (Self::Fighting, Self::Fairy) => Effectiveness::Resisted,
            (Self::Fighting, Self::Flying) => Effectiveness::Resisted,
            (Self::Fighting, Self::Ghost) => Effectiveness::Immune,
            (Self::Fighting, Self::Ice) => Effectiveness::Effective,
            (Self::Fighting, Self::Normal) => Effectiveness::Effective,
            (Self::Fighting, Self::Poison) => Effectiveness::Resisted,
            (Self::Fighting, Self::Psychic) => Effectiveness::Resisted,
            (Self::Fighting, Self::Rock) => Effectiveness::Effective,
            (Self::Fighting, Self::Steel) => Effectiveness::Effective,
            (Self::Fighting, _) => Effectiveness::Neutral,

            // Fire Matchups
            (Self::Fire, Self::Bug) => Effectiveness::Effective,
            (Self::Fire, Self::Dragon) => Effectiveness::Resisted,
            (Self::Fire, Self::Fire) => Effectiveness::Resisted,
            (Self::Fire, Self::Grass) => Effectiveness::Effective,
            (Self::Fire, Self::Ice) => Effectiveness::Effective,
            (Self::Fire, Self::Rock) => Effectiveness::Resisted,
            (Self::Fire, Self::Steel) => Effectiveness::Effective,
            (Self::Fire, Self::Water) => Effectiveness::Resisted,
            (Self::Fire, _) => Effectiveness::Neutral,

            // Flying Matchups
            (Self::Flying, Self::Bug) => Effectiveness::Effective,
            (Self::Flying, Self::Electric) => Effectiveness::Resisted,
            (Self::Flying, Self::Fighting) => Effectiveness::Effective,
            (Self::Flying, Self::Grass) => Effectiveness::Effective,
            (Self::Flying, Self::Rock) => Effectiveness::Resisted,
            (Self::Flying, Self::Steel) => Effectiveness::Resisted,
            (Self::Flying, _) => Effectiveness::Neutral,

            // Ghost Matchups
            (Self::Ghost, Self::Dark) => Effectiveness::Resisted,
            (Self::Ghost, Self::Ghost) => Effectiveness::Effective,
            (Self::Ghost, Self::Normal) => Effectiveness::Immune,
            (Self::Ghost, Self::Psychic) => Effectiveness::Effective,
            (Self::Ghost, _) => Effectiveness::Neutral,

            // Grass Matchups
            (Self::Grass, Self::Bug) => Effectiveness::Resisted,
            (Self::Grass, Self::Dragon) => Effectiveness::Resisted,
            (Self::Grass, Self::Fire) => Effectiveness::Resisted,
            (Self::Grass, Self::Flying) => Effectiveness::Resisted,
            (Self::Grass, Self::Grass) => Effectiveness::Resisted,
            (Self::Grass, Self::Ground) => Effectiveness::Effective,
            (Self::Grass, Self::Poison) => Effectiveness::Resisted,
            (Self::Grass, Self::Rock) => Effectiveness::Effective,
            (Self::Grass, Self::Steel) => Effectiveness::Resisted,
            (Self::Grass, Self::Water) => Effectiveness::Effective,
            (Self::Grass, _) => Effectiveness::Neutral,

            // Ground Matchups
            (Self::Ground, Self::Bug) => Effectiveness::Resisted,
            (Self::Ground, Self::Electric) => Effectiveness::Effective,
            (Self::Ground, Self::Fire) => Effectiveness::Effective,
            (Self::Ground, Self::Flying) => Effectiveness::Immune,
            (Self::Ground, Self::Grass) => Effectiveness::Resisted,
            (Self::Ground, Self::Poison) => Effectiveness::Effective,
            (Self::Ground, Self::Rock) => Effectiveness::Effective,
            (Self::Ground, Self::Steel) => Effectiveness::Effective,
            (Self::Ground, _) => Effectiveness::Neutral,

            // Ice Matchups
            (Self::Ice, Self::Dragon) => Effectiveness::Effective,
            (Self::Ice, Self::Fire) => Effectiveness::Resisted,
            (Self::Ice, Self::Flying) => Effectiveness::Effective,
            (Self::Ice, Self::Grass) => Effectiveness::Effective,
            (Self::Ice, Self::Ground) => Effectiveness::Effective,
            (Self::Ice, Self::Ice) => Effectiveness::Resisted,
            (Self::Ice, Self::Steel) => Effectiveness::Resisted,
            (Self::Ice, Self::Water) => Effectiveness::Resisted,
            (Self::Ice, _) => Effectiveness::Neutral,

            // Normal Matchups
            (Self::Normal, Self::Ghost) => Effectiveness::Immune,
            (Self::Normal, Self::Rock) => Effectiveness::Resisted,
            (Self::Normal, Self::Steel) => Effectiveness::Resisted,
            (Self::Normal, _) => Effectiveness::Neutral,

            // Poison Matchups
            (Self::Poison, Self::Fairy) => Effectiveness::Effective,
            (Self::Poison, Self::Ghost) => Effectiveness::Resisted,
            (Self::Poison, Self::Grass) => Effectiveness::Effective,
            (Self::Poison, Self::Ground) => Effectiveness::Resisted,
            (Self::Poison, Self::Poison) => Effectiveness::Resisted,
            (Self::Poison, Self::Rock) => Effectiveness::Resisted,
            (Self::Poison, Self::Steel) => Effectiveness::Immune,
            (Self::Poison, _) => Effectiveness::Neutral,

            // Psychic Matchups
            (Self::Psychic, Self::Fighting) => Effectiveness::Effective,
            (Self::Psychic, Self::Poison) => Effectiveness::Effective,
            (Self::Psychic, Self::Psychic) => Effectiveness::Resisted,
            (Self::Psychic, Self::Steel) => Effectiveness::Resisted,
            (Self::Psychic, _) => Effectiveness::Neutral,

            // Rock Matchups
            (Self::Rock, Self::Bug) => Effectiveness::Effective,
            (Self::Rock, Self::Fighting) => Effectiveness::Resisted,
            (Self::Rock, Self::Fire) => Effectiveness::Effective,
            (Self::Rock, Self::Flying) => Effectiveness::Effective,
            (Self::Rock, Self::Ground) => Effectiveness::Resisted,
            (Self::Rock, Self::Ice) => Effectiveness::Effective,
            (Self::Rock, Self::Steel) => Effectiveness::Resisted,
            (Self::Rock, _) => Effectiveness::Neutral,

            // Steel Matchups
            (Self::Steel, Self::Electric) => Effectiveness::Resisted,
            (Self::Steel, Self::Fire) => Effectiveness::Resisted,
            (Self::Steel, Self::Ice) => Effectiveness::Effective,
            (Self::Steel, Self::Rock) => Effectiveness::Effective,
            (Self::Steel, Self::Steel) => Effectiveness::Resisted,
            (Self::Steel, Self::Water) => Effectiveness::Resisted,
            (Self::Steel, _) => Effectiveness::Neutral,

            // Water Matchups
            (Self::Water, Self::Dragon) => Effectiveness::Resisted,
            (Self::Water, Self::Fire) => Effectiveness::Effective,
            (Self::Water, Self::Grass) => Effectiveness::Resisted,
            (Self::Water, Self::Ground) => Effectiveness::Effective,
            (Self::Water, Self::Rock) => Effectiveness::Effective,
            (Self::Water, Self::Water) => Effectiveness::Resisted,
            (Self::Water, _) => Effectiveness::Neutral,
        }
    }

    pub const fn weaknesses(&self) -> &[Self] {
        match self {
            Self::Bug => &BUG_WEAKNESSES,
            Self::Dark => &DARK_WEAKNESSES,
            Self::Dragon => &DRAGON_WEAKNESSES,
            Self::Electric => &ELECTRIC_WEAKNESSES,
            Self::Fairy => &FAIRY_WEAKNESSES,
            Self::Fighting => &FIGHTING_WEAKNESSES,
            Self::Fire => &FIRE_WEAKNESSES,
            Self::Flying => &FLYING_WEAKNESSES,
            Self::Ghost => &GHOST_WEAKNESSES,
            Self::Grass => &GRASS_WEAKNESSES,
            Self::Ground => &GROUND_WEAKNESSES,
            Self::Ice => &ICE_WEAKNESSES,
            Self::Normal => &NORMAL_WEAKNESSES,
            Self::Poison => &POISON_WEAKNESSES,
            Self::Psychic => &PSYCHIC_WEAKNESSES,
            Self::Rock => &ROCK_WEAKNESSES,
            Self::Steel => &STEEL_WEAKNESSES,
            Self::Water => &WATER_WEAKNESSES,
            Self::None => &NONE_WEAKNESSES,
        }
    }
}

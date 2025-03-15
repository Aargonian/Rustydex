use pokemon_lib::*;

fn main() {
    let mut total = 0u32;
    let mut neutral = 0u32;
    let mut immune = 0u32;
    let mut effective = 0u32;
    let mut resisted = 0u32;
    let mut double_effective = 0u32;
    let mut double_resisted = 0u32;

    for t1 in TYPES {
        for t2 in TYPES {
            for t3 in TYPES {
                total += 1;

                let effectiveness = Type::double_effectiveness(&t1, &t2, &t3);

                match effectiveness {
                    DoubleEffectiveness::Immune => immune += 1,
                    DoubleEffectiveness::Neutral => neutral += 1,
                    DoubleEffectiveness::Effective => effective += 1,
                    DoubleEffectiveness::Resisted => resisted += 1,
                    DoubleEffectiveness::DoubleEffective => double_effective += 1,
                    DoubleEffectiveness::DoubleResisted => double_resisted += 1,
                }
            }
        }
    }

    let mut percent: f64;

    println!("TOTALS: ");
    println!("TOTAL: {total}");
    percent = 100.0 * neutral as f64 / total as f64;
    println!("NEUTRAL: {neutral} ({percent:.2}%)");
    percent = 100.0 * immune as f64 / total as f64;
    println!("IMMUNE: {immune} ({percent:.2}%)");
    percent = 100.0 * effective as f64 / total as f64;
    println!("EFFECTIVE: {effective} ({percent:.2}%)");
    percent = 100.0 * resisted as f64 / total as f64;
    println!("RESISTED: {resisted} ({percent:.2}%)");
    percent = 100.0 * double_effective as f64 / total as f64;
    println!("DOUBLE_EFFECTIVE: {double_effective} ({percent:.2}%)");
    percent = 100.0 * double_resisted as f64 / total as f64;
    println!("DOUBLE_RESISTED: {double_resisted} ({percent:.2}%)");
}

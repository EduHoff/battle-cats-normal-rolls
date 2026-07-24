use crate::domain::banner::BannerData;

pub fn advance_seed(mut seed: u32) -> u32 {
    seed ^= seed << 13;
    seed ^= seed >> 17;
    seed ^= seed << 15;
    seed
}

#[derive(Debug, Clone)]
pub struct UnitRoll {
    pub unit_name: String,
    pub unit_seed: u32,
}

#[derive(Debug, Clone)]
pub struct Roll {
    pub rarity: usize,
    pub rarity_seed: u32,
    pub unit_if_distinct: UnitRoll,
    pub unit_if_dupe: Option<UnitRoll>,
}

pub fn get_rarity(seed: u32, rate_cum_sum: &[u32]) -> usize {
    let max_rate = *rate_cum_sum.last().unwrap_or(&10000);
    if max_rate == 0 {
        return 0;
    }

    let seed_mod = seed % max_rate;
    rate_cum_sum
        .iter()
        .position(|&sum| seed_mod < sum)
        .unwrap_or(0)
}

pub fn get_unit(seed: u32, units: &[String], removed_indices: &[usize]) -> (usize, String) {
    if units.is_empty() {
        return (0, String::new());
    }

    let num_units_in_pool = units.len().saturating_sub(removed_indices.len());
    if num_units_in_pool == 0 {
        return (0, units[0].clone());
    }

    let seed_mod = (seed as usize) % num_units_in_pool;

    let mut real_idx = seed_mod;
    let mut sorted_removed = removed_indices.to_vec();
    sorted_removed.sort_unstable();

    for &removed in &sorted_removed {
        if removed <= real_idx {
            real_idx += 1;
        }
    }

    (real_idx, units[real_idx].clone())
}

pub fn generate_rolls(mut seed: u32, num_rolls: usize, banner: &BannerData) -> Vec<Roll> {
    let mut rolls = Vec::with_capacity(num_rolls);

    for _ in 0..num_rolls {
        seed = advance_seed(seed);
        let rarity_seed = seed;
        let rarity = get_rarity(rarity_seed, &banner.rate_cum_sum);

        seed = advance_seed(seed);
        let unit_seed = seed;
        let pool = &banner.pools[rarity];
        let (unit_id, unit_name) = get_unit(unit_seed, &pool.units, &[]);

        let unit_if_distinct = UnitRoll {
            unit_name: unit_name.clone(),
            unit_seed,
        };

        let mut unit_if_dupe = None;
        if pool.reroll {
            let mut reroll_seed = unit_seed;
            let mut reroll_unit_name = unit_name.clone();
            let mut reroll_removed = vec![unit_id];

            while reroll_unit_name == unit_name {
                reroll_seed = advance_seed(reroll_seed);
                let (next_unit_id, next_unit_name) =
                    get_unit(reroll_seed, &pool.units, &reroll_removed);
                reroll_unit_name = next_unit_name;
                reroll_removed.push(next_unit_id);
            }

            unit_if_dupe = Some(UnitRoll {
                unit_name: reroll_unit_name,
                unit_seed: reroll_seed,
            });
        }

        rolls.push(Roll {
            rarity,
            rarity_seed,
            unit_if_distinct,
            unit_if_dupe,
        });
    }

    rolls
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::banner::BannerData;

    #[test]
    fn test_xorshift_seed_advancement() {
        let seed = 1;
        let next_seed = advance_seed(seed);

        assert_ne!(seed, next_seed);
        assert_eq!(next_seed, advance_seed(seed));
    }

    #[test]
    fn test_generate_rolls_normal_banner() {
        let banner = BannerData::normal_banner();
        let rolls = generate_rolls(12345, 3, &banner);

        assert_eq!(rolls.len(), 3);

        for (i, roll) in rolls.iter().enumerate() {
            println!(
                "Roll {}: Unit = {}, Rarity = {}",
                i + 1,
                roll.unit_if_distinct.unit_name,
                roll.rarity
            );
        }
    }
}

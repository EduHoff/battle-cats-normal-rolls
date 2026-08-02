use serde::Serialize;

use crate::domain::banner::BannerData;

pub const fn advance_seed(mut seed: u32) -> u32 {
    seed ^= seed << 13;
    seed ^= seed >> 17;
    seed ^= seed << 15;
    seed
}

#[derive(Debug, Clone, Serialize)]
pub struct UnitRoll {
    pub unit_name: String,
    pub unit_seed: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Roll {
    pub rarity: usize,
    pub rarity_seed: u32,
    pub unit_if_distinct: UnitRoll,
    pub unit_if_dupe: Option<UnitRoll>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CellData {
    pub name: String,
    pub seed: u32,
    pub rarity: usize,
    pub is_utility: bool,
    pub dupe_name: Option<String>,
    pub dupe_seed: Option<u32>,
    pub dupe_target_no: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Row {
    pub track_a_seed: u32,
    pub track_b_seed: u32,
    pub cell_a: CellData,
    pub cell_b: CellData,
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
        let fallback_unit = units
            .first()
            .expect("units vector must contain at least one unit")
            .clone();
        return (0, fallback_unit);
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

    let unit = units
        .get(real_idx)
        .expect("real_idx calculated must be within units bounds")
        .clone();

    (real_idx, unit)
}

pub fn generate_rolls(mut seed: u32, num_rolls: usize, banner: &BannerData) -> Vec<Roll> {
    let mut rolls = Vec::with_capacity(num_rolls);

    for _ in 0..num_rolls {
        seed = advance_seed(seed);
        let rarity_seed = seed;
        let rarity = get_rarity(rarity_seed, &banner.rate_cum_sum);

        seed = advance_seed(seed);
        let unit_seed = seed;
        let pool = banner
            .pools
            .get(rarity)
            .expect("rarity index must be valid for banner pools");

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

pub fn build_tracker_rows(initial_seed: u32, count: usize, banner: &BannerData) -> Vec<Row> {
    let track_a_rolls = generate_rolls(initial_seed, count, banner);

    let b_initial_seed = advance_seed(initial_seed);
    let track_b_rolls = generate_rolls(b_initial_seed, count, banner);

    let mut rows = Vec::with_capacity(count);

    for i in 0..count {
        let roll_a = track_a_rolls
            .get(i)
            .expect("i must be within track_a_rolls bounds");

        let roll_b = track_b_rolls
            .get(i)
            .expect("i must be within track_b_rolls bounds");

        let is_utility_a = BannerData::is_utility(&roll_a.unit_if_distinct.unit_name);
        let is_utility_b = BannerData::is_utility(&roll_b.unit_if_distinct.unit_name);

        let (dupe_name_a, dupe_seed_a, dupe_target_a) = if i > 0 {
            let curr_a = track_a_rolls
                .get(i)
                .expect("index i out of bounds for track_a_rolls");

            let prev_a = track_a_rolls
                .get(i - 1)
                .expect("index i - 1 out of bounds for track_a_rolls");

            if curr_a.unit_if_distinct.unit_name == prev_a.unit_if_distinct.unit_name
                && curr_a.unit_if_dupe.is_some()
            {
                let target_seed = if i + 1 < count {
                    track_b_rolls
                        .get(i + 1)
                        .expect("index i + 1 out of bounds for track_b_rolls")
                        .unit_if_distinct
                        .unit_seed
                } else {
                    roll_a.unit_if_distinct.unit_seed
                };

                let alt_name = roll_a.unit_if_dupe.as_ref().map_or_else(
                    || roll_a.unit_if_distinct.unit_name.clone(),
                    |d| d.unit_name.clone(),
                );

                (
                    Some(alt_name),
                    Some(target_seed),
                    Some(format!("{}B", i + 2)),
                )
            } else {
                (None, None, None)
            }
        } else {
            (None, None, None)
        };

        let (dupe_name_b, dupe_seed_b, dupe_target_b) = if i > 0 {
            let curr_b = track_b_rolls
                .get(i)
                .expect("index i out of bounds for track_b_rolls");

            let prev_b = track_b_rolls
                .get(i - 1)
                .expect("index i - 1 out of bounds for track_b_rolls");

            if curr_b.unit_if_distinct.unit_name == prev_b.unit_if_distinct.unit_name
                && curr_b.unit_if_dupe.is_some()
            {
                let target_seed = if i + 1 < count {
                    track_a_rolls
                        .get(i + 1)
                        .expect("index i + 1 out of bounds for track_a_rolls")
                        .unit_if_distinct
                        .unit_seed
                } else {
                    roll_b.unit_if_distinct.unit_seed
                };

                let alt_name = roll_b.unit_if_dupe.as_ref().map_or_else(
                    || roll_b.unit_if_distinct.unit_name.clone(),
                    |d| d.unit_name.clone(),
                );

                (
                    Some(alt_name),
                    Some(target_seed),
                    Some(format!("{}A", i + 3)),
                )
            } else {
                (None, None, None)
            }
        } else {
            (None, None, None)
        };

        rows.push(Row {
            track_a_seed: roll_a.unit_if_distinct.unit_seed,
            track_b_seed: roll_b.unit_if_distinct.unit_seed,
            cell_a: CellData {
                name: roll_a.unit_if_distinct.unit_name.clone(),
                seed: roll_a.unit_if_distinct.unit_seed,
                rarity: roll_a.rarity,
                is_utility: is_utility_a,
                dupe_name: dupe_name_a,
                dupe_seed: dupe_seed_a,
                dupe_target_no: dupe_target_a,
            },
            cell_b: CellData {
                name: roll_b.unit_if_distinct.unit_name.clone(),
                seed: roll_b.unit_if_distinct.unit_seed,
                rarity: roll_b.rarity,
                is_utility: is_utility_b,
                dupe_name: dupe_name_b,
                dupe_seed: dupe_seed_b,
                dupe_target_no: dupe_target_b,
            },
        });
    }

    rows
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

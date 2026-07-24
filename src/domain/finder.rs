use crate::domain::banner::BannerData;
use crate::domain::seed::{advance_seed, get_rarity, get_unit};
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct FinderResult {
    pub seed_before: u32,
    pub seed_after: u32,
    pub next_roll_is_reroll: bool,
}

pub fn find_seeds(user_rolls: &[String], banner: &BannerData) -> Vec<FinderResult> {
    if user_rolls.is_empty() {
        return Vec::new();
    }

    (1..=u32::MAX)
        .into_par_iter()
        .filter_map(|seed| {
            if check_seed_matches(seed, user_rolls, banner) {
                let (seed_after, next_roll_is_reroll) =
                    calculate_seed_after(seed, user_rolls.len(), banner);

                Some(FinderResult {
                    seed_before: seed,
                    seed_after,
                    next_roll_is_reroll,
                })
            } else {
                None
            }
        })
        .collect()
}

fn check_seed_matches(initial_seed: u32, user_rolls: &[String], banner: &BannerData) -> bool {
    let mut current_seed = initial_seed;
    let mut last_unit_name: Option<String> = None;

    for user_roll in user_rolls {
        current_seed = advance_seed(current_seed);
        let rarity = get_rarity(current_seed, &banner.rate_cum_sum);
        let pool = &banner.pools[rarity];

        current_seed = advance_seed(current_seed);
        let (unit_id, unit_name) = get_unit(current_seed, &pool.units, &[]);

        let rolled_unit = if pool.reroll && Some(&unit_name) == last_unit_name.as_ref() {
            current_seed = advance_seed(current_seed);
            let (_reroll_id, rerolled_name) = get_unit(current_seed, &pool.units, &[unit_id]);
            rerolled_name
        } else {
            unit_name
        };

        if &rolled_unit != user_roll {
            return false;
        }

        last_unit_name = Some(rolled_unit);
    }

    true
}

fn calculate_seed_after(initial_seed: u32, num_pulls: usize, banner: &BannerData) -> (u32, bool) {
    let mut current_seed = initial_seed;
    let mut last_unit_name: Option<String> = None;

    for _ in 0..num_pulls {
        current_seed = advance_seed(current_seed);
        let rarity = get_rarity(current_seed, &banner.rate_cum_sum);
        let pool = &banner.pools[rarity];

        current_seed = advance_seed(current_seed);
        let (unit_id, unit_name) = get_unit(current_seed, &pool.units, &[]);

        if pool.reroll && Some(&unit_name) == last_unit_name.as_ref() {
            current_seed = advance_seed(current_seed);
            let (_reroll_id, rerolled_name) = get_unit(current_seed, &pool.units, &[unit_id]);
            last_unit_name = Some(rerolled_name);
        } else {
            last_unit_name = Some(unit_name);
        }
    }

    let peek_seed = advance_seed(current_seed);
    let peek_rarity = get_rarity(peek_seed, &banner.rate_cum_sum);
    let peek_pool = &banner.pools[peek_rarity];

    let peek_unit_seed = advance_seed(peek_seed);
    let (_peek_id, peek_unit_name) = get_unit(peek_unit_seed, &peek_pool.units, &[]);

    let next_roll_is_reroll = peek_pool.reroll && Some(&peek_unit_name) == last_unit_name.as_ref();

    (current_seed, next_roll_is_reroll)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::banner::BannerData;
    use crate::domain::seed::generate_rolls;

    #[test]
    #[ignore]
    fn test_finder_roundtrip() {
        let banner = BannerData::normal_banner();
        let original_seed: u32 = 123_456_789;
        let num_rolls = 10;

        let generated_rolls = generate_rolls(original_seed, num_rolls, &banner);

        let mut user_rolls = Vec::new();
        let mut last_name: Option<String> = None;

        for roll in generated_rolls {
            let name = if Some(&roll.unit_if_distinct.unit_name) == last_name.as_ref() {
                roll.unit_if_dupe.unwrap().unit_name
            } else {
                roll.unit_if_distinct.unit_name
            };
            user_rolls.push(name.clone());
            last_name = Some(name);
        }

        let results = find_seeds(&user_rolls, &banner);

        assert!(
            !results.is_empty(),
            "Finder should have found at least one seed"
        );

        let found_original = results.iter().any(|res| res.seed_before == original_seed);
        assert!(found_original, "Original seed was not found in results!");
    }
}

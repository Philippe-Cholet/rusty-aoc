use itertools::Itertools;

use common::{prelude::*, Ok};
use crate::utils::OkIterator;

/// Allergen Assessment
pub fn solver(part: Part, input: &str) -> Result<String> {
    let data = input
        .lines()
        .map(|line| {
            let (left, right) = line
                .split_once(" (contains ")
                .context("No \" (contains\"")?;
            let ingredients = left.split_whitespace().collect_vec();
            let allergens = right
                .strip_suffix(')')
                .context("No closing bracket")?
                .split(", ")
                .collect_vec();
            Ok((ingredients, allergens))
        })
        .ok_collect_vec()?;
    let mut allergen2candidates = data
        .iter()
        .flat_map(|(ings, allergens)| allergens.iter().map(|a| (a, ings.clone())))
        .into_group_map() // {allergen: Vec<Vec<Ingredient>>, ...}
        .into_iter()
        .map(|(allergen, candidates)| {
            candidates
                .into_iter()
                .reduce(|mut intersection, ings| {
                    intersection.retain(|ing| ings.contains(ing));
                    intersection
                })
                .map(|intersection| (allergen, intersection))
                .context("No candidates for an allergen")
        })
        .ok_collect_hmap()?;
    #[cfg(debug_assertions)]
    for (allergen, candidates) in &allergen2candidates {
        println!("{allergen:<10}: {}", candidates.join(", "));
    }
    Ok(match part {
        Part1 => {
            let ingredients_with_possible_allergen: HashSet<_> =
                allergen2candidates.values().flatten().collect();
            data.iter()
                .flat_map(|(ings, _)| ings)
                .filter(|ing| !ingredients_with_possible_allergen.contains(ing))
                .count()
                .to_string()
        }
        Part2 => {
            let mut allergen2ing = vec![];
            while !allergen2candidates.is_empty() {
                let (allergen, ingredient) = allergen2candidates
                    .iter()
                    .find_map(|(a, candidates)| {
                        if let [ing] = candidates[..] {
                            Some((*a, ing))
                        } else {
                            None
                        }
                    })
                    .context("No allergen has a single candidate")?;
                allergen2ing.push((allergen, ingredient));
                allergen2candidates.remove(allergen);
                allergen2candidates
                    .values_mut()
                    .for_each(|ings| ings.retain(|ing| ing != &ingredient));
            }
            allergen2ing.sort();
            allergen2ing.into_iter().map(|(_, ing)| ing).join(",")
        }
    })
}

test_solver! {
    "\
mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)
" => ("5", "mxmxvkd,sqjhc,fvjkl"),
    include_input!(20 21) => ("1829", "mxkh,gkcqxs,bvh,sp,rgc,krjn,bpbdlmg,tdbcfb"),
}

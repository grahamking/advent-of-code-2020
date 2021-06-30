use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let s = read_to_string("input").unwrap();
    println!("Part 1: {}", part1(&s));
    println!("Part 2: {}", part2(&s));
}

fn part1(input: &str) -> usize {
    let menu = Menu::new(input);
    let active_i = menu.active_ingredients();
    menu.all_ingredients
        .iter()
        .filter(|x| !active_i.contains(x.as_str()))
        .count()
}

fn part2(input: &str) -> String {
    let menu = Menu::new(input);
    let mut allergens: Vec<&str> = menu.by_allergen.keys().map(String::as_str).collect();
    allergens.sort();
    allergens
        .iter()
        .map(|k| menu.by_allergen.get(*k).unwrap().as_str())
        .collect::<Vec<&str>>()
        .join(",")
}

#[derive(Debug)]
struct Menu {
    by_allergen: HashMap<String, String>,
    all_ingredients: Vec<String>,
}

impl Menu {
    fn new(input: &str) -> Self {
        let mut by_allergen: HashMap<String, HashSet<String>> = HashMap::new();
        let mut all_ingredients = Vec::new();

        for line in input.lines() {
            let e: Entry = line.into();
            all_ingredients.extend(e.ingredients.clone());
            for a_name in e.allergens {
                let a = by_allergen.entry(a_name).or_insert(e.ingredients.clone());
                *a = a
                    .intersection(&e.ingredients)
                    .map(String::to_string)
                    .collect();
            }
        }

        // by_allergen now contains a list of all the ingredients that _could_
        // contain the allergen. reduce the list.

        let mut baf = HashMap::new(); // by_allergen_final
        let mut singles = HashSet::new();
        let keys: Vec<String> = by_allergen.keys().map(String::to_string).collect();
        loop {
            let mut keep_going = false;
            for k in keys.iter() {
                let v = by_allergen.get(k).unwrap();
                if v.len() == 1 {
                    let s = v.iter().next().unwrap();
                    singles.insert(s.to_string());
                    baf.insert(k.to_string(), s.to_string());
                } else {
                    let v = by_allergen.entry(k.to_string()).or_default();
                    *v = v.difference(&singles).map(String::to_string).collect();
                    keep_going = true;
                }
            }
            if !keep_going {
                break;
            }
        }

        Menu {
            by_allergen: baf,
            all_ingredients,
        }
    }

    fn active_ingredients(&self) -> HashSet<&str> {
        let mut active_i = HashSet::new();
        for v in self.by_allergen.values() {
            active_i.insert(v.as_str());
        }
        active_i
    }
}

#[derive(Debug)]
struct Entry {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

impl From<&str> for Entry {
    fn from(s: &str) -> Self {
        let mut ingredients = HashSet::new();
        let mut allergens = HashSet::new();
        let mut in_allergens = false;
        for name in s.strip_suffix(")").unwrap().replace(",", "").split(" ") {
            if name == "(contains" {
                in_allergens = true;
                continue;
            }
            if in_allergens {
                allergens.insert(String::from(name));
            } else {
                ingredients.insert(String::from(name));
            }
        }
        Entry {
            ingredients,
            allergens,
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";

    #[test]
    fn test_part1() {
        assert_eq!(super::part1(INPUT), 5);
    }

    #[test]
    fn test_part2() {
        assert_eq!(super::part2(INPUT), "mxmxvkd,sqjhc,fvjkl");
    }
}

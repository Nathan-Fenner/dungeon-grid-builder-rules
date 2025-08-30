use std::collections::HashSet;

use rand::seq::IndexedRandom;

use crate::{
    color::{COLOR_DOOR, COLOR_EMPTY, COLOR_WILD},
    grid::Grid,
    pos::Pos,
    rule::{Rule, load_rules},
};

pub mod color;
pub mod grid;
pub mod pos;
pub mod rule;

fn is_rule_valid_at(grid: &Grid, rule: &Rule, shift: Pos) -> bool {
    for (p, color) in rule.before.cells.iter() {
        let g = p.shift(shift.x, shift.y);
        if !grid.cells.contains_key(&g) {
            return false;
        }
        if *color == COLOR_WILD {
            // The original color must be not EMPTY.
            if grid.cells[&g] == COLOR_EMPTY {
                return false;
            }
        } else {
            // The original color must match.
            if grid.cells[&g] != *color {
                return false;
            }
        }
    }
    for (p, _color_after) in rule.after.cells.iter() {
        if rule.before.cells.contains_key(p) {
            continue;
        }
        let g = p.shift(shift.x, shift.y);
        if !grid.cells.contains_key(&g) {
            // Must remain fully within the original grid.
            return false;
        }
        // If it wasn't in the "before" pattern, it must be empty.
        if grid.cells[&g] != COLOR_EMPTY {
            return false;
        }
    }

    true
}

/// All of the positions it is legal to apply this rule at.
fn valid_shifts(level: &Grid, rule: &Rule) -> Vec<Pos> {
    let seed_cells: Vec<Pos> = level
        .cells
        .iter()
        .filter(|p| *p.1 != COLOR_EMPTY)
        .map(|p| *p.0)
        .collect();

    let target_cells: Vec<Pos> = rule
        .before
        .cells
        .iter()
        .filter(|p| *p.1 == COLOR_WILD)
        .map(|p| *p.0)
        .collect();

    let mut shifts: HashSet<Pos> = HashSet::new();
    for p1 in seed_cells.iter() {
        for p2 in target_cells.iter() {
            let shift = Pos {
                x: p1.x - p2.x,
                y: p1.y - p2.y,
            };
            shifts.insert(shift);
        }
    }

    shifts.retain(|shift| is_rule_valid_at(level, rule, *shift));

    shifts.into_iter().collect()
}

fn apply_rule(level: &mut Grid, rule: &Rule, shift: Pos) {
    // Delete the before pattern (in case it's larger than the 'after')
    for (p, color) in rule.before.cells.iter() {
        if *color == COLOR_WILD {
            continue;
        }
        let g = p.shift(shift.x, shift.y);
        *level.cells.get_mut(&g).unwrap() = COLOR_EMPTY;
    }
    for (p, color) in rule.after.cells.iter() {
        if *color == COLOR_WILD {
            continue;
        }
        let g = p.shift(shift.x, shift.y);
        *level.cells.get_mut(&g).unwrap() = *color;
    }
}

fn generate_level(start: &str, rule_files: &[&str]) -> Grid {
    let mut level = Grid::load_from_image(start);

    let mut rng = rand::rng();

    for rule_file in rule_files.iter() {
        let rules: Vec<Rule> = load_rules(rule_file)
            .into_iter()
            .flat_map(|rule| {
                let mut rots = vec![rule.clone()];
                let rule = rule.rotated90();
                rots.push(rule.clone());
                let rule = rule.rotated90();
                rots.push(rule.clone());
                let rule = rule.rotated90();
                rots.push(rule);

                rots
            })
            .collect();

        for _ in 0..1000 {
            let rule = rules.choose(&mut rng).unwrap();

            let shifts = valid_shifts(&level, rule);
            let Some(shift) = shifts.choose(&mut rng) else {
                continue;
            };

            println!("applying rule");
            apply_rule(&mut level, rule, *shift);
        }
    }
    level
}

fn main() {
    for i in 0..5 {
        let level = generate_level("start.png", &["rules.png", "cleanup.png", "reshape.png"]);
        level.save_to_image(&format!("result{i}.png"));
    }
}

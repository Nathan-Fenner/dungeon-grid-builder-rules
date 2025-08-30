use std::collections::HashSet;

use rand::seq::IndexedRandom;

use crate::{
    color::{COLOR_EMPTY, COLOR_WILD},
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

fn main() {
    let mut level = Grid::load_from_image("start.png");
    let rules = load_rules("rules.png");

    let mut rng = rand::rng();

    let shifts = valid_shifts(&level, &rules[0]);
    println!("shifts: {:?}", shifts);

    let shift = shifts.choose(&mut rng).unwrap();
    apply_rule(&mut level, &rules[0], *shift);

    level.save_to_image("result.png");
}

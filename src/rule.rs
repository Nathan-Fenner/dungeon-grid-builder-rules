use std::collections::{HashMap, HashSet};

use crate::{
    color::{COLOR_RULE, COLOR_RULE_THEN, COLOR_WILD, Color},
    grid::Grid,
    pos::Pos,
};

/// A rule converts a grid into another.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Rule {
    pub before: Grid,
    pub after: Grid,
}

impl Rule {
    pub fn rotated90(&self) -> Rule {
        Self {
            before: Grid {
                cells: self
                    .before
                    .cells
                    .iter()
                    .map(|(p, v)| (p.rotated90(), *v))
                    .collect(),
            },
            after: Grid {
                cells: self
                    .after
                    .cells
                    .iter()
                    .map(|(p, v)| (p.rotated90(), *v))
                    .collect(),
            },
        }
    }
}

pub fn load_rules(src: &str) -> Vec<Rule> {
    let grid = Grid::load_from_image(src);

    // Find contiguous sections, treating COLOR_RULE as splitting.
    // Then find sections connected by COLOR_RULE. The side that is more-right is the output.
    // The COLOR_WILD must match up.

    let all_colors: HashSet<Color> = grid.cells.values().copied().collect();
    println!("all_colors = {all_colors:?}");

    let mut regions: Vec<Grid> = Vec::new();
    let mut visited: HashSet<Pos> = HashSet::new();

    for p in grid.cells.keys().copied() {
        if visited.contains(&p) {
            continue;
        }
        if grid.cells[&p] == COLOR_RULE || grid.cells[&p] == COLOR_RULE_THEN {
            // Rule lines are skipped.
            continue;
        }

        let mut stack = vec![p];
        let mut island: HashSet<Pos> = stack.iter().copied().collect();
        visited.insert(p);

        while let Some(current) = stack.pop() {
            for neighbor in current.neighbors4() {
                if visited.contains(&neighbor)
                    || !grid.cells.contains_key(&neighbor)
                    || grid.cells[&neighbor] == COLOR_RULE
                    || grid.cells[&neighbor] == COLOR_RULE_THEN
                {
                    continue;
                }
                visited.insert(neighbor);
                island.insert(neighbor);
                stack.push(neighbor);
            }
        }

        let new_grid = Grid {
            cells: island.into_iter().map(|p| (p, grid.cells[&p])).collect(),
        };
        regions.push(new_grid);
    }

    let cell_to_region_index: HashMap<Pos, usize> = regions
        .iter()
        .enumerate()
        .flat_map(|(r_index, r)| r.cells.keys().map(move |p| (*p, r_index)))
        .collect();

    let mut rules: Vec<Rule> = Vec::new();
    for (index1, island1) in regions.iter().enumerate() {
        let mut connected: HashSet<Pos> = HashSet::new();
        for p in island1.cells.keys() {
            for n in p.neighbors4() {
                if grid.cells.contains_key(&n) && grid.cells[&n] == COLOR_RULE {
                    connected.insert(n);
                }
            }
        }

        // Follow the connected cells to all possible neighbors.
        let mut stack: Vec<Pos> = connected.iter().copied().collect();
        let mut neighbors: HashSet<usize> = HashSet::new();
        while let Some(reach) = stack.pop() {
            for neighbor in reach.neighbors4() {
                if !grid.cells.contains_key(&neighbor) {
                    continue;
                }
                if cell_to_region_index.contains_key(&neighbor) {
                    neighbors.insert(cell_to_region_index[&neighbor]);
                    continue;
                }
                if connected.contains(&neighbor) {
                    continue;
                }
                if grid.cells[&neighbor] == COLOR_RULE || grid.cells[&neighbor] == COLOR_RULE_THEN {
                    connected.insert(neighbor);
                    stack.push(neighbor);
                }
            }
        }

        for neighbor_index in neighbors {
            if neighbor_index == index1 {
                continue;
            }
            println!("rule {} -> {}", index1, neighbor_index);

            fn normalize(items: &mut [Pos]) -> Pos {
                assert!(!items.is_empty());

                let min_x = items.iter().map(|p| p.x).min().unwrap();
                let min_y = items.iter().map(|p| p.y).min().unwrap();
                for p in items.iter_mut() {
                    *p = p.shift(-min_x, -min_y);
                }
                items.sort();
                Pos {
                    x: -min_x,
                    y: -min_y,
                }
            }

            // Attempt to align them.
            let mut wild1: Vec<Pos> = island1
                .cells
                .iter()
                .filter(|p| *p.1 == COLOR_WILD)
                .map(|p| p.0)
                .copied()
                .collect::<Vec<Pos>>();
            if wild1.is_empty() {
                println!(" -- skip / island1 has no wild");
            }
            let mut wild2: Vec<Pos> = regions[neighbor_index]
                .cells
                .iter()
                .filter(|p| *p.1 == COLOR_WILD)
                .map(|p| p.0)
                .copied()
                .collect::<Vec<Pos>>();
            if wild2.is_empty() {
                println!("-- skip / island2 has no wild");
            }

            let shift1 = normalize(&mut wild1);
            let shift2 = normalize(&mut wild2);
            if wild1 != wild2 {
                println!("-- skip / wilds are different");
            }

            rules.push(Rule {
                before: island1.shift(shift1),
                after: regions[neighbor_index].shift(shift2),
            });
        }
    }

    rules
}

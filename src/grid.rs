use std::collections::BTreeMap;

use crate::{
    color::{COLOR_OUT, Color},
    pos::Pos,
};

#[derive(Default, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Grid {
    pub cells: BTreeMap<Pos, Color>,
}

impl Grid {
    /// Loads a grid from the provided image.
    /// COLOR_OUT cells are skipped.
    pub fn load_from_image(src: &str) -> Self {
        let img = image::open(src).expect("can open");
        let img = img.as_rgb8().expect("is rgb8");
        let mut cells = BTreeMap::new();

        for x in 0..img.width() {
            for y in 0..img.height() {
                let c: Color = img[(x, y)];
                if c == COLOR_OUT {
                    continue;
                }
                cells.insert(
                    Pos {
                        x: x as i32,
                        y: y as i32,
                    },
                    c,
                );
            }
        }

        Self { cells }
    }

    /// Saves to an image with a border of COLOR_OUT.
    pub fn save_to_image(&self, src: &str) {
        assert!(!self.cells.is_empty());
        let low_x = self.cells.keys().map(|p| p.x).min().unwrap();
        let low_y = self.cells.keys().map(|p| p.y).min().unwrap();
        let high_x = self.cells.keys().map(|p| p.x).max().unwrap();
        let high_y = self.cells.keys().map(|p| p.y).max().unwrap();

        let mut img =
            image::RgbImage::new((high_x - low_x) as u32 + 3, (high_y - low_y) as u32 + 3);

        for p in img.pixels_mut() {
            *p = COLOR_OUT;
        }

        for (p, c) in self.cells.iter() {
            let img_p = ((p.x - low_x) as u32 + 1, (p.y - low_y) as u32 + 1);
            img[img_p] = *c;
        }

        img.save(src).expect("can save image");
    }

    pub fn shift(&self, delta: Pos) -> Grid {
        Grid {
            cells: self
                .cells
                .iter()
                .map(|(p, v)| (p.shift(delta.x, delta.y), *v))
                .collect(),
        }
    }
}

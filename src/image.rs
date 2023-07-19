use raylib::prelude::*;
use quadtree_rs::{Quadtree, area::AreaBuilder, point::Point};
use rand::Rng;

pub struct MyImage {
    data: Quadtree<u64, Color>,
    pub zoom: f32,
}

impl MyImage {
    pub fn new(depth: usize) -> Self {
        // This is test code
        Self {
            data: Quadtree::new(depth),
            zoom: 0.0,
        }
    }

    pub fn start(&mut self) {
        for x in 0..640 {
            for y in 0..480 {
                let region = AreaBuilder::default()
                    .anchor(Point {x, y})
                    .dimensions((1,1))
                    .build().unwrap();

                let random = rand::thread_rng().gen_range(1..4);

                if random == 1 {
                    self.data.insert(region, Color::RED);
                } else if random == 2 {
                    self.data.insert(region, Color::BLUE);
                } else if random == 3 {
                    self.data.insert(region, Color::GREEN);
                }
            }
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle) {
        for i in &self.data {
            let pos = Vector2::new(i.anchor().x() as f32, i.anchor().y() as f32) * self.zoom;
            let scale = Vector2::new(i.anchor().x() as f32, i.anchor().y() as f32) * self.zoom;
            d.draw_rectangle_v(pos, scale, i.value_ref());
        }
    }
}

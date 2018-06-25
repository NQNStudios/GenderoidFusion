extern crate pxl;

use pxl::*;

const WIDTH:  usize = 512;
const HEIGHT: usize = 512;


struct Genderoid {
}

impl Genderoid {
  fn index(&self, x: usize, y: usize) -> usize {
    x + y * WIDTH
  }

  pub fn step(&mut self) {
  }
}

impl Program for Genderoid {
  fn new() -> Genderoid {
    Genderoid {
    }
  }

  fn dimensions() -> (usize, usize) {
    (WIDTH, HEIGHT)
  }

  fn title(&self) -> &str {
    "Genderoid: Fusion"
  }

  fn tick(&mut self, events: &[Event]) {
    for event in events {
    }

    self.step();
  }

  fn render(&mut self, pixels: &mut [Pixel]) {
  }
}

fn main() {
  pxl::run::<Genderoid>();
}

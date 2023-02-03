use crate::xy;
use rand::{distributions::Bernoulli, prelude::Distribution};
use rayon::prelude::{IntoParallelRefMutIterator, ParallelIterator, IndexedParallelIterator};

pub fn get(preset: Presets) -> Box<dyn Preset> {
    match preset {
        Presets::Empty => Box::new(Empty),
        Presets::Invert => Box::new(Invert),
        Presets::Random => Box::new(Random),
        Presets::Grid => Box::new(Grid),
        Presets::Cross => Box::new(Cross),
        Presets::X => Box::new(X),
    }
}

pub enum Presets {
    Empty,
    Invert,
    Random,
    Grid,
    Cross,
    X
}

pub trait Preset {
    fn make(&self, buff: &mut Vec<bool>, w: u32, h: u32, or: bool);
}

struct Empty;
impl Preset for Empty {
    fn make(&self, buff: &mut Vec<bool>, _: u32, _: u32, _: bool) {
        buff.par_iter_mut().for_each(|c| *c = false);
    }
}

struct Invert;
impl Preset for Invert {
    fn make(&self, buff: &mut Vec<bool>, _: u32, _: u32, _: bool) {
        buff.par_iter_mut().for_each(|c| *c = !*c);
    }
}

struct Random;
impl Preset for Random {
    fn make(&self, buff: &mut Vec<bool>, _: u32, _: u32, or: bool) {
        let dist = Bernoulli::from_ratio(1, 6).unwrap();
        buff.par_iter_mut()
            .for_each(|c|
                *c = (*c && or) || dist.sample(&mut rand::thread_rng())
            );
    }
}

struct Grid;
impl Preset for Grid {
    fn make(&self, buff: &mut Vec<bool>, w: u32, _: u32, or: bool) {
        buff.par_iter_mut()
            .enumerate()
            .for_each(|(i, c)| {
                let (x, y) = xy![i as u32, w];
                *c = (*c && or) || (x % 3 != 0 && y % 3 != 0);
            });
    }
}

struct Cross;
impl Preset for Cross {
    fn make(&self, buff: &mut Vec<bool>, w: u32, h: u32, or: bool) {
        let w2 = w / 2;
        let h2 = h / 2;
        buff.par_iter_mut()
            .enumerate()
            .for_each(|(i, c)| {
                let (x, y) = xy![i as u32, w];
                *c = (*c && or) || x == w2 || y == h2;
            });
    }
}

struct X;
impl Preset for X {
    fn make(&self, buff: &mut Vec<bool>, w: u32, h: u32, or: bool) {
        let width = w;
        let height = h;
        let w = (width - height) / 2;
        buff.par_iter_mut()
            .enumerate()
            .for_each(|(i, c)| {
                let (x, y) = xy![i as u32, width];
                *c = (*c && or) || x - w == y || x - w == height - y;
            });
    }
}

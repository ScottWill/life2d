use crate::{ix, xy};
use line_drawing::Bresenham;
use rand::{distributions::Bernoulli, prelude::Distribution};
use rayon::prelude::*;
use super::rules::Rules;

const RAND_DENOM: u32 = 6;

pub struct Grid {
    pub rules: Rules,
    cell_ref: bool,
    cells: [Vec<bool>; 2],
    height: u32,
    width: u32,
    overlay: bool,
}

impl Grid {

    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let cells = vec![false; size];
        Self {
            rules: Rules::default(),
            cell_ref: false,
            cells: [cells.clone(), cells],
            height,
            width,
            overlay: false,
        }
    }

    pub fn clear(&mut self) {
        let cells = &mut self.cells[self.cell_ref as usize];
        cells.par_iter_mut().for_each(|c| *c = false);
    }

    pub fn draw_line(&mut self, start: (i32, i32), end: (i32, i32), state: bool, sym: bool) {
        Bresenham::new(start, end)
            .collect::<Vec<(i32,i32)>>()
            .into_iter()
            .for_each(|p| {
                self.set_state(p, state, sym);
            });
    }

    pub fn invert(&mut self) {
        let cells = &mut self.cells[self.cell_ref as usize];
        cells.par_iter_mut()
            .for_each(|c| *c = !*c);
    }
    
    pub fn randomize(&mut self) {
        let dist = Bernoulli::from_ratio(1, RAND_DENOM).unwrap();
        let cells = &mut self.cells[self.cell_ref as usize];
        cells.par_iter_mut()
            .for_each(|c|
                *c = (*c && self.overlay) || dist.sample(&mut rand::thread_rng())
            );
    }
    
    pub fn preset_grid(&mut self) {
        let cells = &mut self.cells[self.cell_ref as usize];
        cells.par_iter_mut()
            .enumerate()
            .for_each(|(i, c)| {
                let (x, y) = xy![i as u32, self.width];
                *c = (*c && self.overlay) || (x % 3 != 0 && y % 3 != 0);
            });
    }
    
    pub fn preset_cross(&mut self) {
        let w = self.width / 2;
        let h = self.height / 2;
        let cells = &mut self.cells[self.cell_ref as usize];
        cells.par_iter_mut()
            .enumerate()
            .for_each(|(i, c)| {
                let (x, y) = xy![i as u32, self.width];
                *c = (*c && self.overlay) || x == w || y == h;
            });
    }
    
    pub fn preset_eks(&mut self) {
        let width = self.width;
        let height = self.height;
        let w = (width - height) / 2;
        let cells = &mut self.cells[self.cell_ref as usize];
        cells.par_iter_mut()
            .enumerate()
            .for_each(|(i, c)| {
                let (x, y) = xy![i as u32, self.width];
                *c = (*c && self.overlay) || x - w == y || x - w == height - y;
            });
    }
    
    fn set_state(&mut self, pos: (i32, i32), state: bool, sym: bool) {
        let cells = &mut self.cells[self.cell_ref as usize];
        for i in mirror_pos(pos, self.height as i32, self.width as i32, sym) {
            if i < cells.len() {
                cells[i] = state;
            }
        }
    }

    pub fn toggle_overlay(&mut self) {
        self.overlay = !self.overlay;
    }

    pub fn step(&mut self) {
        
        self.cell_ref = !self.cell_ref;
        
        let (left, right) = self.cells.split_at_mut(1);
        let (old_cells, new_cells) = match self.cell_ref {
            true  => (&left[0], &mut right[0]),
            false => (&right[0], &mut left[0]),
        };
        
        old_cells
            .par_iter()
            .enumerate()
            .map(|(i, c)| {
                let (x, y) = xy![i as u32, self.width];
                self.rules.eval(*c, count(&old_cells, x, y, self.width, self.height))
            })
            .collect_into_vec(new_cells);
        
    }

    pub fn render(&self, dims: usize) -> Vec<u8> {
        let cells: &Vec<bool> = &self.cells[self.cell_ref as usize];
        let size = cells.len() * dims;
        let mut buf = Vec::with_capacity(size);
        (0..size)
            .into_par_iter()
            .map(|i| !cells[i / dims] as u8 * 255)
            .collect_into_vec(&mut buf);
        buf
    }

}

fn count(cells: &Vec<bool>, x: u32, y: u32, w: u32, h: u32) -> usize {
    let xn = (w + x - 1) % w;
    let xp = (x + 1) % w;
    let yn = (h + y - 1) % h;
    let yp = (y + 1) % h;

    cells[ix!(xn, yn, w)] as usize +
    cells[ix!(x, yn, w)] as usize +
    cells[ix!(xp, yn, w)] as usize +
    cells[ix!(xn, y, w)] as usize +
    cells[ix!(xp, y, w)] as usize +
    cells[ix!(xn, yp, w)] as usize +
    cells[ix!(x, yp, w)] as usize +
    cells[ix!(xp, yp, w)] as usize
}

fn mirror_pos((x, y): (i32, i32), h: i32, w: i32, sym: bool) -> Vec<usize> {
    if sym {
        vec![
            ix!(w - x, h - y, w),
            ix!(w - x, y, w),
            ix!(x, h - y, w),
            ix!(x, y, w),
        ]

    } else {
        vec![ix!(x, y, w)]
    }
}

#[macro_export]
macro_rules! ix {
    ( $x:expr, $y:expr, $w:expr ) => {
        ($y * $w + $x) as usize
    }
}

#[macro_export]
macro_rules! xy {
    ( $x:expr, $w:expr ) => {
        ($x % $w, $x / $w)
    }
}
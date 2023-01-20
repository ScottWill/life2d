use line_drawing::Bresenham;
use nannou::{image::{ImageBuffer, DynamicImage}, rand::{distributions::Bernoulli, prelude::Distribution, self}};
use rayon::prelude::{IntoParallelRefIterator, IndexedParallelIterator, ParallelIterator, IntoParallelRefMutIterator, IntoParallelIterator};
use super::rules::Rules;

const DIM: usize = 3;
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
        let width = self.width;
        let cells = &mut self.cells[self.cell_ref as usize];
        cells.par_iter_mut()
            .enumerate()
            .for_each(|(i, c)| {
                let x = i as u32 % width;
                let y = i as u32 / width;
                *c = (*c && self.overlay) || (x % 3 != 0 && y % 3 != 0);
            });
    }
    
    pub fn preset_cross(&mut self) {
        let width = self.width;
        let w = self.width / 2;
        let h = self.height / 2;
        let cells = &mut self.cells[self.cell_ref as usize];
        cells.par_iter_mut()
            .enumerate()
            .for_each(|(i, c)| {
                let x = i as u32 % width;
                let y = i as u32 / width;
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
                let x = i as u32 % width;
                let y = i as u32 / width;
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
                let (x, y) = (i as u32 % self.width, i as u32 / self.width);
                self.rules.eval(*c, count(&old_cells, x, y, self.width, self.height))
            })
            .collect_into_vec(new_cells);
        
    }

    pub fn render(&self) -> DynamicImage {
        let cells: &Vec<bool> = &self.cells[self.cell_ref as usize];
        let buf = (0..cells.len() * DIM)
            .into_par_iter()
            .map(|i| !cells[i / DIM] as u8 * 255)
            .collect();
        DynamicImage::ImageRgb8(ImageBuffer::from_raw(self.width, self.height, buf).unwrap())
    }

}

fn count(cells: &Vec<bool>, x: u32, y: u32, w: u32, h: u32) -> usize {
    let xn = (w + x - 1) % w;
    let xp = (x + 1) % w;
    let yn = (h + y - 1) % h;
    let yp = (y + 1) % h;

    cells[uix(xn, yn, w)] as usize +
    cells[uix(x, yn, w)] as usize +
    cells[uix(xp, yn, w)] as usize +
    cells[uix(xn, y, w)] as usize +
    cells[uix(xp, y, w)] as usize +
    cells[uix(xn, yp, w)] as usize +
    cells[uix(x, yp, w)] as usize +
    cells[uix(xp, yp, w)] as usize
}

fn mirror_pos(pos: (i32, i32), h: i32, w: i32, sym: bool) -> Vec<usize> {
    let (x, y) = pos;
    if sym {
        let hw = w / 2;
        let hh = h / 2;
        vec![
            iix(hw - (x - hw), hh - (y - hh), w),
            iix(hw - (x - hw), hh + (y - hh), w),
            iix(hw + (x - hw), hh - (y - hh), w),
            iix(hw + (x - hw), hh + (y - hh), w),
        ]

    } else {
        vec![iix(x, y, w)]
    }
}

#[inline(always)]
fn iix(x: i32, y: i32, w: i32) -> usize {
    (y * w + x) as usize
}

#[inline(always)]
fn uix(x: u32, y: u32, w: u32) -> usize {
    (y * w + x) as usize
}
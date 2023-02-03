use crate::{ix, xy, EventHandler};
use line_drawing::Bresenham;
use nannou::prelude::*;
use rayon::prelude::*;
use super::{rules::Rules, presets::{Presets, self}};

pub struct Grid {
    pub overlay: bool,
    pub rules: Rules,
    cell_ref: bool,
    cells: [Vec<bool>; 2],
    height: u32,
    width: u32,
}

impl Grid {

    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        let cells = vec![false; size];
        let mut grid = Self {
            cell_ref: false,
            cells: [cells.clone(), cells],
            overlay: false,
            rules: Rules::default(),
            height,
            width,
        };
        grid.preset(Presets::Random);
        grid
    }

    pub fn draw_line(&mut self, start: (i32, i32), end: (i32, i32), state: bool, sym: bool) {
        Bresenham::new(start, end)
            .collect::<Vec<(i32,i32)>>()
            .into_iter()
            .for_each(|p| {
                self.set_state(p, state, sym);
            });
    }

    fn preset(&mut self, preset: Presets) {
        let cells = &mut self.cells[self.cell_ref as usize];
        presets::get(preset).make(cells, self.width, self.overlay);
    }
    
    fn set_state(&mut self, pos: (i32, i32), state: bool, sym: bool) {
        let cells = &mut self.cells[self.cell_ref as usize];
        for i in mirror_pos(pos, self.height as i32, self.width as i32, sym) {
            if i < cells.len() {
                cells[i] = state;
            }
        }
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

impl EventHandler for Grid {
    fn handle_event(&mut self, app: &App, event: &WindowEvent) {
        match event {
            KeyPressed(key) => match key {
                Key::C      => self.preset(Presets::Cross),
                Key::G      => self.preset(Presets::Grid),
                Key::I      => self.preset(Presets::Invert),
                Key::O      => self.overlay = !self.overlay,
                Key::R      => self.preset(Presets::Random),
                Key::X      => self.preset(Presets::X),
                Key::Back   => self.preset(Presets::Empty),
                _           => self.rules.handle_event(app, event),
            },
            _ => (),
        };
    }
}

fn count(cells: &Vec<bool>, x: u32, y: u32, w: u32, h: u32) -> usize {
    let xn = (w + x - 1) % w;
    let xp = (x + 1) % w;
    let yn = (h + y - 1) % h;
    let yp = (y + 1) % h;

    cells[ix!(xn, yn, w)] as usize +
    cells[ix!( x, yn, w)] as usize +
    cells[ix!(xp, yn, w)] as usize +
    cells[ix!(xn,  y, w)] as usize +
    cells[ix!(xp,  y, w)] as usize +
    cells[ix!(xn, yp, w)] as usize +
    cells[ix!( x, yp, w)] as usize +
    cells[ix!(xp, yp, w)] as usize
}

fn mirror_pos((x, y): (i32, i32), h: i32, w: i32, sym: bool) -> Vec<usize> {
    match sym {
        true => vec![
            ix!(w - x, h - y, w),
            ix!(w - x, y, w),
            ix!(x, h - y, w),
            ix!(x, y, w),
        ],
        false => vec![
            ix!(x, y, w)
        ],
    }
}

#[macro_export]
macro_rules! ix {
    ( $x:expr, $y:expr, $w:expr ) => {
        (($y) * ($w) + ($x)) as usize
    }
}

#[macro_export]
macro_rules! xy {
    ( $x:expr, $w:expr ) => {
        (($x) % ($w), ($x) / ($w))
    }
}
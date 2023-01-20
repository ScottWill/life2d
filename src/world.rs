use crate::grid::grid::Grid;
use nannou::{prelude::*, wgpu::*, image::*};

const PAUSED: &'static str = "Paused";
const RUNNING: &'static str = "Running";

pub struct Model {
    grid: Grid,
    mouse_pos: Option<Vec2>,
    scale: f32,
    dims: (u32, u32),
    stepping: bool,
}

impl Model {

    pub fn new(width: u32, height: u32, scale: u32) -> Self {
        let dims = (width / scale, height / scale);
        let mut grid = Grid::new(dims.0, dims.1);
        grid.randomize();
        Model {
            grid,
            dims,
            mouse_pos: None,
            scale: scale as f32,
            stepping: true,
        }
    }

    pub fn handle_event(&mut self, app: &App, event: nannou::event::WindowEvent) {
        match event {
            KeyPressed(key) => match key {
                Key::C      => self.grid.preset_cross(),
                Key::G      => self.grid.preset_grid(),
                Key::I      => self.grid.invert(),
                Key::O      => self.grid.toggle_overlay(),
                Key::R      => self.grid.randomize(),
                Key::X      => self.grid.preset_eks(),
                Key::Back   => self.grid.clear(),
                Key::Comma  => self.grid.rules.prev_rule(),
                Key::Return => self.step_once(),
                Key::Period => self.grid.rules.next_rule(),
                Key::Slash  => self.grid.rules.reset_rules(),
                Key::Space  => self.toggle_stepping(),
                _ => ()
            },
            MouseMoved(pos1) => if let Some(pos0) = self.mouse_pos {
                let pos1 = ny(pos1);
                let offset = {
                    let rect = app.window_rect();
                    Vec2::new(rect.w() * 0.5, rect.h() * 0.5)
                };
                self.grid.draw_line(
                    v2t((pos0 + offset) / self.scale),
                    v2t((pos1 + offset) / self.scale),
                    true,
                    app.keys.down.contains(&Key::LShift)
                );
                self.mouse_pos = Some(pos1);
            },
            MousePressed(button) => match button {
                MouseButton::Left => self.mouse_pos = Some(ny(app.mouse.position())),
                _ => ()
            },
            MouseReleased(button) => match button {
                MouseButton::Left => self.mouse_pos = None,
                _ => ()
            },
            _ => (),
        };
    }

    pub fn step(&mut self) {
        if self.stepping {
            self.grid.step();
        }
    }

    pub fn step_once(&mut self) {
        self.stepping = false;
        self.grid.step();
    }

    pub fn title_meta(&self) -> (&str, &str) {
        let rule = self.grid.rules.name();
        let running = match self.stepping {
            true  => RUNNING,
            false => PAUSED,
        };
        (rule, running)
    }

    pub fn view(&self, app: &App, draw: &Draw) {
        let buf = ImageBuffer::from_raw(self.dims.0, self.dims.1, self.grid.render(3)).unwrap();
        let view = Texture::from_image(app, &DynamicImage::ImageRgb8(buf));
        let mut desc = SamplerDescriptor::default();
        desc.mag_filter = FilterMode::Nearest;
        draw.scale(self.scale)
            .sampler(desc)
            .texture(&view);
    }

    fn toggle_stepping(&mut self) {
        self.stepping = !self.stepping;
    }

}

#[inline(always)]
fn ny(v: Vec2) -> Vec2 {
    Vec2::new(v.x, -v.y)
}

#[inline(always)]
fn v2t(v: Vec2) -> (i32, i32) {
    (v.x as i32, v.y as i32)
}
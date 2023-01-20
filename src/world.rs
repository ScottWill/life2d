use crate::grid::grid::Grid;
use nannou::{prelude::*, wgpu::*, image::*};

const PAUSED: &'static str = "Paused";
const RUNNING: &'static str = "Running";

pub struct Model {
    pub debug: bool,
    grid: Grid,
    mouse_pos: Option<IVec2>,
    scale: u32,
    dims: (u32, u32),
    stepping: bool,
    offset: IVec2,
}

impl Model {

    pub fn new(width: u32, height: u32, scale: u32, debug: bool) -> Self {
        assert!(height % scale == 0, "`scale` must be a divisor of `height`");
        assert!(width % scale == 0, "`scale` must be a divisor of `width`");
        let dims = (width / scale, height / scale);
        let mut grid = Grid::new(dims.0, dims.1);
        grid.randomize();
        Model {
            offset: IVec2::new(width as i32, height as i32) / 2,
            mouse_pos: None,
            stepping: true,
            debug,
            dims,
            grid,
            scale,
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
            MouseMoved(end) => if let Some(start) = self.mouse_pos {
                if !app.keys.down.contains(&Key::LControl) {
                    let end = end.as_i32();
                    let sym = app.keys.down.contains(&Key::LShift);
                    self.draw_line(start, end, self.offset, sym);
                    self.mouse_pos = Some(end);
                }
            },
            MousePressed(button) => match button {
                MouseButton::Left => self.mouse_pos = Some(app.mouse.position().as_i32()),
                _ => ()
            },
            MouseReleased(button) => match button {
                MouseButton::Left => if let Some(start) = self.mouse_pos {
                    if app.keys.down.contains(&Key::LControl) {
                        let end = app.mouse.position().as_i32();
                        let sym = app.keys.down.contains(&Key::LShift);
                        self.draw_line(start, end, self.offset, sym);
                    }
                    self.mouse_pos = None;
                },
                _ => ()
            },
            _ => (),
        };
    }

    fn draw_line(&mut self, start: IVec2, end: IVec2, offset: IVec2, sym: bool) {
        self.grid.draw_line(
            v2t((ny(start) + offset) / self.scale as i32),
            v2t((ny(end) + offset) / self.scale as i32),
            true,
            sym
        );
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
        draw.scale(self.scale as f32)
            .sampler(desc)
            .texture(&view);
    }

    fn toggle_stepping(&mut self) {
        self.stepping = !self.stepping;
    }

}

#[inline(always)]
fn ny(v: IVec2) -> IVec2 {
    IVec2::new(v.x, -v.y)
}

#[inline(always)]
fn v2t(v: IVec2) -> (i32, i32) {
    (v.x, v.y)
}
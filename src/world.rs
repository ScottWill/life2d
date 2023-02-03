use crate::{grid::grid::Grid, Args};
use nannou::{prelude::*, wgpu::*, image::*};

const PAUSED: &'static str = "Paused";
const RUNNING: &'static str = "Running";

pub struct Model {
    pub debug: bool,
    dims: (u32, u32),
    grid: Grid,
    mouse_pos: Option<IVec2>,
    offset: IVec2,
    scale: u32,
    scale3: Vec3,
    speed: u64,
    stepping: bool,
}

impl Model {

    pub fn new(args: &Args) -> Self {
        let dims = (args.width / args.resolution, args.height / args.resolution);
        let mut grid = Grid::new(dims.0, dims.1);
        grid.randomize();
        Model {
            debug: args.debug,
            mouse_pos: None,
            offset: IVec2::new(args.width as i32, args.height as i32) / 2,
            scale: args.resolution,
            scale3: Vec3::new(args.width as f32 / dims.0 as f32, args.height as f32 / dims.1 as f32, 0.0),
            speed: args.speed as u64,
            stepping: true,
            dims,
            grid,
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
                Key::S      => self.snapshot(app.main_window().elapsed_frames()),
                Key::X      => self.grid.preset_eks(),
                Key::Back   => self.grid.clear(),
                Key::Comma  => self.grid.rules.prev_rule(),
                Key::Return => self.step_once(),
                Key::Period => self.grid.rules.next_rule(),
                Key::Slash  => self.grid.rules.reset_rules(),
                Key::Space  => self.stepping = !self.stepping,
                _ => ()
            },
            MouseMoved(end) => if let Some(start) = self.mouse_pos {
                // if !app.keys.down.contains(&Key::LControl) {
                    let end = end.as_i32();
                    let sym = app.keys.down.contains(&Key::LShift);
                    self.draw_line(start, end, self.offset, sym);
                    self.mouse_pos = Some(end);
                // }
            },
            MousePressed(button) => match button {
                MouseButton::Left => self.mouse_pos = Some(app.mouse.position().as_i32()),
                _ => ()
            },
            MouseReleased(button) => match button {
                MouseButton::Left => /*if let Some(start) = self.mouse_pos*/ {
                    // if app.keys.down.contains(&Key::LControl) {
                    //     let end = app.mouse.position().as_i32();
                    //     let sym = app.keys.down.contains(&Key::LShift);
                    //     self.draw_line(start, end, self.offset, sym);
                    // }
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

    pub fn step(&mut self, ticks: u64) {
        if self.stepping && ticks % self.speed == 0 {
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
        let view = Texture::from_image(app, &self.grid_img());
        let mut desc = SamplerDescriptor::default();
        desc.mag_filter = FilterMode::Nearest;
        draw.scale_axes(self.scale3)
            .sampler(desc)
            .texture(&view);
    }

    fn snapshot(&self, frame: u64) {
        let img = self.grid_img();
        img.save(format!("snapshots/{frame}.png")).unwrap();
    }

    fn grid_img(&self) -> DynamicImage {
        let buf = self.grid.render(3);
        let img = ImageBuffer::from_raw(self.dims.0, self.dims.1, buf).unwrap();
        DynamicImage::ImageRgb8(img)
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
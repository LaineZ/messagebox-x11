pub fn rgb_to_u32(red: usize, green: usize, blue: usize, alpha: usize) -> u32 {
    let r = red.clamp(0, 255);
    let g = green.clamp(0, 255);
    let b = blue.clamp(0, 255);
    let a = alpha.clamp(0, 255);
    ((a << 24) | (r << 16) | (g << 8) | b) as u32
}

use fontdue::{
    layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle},
    Font,
};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

pub struct Drawing<'a> {
    pub dt: DrawTarget,
    pub font: &'a Font,
    window_w: f32,
    window_h: f32,
    // mouse
    mouse_x: f32,
    mouse_y: f32,
}

#[derive(Clone, Copy)]
pub struct Location2 {
    pub x: f32,
    pub y: f32,
}

impl Location2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<Location4> for Location2 {
    fn from(location: Location4) -> Self {
        Self {
            x: location.x,
            y: location.y,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Location4 {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Location4 {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}
#[derive(Clone, Copy)]
pub struct Color {
    pub a: u8,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(a: u8, r: u8, g: u8, b: u8) -> Self {
        Self { a, r, g, b }
    }

    pub fn black()  -> Self {
        Self { a: 255, r: 0, g: 0, b: 0 }
    }
}

impl<'a> Drawing<'a> {
    pub fn new(w: i32, h: i32, font: &'a Font) -> Self {
        let mut dt = DrawTarget::new(w, h);
        dt.clear(SolidSource::from_unpremultiplied_argb(255, 255, 255, 255));
        Self {
            dt,
            font,
            window_w: w as f32,
            window_h: h as f32,
            mouse_x: 0.0,
            mouse_y: 0.0,
        }
    }

    pub fn process_mouse(&mut self, x: f32, y: f32) {
        self.mouse_x = x;
        self.mouse_y = y;
        //println!("m {}x{}", x, y);
    }

    pub fn check_click(
        &self,
        x1: f32,
        y1: f32,
        w1: f32,
        h1: f32,
        x2: f32,
        y2: f32,
        w2: f32,
        h2: f32,
    ) -> bool {
        return x1 < x2 + w2 && x2 < x1 + w1 && y1 < y2 + h2 && y2 < y1 + h1;
    }

    pub fn draw_square(&mut self, location: Location4, color: Color) {
        let mut pb = PathBuilder::new();
        pb.rect(location.x, location.y, location.w, location.h);
        let path = pb.finish();
        let _ = &self.dt.fill(
            &path,
            &Source::Solid(SolidSource::from_unpremultiplied_argb(
                color.a, color.r, color.g, color.b,
            )),
            &DrawOptions::new(),
        );
    }

    pub fn draw_button(&mut self, text: &str, location: Location4, color: Color) -> bool {
        if self.window_w > location.x + location.w
            && self.window_h > location.y + location.h / 8.0
            && 0.0 < location.y + location.h
        {
            self.draw_square(location, color);
            self.draw_text(
                text,
                Location4::new(location.x + 2.0, location.y + 2.0, location.w, location.h),
                Color::new(255, 0, 0, 0),
                14.0,
            );

            //self.draw_text(&format!("location.x: {} location.y: {} location.w: {} location.h: {} || mx: {} my: {}", location.x, location.y, location.w, location.h, self.mouse_x, self.mouse_y), location.x, location.y);
        }

        return self.check_click(
            location.x,
            location.y,
            location.w,
            location.h,
            self.mouse_x,
            self.mouse_y,
            1.,
            1.,
        );
    }

    pub fn draw_text(&mut self, text: &str, location: Location4, color: Color, size: f32) {
        let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
        layout.reset(&LayoutSettings {
            x: 0.0,
            y: 0.0,
            max_width: Some(location.w),
            max_height: Some(location.h),
            horizontal_align: fontdue::layout::HorizontalAlign::Left,
            ..LayoutSettings::default()
        });
        layout.append(&[self.font], &TextStyle::new(text, size, 0));

        for glyph in layout.glyphs() {
            if glyph.char_data.is_control() {
                continue;
            }
            let (metrics, coverage) = self.font.rasterize_config(glyph.key);

            let mut image_data = Vec::with_capacity(coverage.len());
            for cov in coverage.iter() {
                let pixel = rgb_to_u32(
                    color.r as usize,
                    color.g as usize,
                    color.b as usize,
                    *cov as usize,
                );
                image_data.push(pixel);
            }

            self.dt.draw_image_at(
                glyph.x + location.x,
                glyph.y + location.y,
                &raqote::Image {
                    width: metrics.width as i32,
                    height: metrics.height as i32,
                    data: &image_data,
                },
                &raqote::DrawOptions {
                    blend_mode: raqote::BlendMode::Darken,
                    alpha: 1.0,
                    antialias: raqote::AntialiasMode::Gray,
                },
            );
        }
    }
}

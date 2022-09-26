use drawing::{Color, Drawing, Location4};
use minifb::{Key, Window, WindowOptions};
pub mod drawing;

pub fn msgbox(title: &str, message: &str) {
    let font_data = include_bytes!("OpenSans-Regular.ttf") as &[u8];
    // Parse it into the font type.
    let settings = fontdue::FontSettings::default();

    let font = fontdue::Font::from_bytes(font_data, settings).unwrap();

    let (w, h) = (320, 150);
    let mut render = Drawing::new(w, h, &font);
    render.draw_text(message, Location4::new(10.0, 10.0, w as f32, h as f32), Color::black(), 12.0);
    let mut window = Window::new(title, w as usize, h as usize, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.limit_update_rate(Some(std::time::Duration::from_millis(33)));
    while window.is_open() && !window.is_key_down(Key::Escape) || !window.is_key_down(Key::Enter) {
        let (x, y) = window.get_mouse_pos(minifb::MouseMode::Clamp).unwrap();
        render.process_mouse(x, y);

        if render.draw_button(
            "OK",
            Location4::new(10.0, 110.0, 50.0, 24.0),
            Color::new(255, 200, 200, 200),
        ) && window.get_mouse_down(minifb::MouseButton::Left)
        {
            break;
        }

        window
            .update_with_buffer(
                render.dt.get_data(),
                render.dt.width() as usize,
                render.dt.height() as usize,
            )
            .unwrap();
    }
}

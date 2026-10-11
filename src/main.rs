use macroquad::prelude::*;

const WINDOW_TITLE: &str = "window title";
//static BACKGROUND_COLOR: Color = Color::new(0.01, 0.01, 0.01, 1.00);
const BACKGROUND_COLOR: Color = Color::from_rgba(200, 240, 255, 255);

fn window_configuration() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_owned(),
        window_width: 1200,
        window_height: 800,
        ..Default::default()
    }
}

fn render() {
    clear_background(BACKGROUND_COLOR);

    draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

    draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);
    draw_fps();
}

#[macroquad::main(window_configuration)]
async fn main() {
    loop {
        render();
        next_frame().await
    }
}

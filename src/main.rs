use comfy::*;

comfy_game!("My Game!", GameState);

// Define a global state object
pub struct GameState {
    pub x: f32,
    pub y: f32,
    pub font: FontHandle,
}

// Everything interesting happens here.
impl GameLoop for GameState {
    fn new(_c: &mut EngineState) -> Self {
        game_config_mut().bloom_enabled = true;
        Self {
            x: 2.0,
            y: 3.0,
            font: load_font_from_bytes(include_bytes!("../fonts/Orbitron-Bold.ttf")),
        }
    }

    fn update(&mut self, _c: &mut EngineContext) {
        clear_background(TEAL);

        let text_position = screen_to_world(Vec2 { x: screen_width() / 2.0, y: 40.0 });

        draw_text_pro_experimental(
            simple_styled_text(&format!("x: {} y: {}", self.x, self.y)),
            text_position,
            WHITE,
            TextAlign::Center,
            32.0,
            self.font,
        );

        draw_circle(Vec2 { x: self.x, y: self.y }, 1.0, WHITE, 2);

        if is_key_down(KeyCode::W) {
            self.y += 0.2;
        }
        if is_key_down(KeyCode::S) {
            self.y -= 0.2;
        }
        if is_key_down(KeyCode::A) {
            self.x -= 0.2;
        }
        if is_key_down(KeyCode::D) {
            self.x += 0.2;
        }
    }
}

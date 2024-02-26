use comfy::*;

comfy_game!("My Game!", GameState);

// Define a global state object
pub struct GameState {
    pub x: i32,
    pub y: i32,
}

// Everything interesting happens here.
impl GameLoop for GameState {
    fn new(_c: &mut EngineState) -> Self {
        game_config_mut().bloom_enabled = true;
        Self { x: 2, y: 3 }
    }

    fn update(&mut self, _c: &mut EngineContext) {
        clear_background(TEAL);

        let text_position = screen_to_world(Vec2 {
            x: screen_width() / 2.0,
            y: 40.0,
        });

        draw_text(
            &format!("x: {} y: {}", self.x, self.y),
            text_position,
            WHITE,
            TextAlign::Center,
        );

        draw_circle(
            Vec2 {
                x: self.x as f32 / 2.0,
                y: self.y as f32 / 2.0,
            },
            1.0,
            WHITE,
            2,
        );

        if is_key_down(KeyCode::W) {
            self.y += 1;
        }
        if is_key_down(KeyCode::S) {
            self.y -= 1;
        }
        if is_key_down(KeyCode::A) {
            self.x -= 1;
        }
        if is_key_down(KeyCode::D) {
            self.x += 1;
        }
    }
}

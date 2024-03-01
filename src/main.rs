use comfy::*;

const PAW_HEIGHT: f32 = 6.0;
const PAW_WIDTH: f32 = 4.0;
const PAW_SHAPE: Vec2 = Vec2 { x: PAW_WIDTH, y: PAW_HEIGHT };
const PAW_ACCELERATION: f32 = 0.1;
const PAW_FRICTION: f32 = -0.2;
const BALL_RADIUS: f32 = 1.0;
const BASE_BALL_SPEED: f32 = 0.1;

comfy_game!("Cat Ball Wow", GameState);

// Define a global state object
pub struct GameState {
    pub left_paw_position: Vec2,
    pub left_paw_velocity: Vec2,
    pub right_paw_position: Vec2,
    pub right_paw_velocity: Vec2,
    pub ball_position: Vec2,
    pub ball_movement_vec: Vec2,
    pub score: u32,
    pub best_score: u32,
    pub font: FontHandle,
}

// Everything interesting happens here.
impl GameLoop for GameState {
    fn new(_c: &mut EngineState) -> Self {
        game_config_mut().bloom_enabled = true;
        game_config_mut().resolution = ResolutionConfig::Logical(1000, 1000);
        game_config_mut().tonemapping_enabled = true;

        // Load assets
        let textures = [
            ("background", "background.png"),
            ("paw_left", "paw_left.png"),
            ("paw_right", "paw_right.png"),
            ("ball", "ball.png"),
        ];
        pub static ASSET_DIR: include_dir::Dir<'_> = include_dir::include_dir!("$CARGO_MANIFEST_DIR/assets");
        init_asset_source(&ASSET_DIR, |path| {
            format!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/{}"), path)
        });

        load_multiple_textures(textures.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect_vec());

        Self {
            left_paw_position: Vec2 { x: -PAW_WIDTH, y: 0.0 },
            left_paw_velocity: Vec2::ZERO,
            right_paw_position: Vec2 { x: PAW_WIDTH, y: 0.0 },
            right_paw_velocity: Vec2::ZERO,
            ball_position: Vec2 { x: 0.0, y: 0.0 },
            ball_movement_vec: Vec2 { x: BASE_BALL_SPEED, y: BASE_BALL_SPEED },
            score: 0,
            best_score: 0,
            font: load_font_from_bytes(include_bytes!("../fonts/Orbitron-Bold.ttf")),
        }
    }

    fn update(&mut self, _c: &mut EngineContext) {
        clear_background(DARKPURPLE);

        let screen_position = screen_to_world(Vec2 { x: screen_width(), y: screen_height() });
        let score_position = screen_to_world(Vec2 { x: screen_width() / 2.0, y: 40.0 });
        let best_score_position = screen_to_world(Vec2 { x: screen_width() / 2.0, y: 120.0 });
        let touch_position = screen_to_world(Vec2 { x: screen_width() / 2.0, y: 500.0 });
        let ball_speed = BASE_BALL_SPEED + BASE_BALL_SPEED * ((self.score + 1) as f32 / 40.0);

        draw_sprite(texture_id("background"), Vec2::ZERO, WHITE, 1, screen_position * 2.0);

        draw_text_pro_experimental(
            simple_styled_text(&format!("SCORE: {}", self.score)),
            score_position,
            BLACK,
            TextAlign::Center,
            60.0,
            self.font,
            50,
        );
        draw_text_pro_experimental(
            simple_styled_text(&format!("BEST SCORE: {}", self.best_score)),
            best_score_position,
            BLACK,
            TextAlign::Center,
            60.0,
            self.font,
            50,
        );

        let touch_locations = get_touch_locations();

        draw_text_pro_experimental(
            simple_styled_text(&format!("Touch: {:#?}", touch_locations)),
            touch_position,
            BLACK,
            TextAlign::Center,
            30.0,
            self.font,
            50,
        );

        // Ball
        if self.ball_position.x > (screen_position.x - BALL_RADIUS) {
            self.ball_movement_vec.x = -ball_speed;
            self.score += 1;
        }
        if self.ball_position.x < -(screen_position.x) + BALL_RADIUS {
            self.ball_movement_vec.x = ball_speed;
            self.score += 1;
        }
        if self.ball_position.y > -(screen_position.y) - BALL_RADIUS {
            self.ball_movement_vec.y = -ball_speed;
            self.score += 1;
        }
        // Ball - Check for collision with paws
        if rect_contains(self.right_paw_position, PAW_SHAPE, self.ball_position) {
            self.ball_movement_vec.y = ball_speed;
            self.score += 1;
        }
        if rect_contains(self.left_paw_position, PAW_SHAPE, self.ball_position) {
            self.ball_movement_vec.y = ball_speed;
            self.score += 1;
        }
        // Ball - Check end-game
        if self.ball_position.y < screen_position.y - BALL_RADIUS {
            self.ball_position = Vec2::ZERO;
            self.score = 0;
            self.ball_movement_vec.x = BASE_BALL_SPEED;
            self.ball_movement_vec.y = BASE_BALL_SPEED;
        }
        self.ball_position += self.ball_movement_vec;
        if self.score > self.best_score {
            self.best_score = self.score
        }
        draw_sprite(texture_id("ball"), self.ball_position, WHITE, 6, Vec2::ONE * 2.5);

        // Paws
        let mut right_paw_acceleration = Vec2::ZERO;
        let mut left_paw_acceleration = Vec2::ZERO;
        if is_key_down(KeyCode::J) {
            right_paw_acceleration.x = -PAW_ACCELERATION;
        }
        if is_key_down(KeyCode::L) {
            right_paw_acceleration.x = PAW_ACCELERATION;
        }
        if is_key_down(KeyCode::A) {
            left_paw_acceleration.x = -PAW_ACCELERATION;
        }
        if is_key_down(KeyCode::D) {
            left_paw_acceleration.x = PAW_ACCELERATION;
        }

        for touch_location in touch_locations {
            let world_touch_location = screen_to_world(touch_location);
            if world_touch_location.x > 0.0 {
                if world_touch_location.x > self.right_paw_position.x {
                    right_paw_acceleration.x = PAW_ACCELERATION;
                }
                if world_touch_location.x < self.right_paw_position.x {
                    right_paw_acceleration.x = -PAW_ACCELERATION;
                }
            } else {
                if world_touch_location.x > self.left_paw_position.x {
                    left_paw_acceleration.x = PAW_ACCELERATION;
                }
                if world_touch_location.x < self.left_paw_position.x {
                    left_paw_acceleration.x = -PAW_ACCELERATION;
                }
            }
        }

        right_paw_acceleration.x += self.right_paw_velocity.x * PAW_FRICTION;
        left_paw_acceleration.x += self.left_paw_velocity.x * PAW_FRICTION;
        self.right_paw_velocity += right_paw_acceleration;
        self.left_paw_velocity += left_paw_acceleration;
        self.right_paw_position += self.right_paw_velocity + 0.5 * right_paw_acceleration;
        self.left_paw_position += self.left_paw_velocity + 0.5 * left_paw_acceleration;

        self.right_paw_position.x =
            self.right_paw_position.x.clamp(PAW_WIDTH / 2.0, screen_position.x - (PAW_WIDTH / 2.0));
        self.left_paw_position.x =
            self.left_paw_position.x.clamp(-screen_position.x + (PAW_WIDTH / 2.0), -(PAW_WIDTH / 2.0));

        self.right_paw_position.y = screen_position.y + (PAW_HEIGHT / 2.0);
        self.left_paw_position.y = screen_position.y + (PAW_HEIGHT / 2.0);

        draw_sprite(texture_id("paw_right"), self.right_paw_position, WHITE, 5, PAW_SHAPE);
        draw_sprite(texture_id("paw_left"), self.left_paw_position, WHITE, 5, PAW_SHAPE);
    }
}

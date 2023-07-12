use macroquad::prelude::{*, animation::{AnimatedSprite, Animation}};

#[no_mangle]
extern "C" {
    fn go_to_location();
}
pub enum LookAt {
    Up,
    Right,
    Left,
    Down
}

pub struct Player {
    position: Vec2,
    look_at: LookAt,
    animations: AnimatedSprite,
}

const MOVEMENT_KEYS: [KeyCode; 8] = [
    KeyCode::W,
    KeyCode::Up,

    KeyCode::S,
    KeyCode::Down,

    KeyCode::D,
    KeyCode::Right,
    
    KeyCode::A,
    KeyCode::Left,
];

impl Player {
    pub fn is_moving(&self) -> bool {
        let mut moving = false;
        for key in MOVEMENT_KEYS {
            if is_key_down(key){
                moving = true
            }
        } 
        moving
    }
}

#[macroquad::main("Experiment")]
async fn main() {
    let mut player = Player {
        look_at: LookAt::Down,
        position: vec2(screen_width() / 2.0, screen_height() / 2.0),
        animations: AnimatedSprite::new(32, 32, &[
            Animation {
                name: "idle-down".to_owned(),
                row: 0,
                frames: 1,
                fps: 12
            },
            Animation {
                name: "walking-down".to_owned(),
                row: 0,
                frames: 4,
                fps: 8
            },

            Animation {
                name: "idle-left".to_owned(),
                row: 0,
                frames: 1,
                fps: 12
            },
            Animation {
                name: "walking-left".to_owned(),
                row: 0,
                frames: 4,
                fps: 8
            },

            Animation {
                name: "idle-right".to_owned(),
                row: 0,
                frames: 1,
                fps: 12
            },
            Animation {
                name: "walking-right".to_owned(),
                row: 0,
                frames: 4,
                fps: 8
            },

            Animation {
                name: "idle-up".to_owned(),
                row: 0,
                frames: 1,
                fps: 12
            },
            Animation {
                name: "walking-up".to_owned(),
                row: 0,
                frames: 4,
                fps: 8
            }
        ], 
        true
    )
    };

    let player_spritesheets = vec![
        Image::from_file_with_format(include_bytes!("../../assets/spr_player_down.png"), Some(ImageFormat::Png)),
        Image::from_file_with_format(include_bytes!("../../assets/spr_player_down_walk.png"), Some(ImageFormat::Png)),
        Image::from_file_with_format(include_bytes!("../../assets/spr_player_left.png"), Some(ImageFormat::Png)),
        Image::from_file_with_format(include_bytes!("../../assets/spr_player_left_walk.png"), Some(ImageFormat::Png)),
        Image::from_file_with_format(include_bytes!("../../assets/spr_player_right.png"), Some(ImageFormat::Png)),
        Image::from_file_with_format(include_bytes!("../../assets/spr_player_right_walk.png"), Some(ImageFormat::Png)),
        Image::from_file_with_format(include_bytes!("../../assets/spr_player_up.png"), Some(ImageFormat::Png)),
        Image::from_file_with_format(include_bytes!("../../assets/spr_player_up_walk.png"), Some(ImageFormat::Png)),
        ];

    let plate_sprite = Texture2D::from_file_with_format(include_bytes!("../../assets/spr_plate.png"), Some(ImageFormat::Png));
    plate_sprite.set_filter(FilterMode::Nearest);

    let light_sprite = Texture2D::from_file_with_format(include_bytes!("../../assets/spr_light.png"), Some(ImageFormat::Png));
    light_sprite.set_filter(FilterMode::Nearest);


    loop {
        // Logical phase
        let sprite = Texture2D::from_image(&player_spritesheets[player.animations.current_animation()]);
        sprite.set_filter(FilterMode::Nearest);

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            player.position.x -= 8.0;
            player.look_at = LookAt::Left;
        } else if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            player.position.x += 8.0;
            player.look_at = LookAt::Right
        }

        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            player.position.y -= 8.0;
            player.look_at = LookAt::Up
        } else if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            player.position.y += 8.0;
            player.look_at = LookAt::Down
        }

        if player.is_moving() {
            match player.look_at {
                LookAt::Up => player.animations.set_animation(7),
                LookAt::Down => player.animations.set_animation(1),
                LookAt::Left => player.animations.set_animation(3),
                LookAt::Right => player.animations.set_animation(5)
            }
        } else {
            match player.look_at {
                LookAt::Up => player.animations.set_animation(6),
                LookAt::Down => player.animations.set_animation(0),
                LookAt::Left => player.animations.set_animation(2),
                LookAt::Right => player.animations.set_animation(4)
            }
        }

        let plate_rect = Rect::new(screen_width() / 2.0 - 48.0, screen_height() / 2.0 - 48.0, 96.0, 64.0);
        let player_rect = Rect::new(player.position.x, player.position.y, 96.0, 96.0);

        if plate_rect.overlaps(&player_rect) && is_key_pressed(KeyCode::Z) {
            unsafe {go_to_location()};
        }

        // Drawing phase
        clear_background(BLACK);
        
        if player.position.y < screen_height() / 2.0 - plate_sprite.height() - 40.0 {

        draw_texture_ex(sprite, 
            player.position.x, 
            player.position.y, 
            WHITE,
            DrawTextureParams { 
                source: Some(player.animations.frame().source_rect), 
                dest_size: vec2(96.0, 96.0).into(), 
                pivot: (player.animations.frame().dest_size / 2.0).into(),
                ..Default::default() }
        );

        draw_texture_ex(plate_sprite, 
                screen_width() / 2.0 - 48.0,
                screen_height() / 2.0 - 48.0, 
                WHITE,
                DrawTextureParams { 
                    dest_size: vec2(96.0, 96.0).into(), 
                    ..Default::default() }
        );

       } else {
            draw_texture_ex(plate_sprite, 
                screen_width() / 2.0 - 48.0,
                screen_height() / 2.0 - 48.0, 
                WHITE,
                DrawTextureParams { 
                    dest_size: vec2(96.0, 96.0).into(), 
                    ..Default::default() }
            );

            draw_texture_ex(sprite, 
                player.position.x, 
                player.position.y, 
                WHITE,
                DrawTextureParams { 
                    source: Some(player.animations.frame().source_rect), 
                    dest_size: vec2(96.0, 96.0).into(), 
                    pivot: (player.animations.frame().dest_size / 2.0).into(),
                    ..Default::default() }
            );
            
       }

        draw_texture_ex(light_sprite, 
        screen_width() / 2.0 - 48.0,
        screen_height() / 2.0 - 384.0 + 64.0, 
        WHITE,
        DrawTextureParams { 
            dest_size: vec2(96.0, 384.0).into(), 
            
            ..Default::default() }
        );

        if plate_rect.overlaps(&player_rect) {
            draw_text("Press Z to interact", screen_width() / 2.0 - 145.0, screen_height() - 32.0, 32.0, WHITE);
        }

        #[cfg(debug_assertions)]{
            draw_text("Player Position", 10.0, 20.0, 32.0, WHITE);
            draw_text(&format!("X: {}", player.position.x), 10.0, 52.0, 32.0, WHITE);
            draw_text(&format!("Y: {}", player.position.y), 10.0, 84.0, 32.0, WHITE);
            
            let mut color_red = RED.clone();
            color_red.a = 0.5;
            draw_rectangle(plate_rect.x, plate_rect.y, plate_rect.w, plate_rect.h, color_red);
            draw_rectangle(player_rect.x, player_rect.y, player_rect.w, player_rect.h, color_red);

        }
        player.animations.update();
        next_frame().await
    }
}
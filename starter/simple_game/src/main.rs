use core::time;
use crossbeam_channel::unbounded;
use my_game_engine::ffi::*;
use my_game_engine::*;
use serde::Deserialize;
use std::error::Error;
use std::thread;
use std::time::Duration;

#[derive(Deserialize)]
struct SpriteData {
    pub width: i32,
    pub height: i32,
    pub r: i32,
    pub g: i32,
    pub b: i32,
    pub x: f32,
    pub y: f32,
}

enum Command {
    GetSprite,
    Quit,
}

fn check_overlap(sprite1: *mut Sprite, sprite2: *mut Sprite) -> bool {
    unsafe {
        let s1 = &*sprite1;
        let s2 = &*sprite2;

        let s1_left = s1.x;
        let s1_right = s1.x + s1.width as f32;
        let s1_top = s1.y;
        let s1_bottom = s1.y + s1.height as f32;

        let s2_left = s2.x;
        let s2_right = s2.x + s2.width as f32;
        let s2_top = s2.y;
        let s2_bottom = s2.y + s2.height as f32;

        !(s1_left >= s2_right || s1_right <= s2_left || s1_top >= s2_bottom || s1_bottom <= s2_top)
    }
}

fn networking_loop(
    cmd_receiver: crossbeam_channel::Receiver<Command>,
    sprite_sender: crossbeam_channel::Sender<SpriteData>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    loop {
        let cmd = cmd_receiver.recv()?;
        match cmd {
            Command::GetSprite => {
                // do nothing, just continue with the request
            }
            Command::Quit => {
                break;
            }
        }

        let response = reqwest::blocking::get(
            "https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler",
        )?;
        let resp_text = response.text()?;

        let spr_data: SpriteData = serde_json::from_str(&resp_text)?;
        sprite_sender.send(spr_data)?;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut player_x = 390.0;
    let mut player_y = 290.0;
    let player = rust_create_sprite(player_x, player_y, 20, 20, 255, 255, 255);
    let mut cur_sprite: *mut Sprite = std::ptr::null_mut();
    const SPRITE_GOAL: i32 = 5;
    let mut game_started = false;
    let mut game_finished = false;
    let mut request_sprite = true;
    let mut num_sprites_caught = 0;
    let mut start_time: std::time::Instant = std::time::Instant::now();
    let mut duration = Duration::new(0, 0);

    let (sprite_sender, sprite_receiver) = unbounded::<SpriteData>();
    let (cmd_sender, cmd_receiver) = unbounded::<Command>();
    let networkingthread = thread::spawn(move || networking_loop(cmd_receiver, sprite_sender));

    start_window_and_game_loop!(
        "Simple Game",
        20,
        {},
        {
            rust_clear_screen();
            if !game_finished {
                rust_render_sprite(player);
                if !cur_sprite.is_null() {
                    rust_render_sprite(cur_sprite);
                }

                if !game_started {
                    rust_write_text(&format!("CATCH {} SPRITES!", SPRITE_GOAL), 340.0);
                    rust_write_text("PRESS SPACE TO START!", 380.0);
                }

                if !cur_sprite.is_null() && check_overlap(player, cur_sprite) {
                    println!("Player overlapped with sprite!");
                    cur_sprite = std::ptr::null_mut();
                    num_sprites_caught += 1;
                    println!("Sprites caught: {}", num_sprites_caught);
                    if num_sprites_caught < SPRITE_GOAL {
                        request_sprite = true;
                    } else {
                        duration = std::time::Instant::now().duration_since(start_time);
                        game_finished = true;
                    }
                }

                on_key_press!(GLFW_KEY_LEFT,
                    player_x -= 4.0;
                    if player_x < 0.0 {
                        player_x = 0.0;
                    }
                    move_sprite!(player, false, player_x, player_y);
                );
                on_key_press!(GLFW_KEY_UP,
                    player_y -= 4.0;
                    if player_y < 0.0 {
                        player_y = 0.0;
                    }
                    move_sprite!(player, false, player_x, player_y);
                );
                on_key_press!(GLFW_KEY_RIGHT,
                    player_x += 4.0;
                    if player_x > 780.0 {
                        player_x = 780.0;
                    }
                    move_sprite!(player, false, player_x, player_y);
                );
                on_key_press!(GLFW_KEY_DOWN,
                    player_y += 4.0;
                    if player_y > 580.0 {
                        player_y = 580.0;
                    }
                    move_sprite!(player, false, player_x, player_y);
                );
                on_key_press!(GLFW_KEY_SPACE,
                    game_started = true;
                    start_time = std::time::Instant::now();
                );

                if game_started && request_sprite {
                    request_sprite = false;
                    cmd_sender.send(Command::GetSprite)?;
                    println!("Requested new sprite data from server");
                }

                if let Ok(sprite_data) = sprite_receiver.try_recv() {
                    cur_sprite = rust_create_sprite(
                        sprite_data.x,
                        sprite_data.y,
                        sprite_data.width,
                        sprite_data.height,
                        sprite_data.r,
                        sprite_data.g,
                        sprite_data.b,
                    );
                    rust_render_sprite(cur_sprite);
                }
            } else {
                rust_write_text(
                    &format!(
                        "WIN! CAUGHT {} SPRITES IN {} SEC!",
                        num_sprites_caught,
                        duration.as_secs()
                    ),
                    300.0,
                );
                rust_write_text("PRESS SPACE TO EXIT!", 340.0);

                on_key_press!(GLFW_KEY_SPACE,
                    break;
                );
            }
        },
        {
            cmd_sender.send(Command::Quit)?;
        }
    );

    networkingthread.join().map_err(|_| {
        Box::<dyn std::error::Error + Send + Sync>::from("Error joining networking thread")
    })?
}

use core::time;
use crossbeam_channel::unbounded;
use my_game_engine::ffi::*;
use my_game_engine::*;
use serde::Deserialize;
use std::error::Error;
use std::thread;

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
        println!("{}", resp_text);

        let spr_data: SpriteData = serde_json::from_str(&resp_text)?;
        sprite_sender.send(spr_data)?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut player_x = 390.0;
    let mut player_y = 290.0;
    let player = rust_create_sprite(player_x, player_y, 20, 20, 255, 255, 255);
    let (sprite_sender, sprite_receiver) = unbounded::<SpriteData>();
    let (cmd_sender, cmd_receiver) = unbounded::<Command>();

    let networkingthread = thread::spawn(move || networking_loop(cmd_receiver, sprite_sender));

    start_window_and_game_loop!(
        "Simple Game",
        20,
        {
            rust_render_sprite(player);
        },
        {
            //rust_clear_screen();
            on_key_press!(GLFW_KEY_LEFT,
                player_x -= 2.0;
                if player_x < 0.0 {
                    player_x = 0.0;
                }
                move_sprite!(player, false, player_x, player_y);
            );
            on_key_press!(GLFW_KEY_UP,
                player_y -= 2.0;
                if player_y < 0.0 {
                    player_y = 0.0;
                }
                move_sprite!(player, false, player_x, player_y);
            );
            on_key_press!(GLFW_KEY_RIGHT,
                player_x += 2.0;
                if player_x > 780.0 {
                    player_x = 780.0;
                }
                move_sprite!(player, false, player_x, player_y);
            );
            on_key_press!(GLFW_KEY_DOWN,
                player_y += 2.0;
                if player_y > 580.0 {
                    player_y = 580.0;
                }
                move_sprite!(player, false, player_x, player_y);
            );
            on_key_press!(GLFW_KEY_SPACE,
                cmd_sender.send(Command::GetSprite)?;
            );

            if let Ok(sprite_data) = sprite_receiver.try_recv() {
                let new_sprite = rust_create_sprite(
                    sprite_data.x,
                    sprite_data.y,
                    sprite_data.width,
                    sprite_data.height,
                    sprite_data.r,
                    sprite_data.g,
                    sprite_data.b,
                );
                rust_render_sprite(new_sprite);
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

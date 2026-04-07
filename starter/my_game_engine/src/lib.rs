mod ffi {
    use std::ffi::{c_char, c_int, c_void};

    pub const GLFW_PRESS: c_int = 1;
    pub const GLFW_KEY_SPACE: c_int = 32;
    pub const GLFW_KEY_RIGHT: c_int = 262;
    pub const GLFW_KEY_LEFT: c_int = 263;
    pub const GLFW_KEY_DOWN: c_int = 264;
    pub const GLFW_KEY_UP: c_int = 265;

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct Sprite {
        width: i32,
        height: i32,
        color: [i32; 3],
        x: f32,
        y: f32,
    }

    unsafe extern "C" {
        fn create_game_window(title: *const c_char, width: i32, height: i32);
        fn create_sprite(
            x: f32,
            y: f32,
            width: i32,
            height: i32,
            r: i32,
            g: i32,
            b: i32,
        ) -> *mut Sprite;
        fn render_sprite(sprite: *mut Sprite);
        fn update_sprite_position(sprite: *mut Sprite, x: f32, y: f32);
        fn update_game_window();
        fn clear_screen();
        fn window_should_close() -> i32;
        fn get_key(window: *mut c_void, key: i32) -> i32;
        fn get_window() -> *mut c_void;
    }

    pub fn rust_create_game_window(title: &str, width: i32, height: i32) {
        let c_title = std::ffi::CString::new(title).unwrap();
        unsafe {
            create_game_window(c_title.as_ptr(), width, height);
        }
    }

    pub fn rust_create_sprite(
        x: f32,
        y: f32,
        width: i32,
        height: i32,
        r: i32,
        g: i32,
        b: i32,
    ) -> *mut Sprite {
        unsafe { create_sprite(x, y, width, height, r, g, b) }
    }

    pub fn rust_render_sprite(sprite: *mut Sprite) {
        unsafe {
            render_sprite(sprite);
        }
    }

    pub fn rust_update_sprite_position(sprite: *mut Sprite, x: f32, y: f32) {
        unsafe {
            update_sprite_position(sprite, x, y);
        }
    }

    pub fn rust_update_game_window() {
        unsafe {
            update_game_window();
        }
    }

    pub fn rust_clear_screen() {
        unsafe {
            clear_screen();
        }
    }

    pub fn rust_window_should_close() -> bool {
        unsafe { window_should_close() != 0 }
    }

    pub fn rust_get_key(key: i32) -> i32 {
        unsafe { get_key(get_window(), key) }
    }

    pub fn rust_get_window() -> *mut c_void {
        unsafe { get_window() }
    }
}

#[cfg(test)]
mod tests {
    use super::ffi::*;
    use core::time;

    #[test]
    #[ignore]
    fn test_simple_game_loop() {
        println!("test_simple_game_loop");
        rust_create_game_window("Test title", 800, 600);
        while !rust_window_should_close() {
            rust_update_game_window();
            std::thread::sleep(time::Duration::from_millis(20));
        }
    }

    #[test]
    #[ignore]
    fn test_sprite_rendering() {
        rust_create_game_window("Test sprite rendering", 800, 600);
        let sprite = rust_create_sprite(80.0, 20.0, 200, 100, 255, 0, 0);
        rust_render_sprite(sprite);
        while !rust_window_should_close() {
            rust_update_game_window();
            std::thread::sleep(time::Duration::from_millis(20));
        }
    }

    #[test]
    #[ignore]
    fn test_screen_clearing() {
        rust_create_game_window("Test screen clearing", 800, 600);
        let sprite = rust_create_sprite(80.0, 20.0, 200, 100, 255, 0, 0);
        rust_render_sprite(sprite);
        rust_update_game_window();
        std::thread::sleep(time::Duration::from_secs(5));
        let sprite = rust_create_sprite(280.0, 220.0, 100, 200, 0, 255, 0);
        rust_clear_screen();
        rust_render_sprite(sprite);
        while !rust_window_should_close() {
            rust_update_game_window();
            std::thread::sleep(time::Duration::from_millis(20));
        }
    }

    #[test]
    #[ignore]
    fn test_key_presses() {
        rust_create_game_window("Test key presses", 800, 600);
        let left_sprite = rust_create_sprite(20.0, 220.0, 100, 100, 255, 0, 0);
        let up_sprite = rust_create_sprite(370.0, 20.0, 100, 100, 255, 0, 0);
        let right_sprite = rust_create_sprite(640.0, 220.0, 100, 100, 255, 0, 0);
        let down_sprite = rust_create_sprite(370.0, 480.0, 100, 100, 255, 0, 0);
        let space_sprite = rust_create_sprite(200.0, 280.0, 400, 70, 255, 0, 0);

        while !rust_window_should_close() {
            rust_clear_screen();
            if rust_get_key(GLFW_KEY_LEFT) == GLFW_PRESS {
                rust_render_sprite(left_sprite);
            } else if rust_get_key(GLFW_KEY_UP) == GLFW_PRESS {
                rust_render_sprite(up_sprite);
            } else if rust_get_key(GLFW_KEY_RIGHT) == GLFW_PRESS {
                rust_render_sprite(right_sprite);
            } else if rust_get_key(GLFW_KEY_DOWN) == GLFW_PRESS {
                rust_render_sprite(down_sprite);
            } else if rust_get_key(GLFW_KEY_SPACE) == GLFW_PRESS {
                rust_render_sprite(space_sprite);
            }

            rust_update_game_window();
            std::thread::sleep(time::Duration::from_millis(20));
        }
    }

    #[test]
    #[ignore]
    fn test_sprite_position_update() {
        rust_create_game_window("Test sprite position update", 800, 600);
        let mut x = 20.0;
        let y = 200.0;
        let sprite = rust_create_sprite(x, y, 100, 100, 255, 0, 0);
        rust_render_sprite(sprite);
        while !rust_window_should_close() {
            rust_clear_screen();
            if x < 640.0 {
                x += 2.0;
                rust_update_sprite_position(sprite, x, y);
            }
            rust_render_sprite(sprite);
            rust_update_game_window();
            std::thread::sleep(time::Duration::from_millis(20));
        }
    }
}

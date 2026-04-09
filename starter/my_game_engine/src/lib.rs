pub mod ffi {
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
        pub width: i32,
        pub height: i32,
        pub color: [i32; 3],
        pub x: f32,
        pub y: f32,
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

    #[macro_export]
    macro_rules! spawn_sprite {
        ($name:ident, $x:expr, $y:expr, $width:expr, $height:expr, $r:expr, $g:expr, $b:expr) => {
            let $name = rust_create_sprite($x, $y, $width, $height, $r, $g, $b);
            rust_render_sprite($name);
        };
    }

    #[macro_export]
    macro_rules! on_key_press {
        ($key:expr, $($action:tt)+) => {
            if rust_get_key($key) == GLFW_PRESS {
                $($action)+
            }
        };
    }

    #[macro_export]
    macro_rules! tick {
        ($sleep_dur:expr) => {
            rust_update_game_window();
            std::thread::sleep(time::Duration::from_millis($sleep_dur));
        };
    }

    #[macro_export]
    macro_rules! start_window_and_game_loop {
        ($title:expr, $sleep_dur:expr, $start_action:block, $iter_action:block, $exit_action:block) => {
            rust_create_game_window($title, 800, 600);
            $start_action
            while !rust_window_should_close() {
                $iter_action
                tick!($sleep_dur);
            }
            $exit_action
        };
    }

    #[macro_export]
    macro_rules! move_sprite {
        ($sprite:expr, $clear:expr, $x:expr, $y:expr) => {
            if $clear {
                rust_clear_screen();
            }
            rust_update_sprite_position($sprite, $x, $y);
            rust_render_sprite($sprite);
        };
    }

    #[macro_export]
    macro_rules! change_sprite_color {
        ($new_sprite:ident, $old_sprite:expr, $r:expr, $g:expr, $b:expr) => {
            unsafe {
                spawn_sprite!(
                    $new_sprite,
                    (*$old_sprite).x,
                    (*$old_sprite).y,
                    (*$old_sprite).width,
                    (*$old_sprite).height,
                    $r,
                    $g,
                    $b
                );
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use super::ffi::*;
    use super::*;
    use core::time;

    #[test]
    #[ignore]
    fn test_simple_game_loop() {
        start_window_and_game_loop!("Test title", 20, {}, {}, {});
    }

    #[test]
    #[ignore]
    fn test_sprite_rendering() {
        start_window_and_game_loop!(
            "Test sprite rendering",
            20,
            {
                spawn_sprite!(sprite, 80.0, 20.0, 200, 100, 255, 0, 0);
            },
            {},
            {}
        );
    }

    #[test]
    #[ignore]
    fn test_screen_clearing() {
        start_window_and_game_loop!(
            "Test screen clearing",
            20,
            {
                spawn_sprite!(sprite, 80.0, 20.0, 200, 100, 255, 0, 0);
                tick!(4000);
                rust_clear_screen();
                tick!(1000);
                change_sprite_color!(new_sprite, sprite, 255, 255, 0);
            },
            {},
            {}
        );
    }

    #[test]
    #[ignore]
    fn test_key_presses() {
        let left_sprite = rust_create_sprite(20.0, 220.0, 100, 100, 255, 0, 0);
        let up_sprite = rust_create_sprite(370.0, 20.0, 100, 100, 255, 0, 0);
        let right_sprite = rust_create_sprite(640.0, 220.0, 100, 100, 255, 0, 0);
        let down_sprite = rust_create_sprite(370.0, 480.0, 100, 100, 255, 0, 0);
        let space_sprite = rust_create_sprite(200.0, 280.0, 400, 70, 255, 0, 0);
        start_window_and_game_loop!(
            "Test key presses",
            20,
            {},
            {
                rust_clear_screen();
                on_key_press!(GLFW_KEY_LEFT, rust_render_sprite(left_sprite););
                on_key_press!(GLFW_KEY_UP, rust_render_sprite(up_sprite););
                on_key_press!(GLFW_KEY_RIGHT, rust_render_sprite(right_sprite););
                on_key_press!(GLFW_KEY_DOWN, rust_render_sprite(down_sprite););
                on_key_press!(GLFW_KEY_SPACE, rust_render_sprite(space_sprite););
            },
            {}
        );
    }

    #[test]
    #[ignore]
    fn test_sprite_position_update() {
        let mut x = 20.0;
        let y = 200.0;
        spawn_sprite!(sprite, x, y, 100, 100, 255, 0, 0);
        start_window_and_game_loop!(
            "Test sprite position update",
            20,
            {},
            {
                if x < 640.0 {
                    x += 2.0;
                    move_sprite!(sprite, true, x, y);
                }
            },
            {}
        );
    }
}

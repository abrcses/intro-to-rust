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

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

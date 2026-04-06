fn main() {
    cc::Build::new()
        .file("../opengl_wrapper_lib/opengl_wrapper_lib.c")
        .pic(true)
        .compile("openglwrapper");

    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=GL");
    println!("cargo:rerun-if-changed=../opengl_wrapper_lib/opengl_wrapper_lib.c");
}

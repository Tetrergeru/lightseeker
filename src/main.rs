pub mod app;
pub mod camera;
pub mod color;
pub mod download;
pub mod geometry;
pub mod gl_context;
pub mod objects;
pub mod shaders;
pub mod light;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}

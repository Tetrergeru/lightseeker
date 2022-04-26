pub mod app;
pub mod camera;
pub mod color;
pub mod download;
pub mod gl_context;
pub mod matrix;
pub mod objects;
pub mod shaders;
pub mod vector;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}

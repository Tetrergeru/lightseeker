pub mod app;
pub mod color;
pub mod gl_context;
pub mod matrix;
pub mod shaders;
pub mod shape;
pub mod vector;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
}

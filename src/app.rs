use std::{collections::HashMap, rc::Rc};

use gloo::{events::EventListener, utils::document};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, KeyboardEvent, MouseEvent, WebGl2RenderingContext};
use yew::{html, Component, Context, Html, NodeRef};

use crate::{
    camera::Camera,
    download::{download_image, download_text},
    gl_context::GlContext,
    matrix::Matrix,
    objects::{object::Object, shape::Shape, texture::Texture},
    vector::{Vector2, Vector3},
};

pub struct App {
    canvas_ref: NodeRef,
    context: Option<GlContext>,

    shapes: HashMap<String, Rc<Shape>>,
    textures: HashMap<String, Rc<Texture>>,
    objects: Vec<Object>,
    currently_downloading: usize,

    camera: Camera,
    mouse_down: bool,
    size: Vector2,

    _keydown_listener: EventListener,
}

pub enum Msg {
    KeyDown(KeyboardEvent),
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    MouseUp(MouseEvent),
    ShapeLoaded(String, Shape),
    TextureLoaded(String, Texture),
}

impl Component for App {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let onkeydown = ctx.link().callback(Msg::KeyDown);
        let keydown_listener = EventListener::new(&document(), "keydown", move |e| {
            let e = e.clone().unchecked_into::<KeyboardEvent>();
            onkeydown.emit(e);
        });

        let size = Vector2::from_xy(1100.0, 800.0);

        Self {
            canvas_ref: NodeRef::default(),

            shapes: HashMap::new(),
            textures: HashMap::new(),
            objects: vec![],
            currently_downloading: 0,

            camera: Camera::new(Vector3::zero(), 0.0, 0.0).with_aspect(size.x() / size.y()),
            mouse_down: false,
            size,

            context: None,
            _keydown_listener: keydown_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyDown(e) => {
                let key = e.code();
                match key.as_str() {
                    "KeyW" => self.camera.move_h(Vector2::from_xy(0.0, 0.2)),
                    "KeyS" => self.camera.move_h(Vector2::from_xy(0.0, -0.2)),
                    "KeyA" => self.camera.move_h(Vector2::from_xy(0.2, 0.0)),
                    "KeyD" => self.camera.move_h(Vector2::from_xy(-0.2, 0.0)),
                    _ => (),
                }
                self.draw();
                false
            }
            Msg::ShapeLoaded(name, shape) => {
                self.shapes.insert(name, Rc::new(shape));
                self.single_download_finished();
                false
            }
            Msg::TextureLoaded(name, texture) => {
                self.textures.insert(name, Rc::new(texture));
                self.single_download_finished();
                false
            }
            Msg::MouseDown(_) => {
                self.mouse_down = true;
                false
            }
            Msg::MouseMove(e) => {
                if self.mouse_down {
                    let x = e.movement_x() as f32;
                    let y = e.movement_y() as f32;
                    log::debug!("Msg::MouseMove x = {}, y = {}", x, y);
                    self.camera.rotate_v(-y / self.size.y() * 10.0);
                    self.camera.rotate_h(x / self.size.x() * 10.0);
                    self.draw();
                }
                false
            }
            Msg::MouseUp(_) => {
                self.mouse_down = false;
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <canvas
                style={format!(
                    "width: {}px; height: {}px; border: solid black;",
                    self.size.x() as usize,
                    self.size.y() as usize,
                )}
                ref={self.canvas_ref.clone()}
                onmousedown={ctx.link().callback(Msg::MouseDown)}
                onmouseup={ctx.link().callback(Msg::MouseUp)}
                onmousemove={ctx.link().callback(Msg::MouseMove)}
            />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            self.context = Some(GlContext::new(
                canvas,
                self.size.x() as u32,
                self.size.y() as u32,
            ));

            for (path, name) in Self::required_shapes() {
                let callback = ctx
                    .link()
                    .clone()
                    .callback(|(str, shape)| Msg::ShapeLoaded(str, shape));

                self.currently_downloading += 1;
                let gl = self.gl();
                spawn_local(async move {
                    let text = download_text(&path).await;
                    callback.emit((name, Shape::parse_obj_file(&text, &gl)));
                });
            }
            for (path, name) in Self::required_textures() {
                let callback = ctx
                    .link()
                    .clone()
                    .callback(|(str, shape)| Msg::TextureLoaded(str, shape));

                self.currently_downloading += 1;
                let gl = self.gl();
                spawn_local(async move {
                    let image = download_image(&path).await;
                    callback.emit((name, Texture::new(image, &gl)));
                });
            }
        }

        self.draw();
    }
}

impl App {
    fn required_shapes() -> Vec<(String, String)> {
        [
            ("resources/skull.obj", "Skull"),
            ("resources/Crate1.obj", "Cube"),
            ("resources/floor.obj", "Floor"),
        ]
        .map(|(path, name)| (path.to_string(), name.to_string()))
        .into_iter()
        .collect()
    }

    fn required_textures() -> Vec<(String, String)> {
        [
            ("resources/skull.jpg", "Skull"),
            ("resources/crate_1.jpg", "Grass"),
            ("resources/carpet.jpg", "Carpet"),
        ]
        .map(|(path, name)| (path.to_string(), name.to_string()))
        .into_iter()
        .collect()
    }

    fn gl(&self) -> WebGl2RenderingContext {
        self.context.as_ref().unwrap().gl()
    }

    fn draw(&mut self) {
        for obj in self.objects.iter_mut() {
            self.context
                .as_ref()
                .unwrap()
                .checkerboard(obj, self.camera.matrix() * obj.transform);
        }
    }

    fn single_download_finished(&mut self) {
        self.currently_downloading -= 1;

        if self.currently_downloading == 0 {
            self.on_downloaded();
        }
    }

    fn on_downloaded(&mut self) {
        self.objects.push(Object::new(
            self.shapes["Skull"].clone(),
            self.textures["Skull"].clone(),
            Matrix::translate(Vector3::from_xyz(0.0, -0.3, 0.0)) * Matrix::rotation_x(1.2 * std::f32::consts::PI / 2.0) * Matrix::scale(0.1),
        ));
        self.objects.push(Object::new(
            self.shapes["Cube"].clone(),
            self.textures["Grass"].clone(),
            Matrix::translate(Vector3::from_xyz(5.0, -1.0, 5.0)),
        ));
        self.objects.push(Object::new(
            self.shapes["Cube"].clone(),
            self.textures["Grass"].clone(),
            Matrix::translate(Vector3::from_xyz(0.0, -1.0, 0.0)),
        ));
        self.objects.push(Object::new(
            self.shapes["Floor"].clone(),
            self.textures["Carpet"].clone(),
            Matrix::translate(Vector3::from_xyz(0.0, -2.0, 0.0)) * Matrix::scale(5.0),
        ));
    }
}

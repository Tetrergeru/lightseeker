use std::{collections::HashMap, rc::Rc};

use gloo::{events::EventListener, utils::document};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, KeyboardEvent, MouseEvent, WebGl2RenderingContext as Gl};
use yew::{html, Component, Context, Html, NodeRef};

use crate::{
    camera::Camera,
    download::{download_image, download_text},
    gl_context::GlContext,
    light_src::LightSrc,
    objects::{object::Object, shape::Shape, texture::Texture},
    transform::Transform,
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
    light: Vec<LightSrc>,
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

            camera: Camera::new(Vector3::from_xyz(-8.0, 0.0, -8.0), 0.0, 0.0)
                .with_aspect(size.x() / size.y()),
            mouse_down: false,
            size,
            light: vec![],

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
                    "ArrowDown" => self.light[0].move_h(-0.2, -0.0),
                    "ArrowUp" => self.light[0].move_h(0.2, 0.0),
                    "ArrowLeft" => self.light[0].rotate_h(0.2),
                    "ArrowRight" => self.light[0].rotate_h(-0.2),
                    "Digit1" => self.light[0].add_fov(0.1),
                    "Digit2" => self.light[0].add_fov(-0.1),
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

    fn gl(&self) -> Gl {
        self.context.as_ref().unwrap().gl()
    }

    fn draw(&mut self) {
        let gl = self.gl();
        gl.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);
        for light in self.light.iter() {
            self.context
                .as_ref()
                .unwrap()
                .wire_light(light, self.camera.matrix());

            light.bind(&gl);
            gl.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);
            for obj in self.objects.iter_mut() {
                if obj.ignored_by_light {
                    continue;
                }
                self.context.as_ref().unwrap().render_light(obj, light);
            }
            gl.bind_framebuffer(Gl::FRAMEBUFFER, None);
        }

        for obj in self.objects.iter_mut() {
            self.context
                .as_ref()
                .unwrap()
                .view(obj, &self.camera, &self.light);
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
            {
                let mut t = Transform::from_xyz(0.0, -0.3, 0.0);
                t.rotate_v(1.2 * std::f32::consts::PI / 2.0);
                t.scale(0.1);
                t
            },
        ));
        self.objects.push(Object::new(
            self.shapes["Cube"].clone(),
            self.textures["Grass"].clone(),
            Transform::from_xyz(5.0, -1.0, 5.0),
        ));
        self.objects.push(Object::new(
            self.shapes["Cube"].clone(),
            self.textures["Grass"].clone(),
            Transform::from_xyz(0.0, -1.0, 0.0),
        ));
        self.objects.push(Object::new(
            self.shapes["Floor"].clone(),
            self.textures["Carpet"].clone(),
            {
                let mut t = Transform::from_xyz(0.0, -2.0, 0.0);
                t.scale(5.0);
                t
            },
        ));

        let light = LightSrc::new(
            &self.gl(),
            Vector3::from_xyz(0.0, 3.0, -5.0),
            std::f32::consts::PI * 1.0,
            0.6,
        );
        self.light.push(light);

        let light = LightSrc::new(
            &self.gl(),
            Vector3::from_xyz(0.0, 3.0, 5.0),
            std::f32::consts::PI * 0.0,
            0.6,
        );

        // self.objects.push(Object::new(
        //     self.shapes["Cube"].clone(),
        //     self.textures["Grass"].clone(),
        //     {
        //         let mut t = Transform::from_xyz(1.0, 3.0, 5.0);
        //         t.rotate_v(0.6);
        //         t
        //     },
        // ));

        self.light.push(light);
    }
}

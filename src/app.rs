use std::{collections::HashMap, rc::Rc};

use gloo::{events::EventListener, utils::document};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, KeyboardEvent, WebGl2RenderingContext};
use yew::{html, Component, Context, Html, NodeRef};

use crate::{
    color::Color,
    download::download_text,
    gl_context::GlContext,
    matrix::Matrix,
    objects::{object::Object, shape::Shape},
    vector::Vector4,
};

pub struct App {
    canvas_ref: NodeRef,

    shapes: HashMap<String, Rc<Shape>>,
    objects: Vec<Object>,
    currently_downloading: usize,

    position: Vector4,
    angle: f32,
    context: Option<GlContext>,

    _keydown_listener: EventListener,
}

pub enum Msg {
    KeyDown(KeyboardEvent),
    ShapeLoaded(String, Shape),
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

        Self {
            canvas_ref: NodeRef::default(),

            shapes: HashMap::new(),
            objects: vec![],
            currently_downloading: 0,

            position: Vector4::from_xyz(0.0, 0.0, -10.0),
            angle: 0.0,
            context: None,
            _keydown_listener: keydown_listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyDown(e) => {
                let key = e.code();
                let mut need_log = true;
                match key.as_str() {
                    "KeyA" => self.position += Vector4::from_xyzw(0.5, 0.0, 0.0, 0.0),
                    "KeyD" => self.position += Vector4::from_xyzw(-0.5, 0.0, 0.0, 0.0),
                    "KeyW" => self.position += Vector4::from_xyzw(0.0, 0.0, 0.5, 0.0),
                    "KeyS" => self.position += Vector4::from_xyzw(0.0, 0.0, -0.5, 0.0),
                    "KeyQ" => self.angle -= 0.1,
                    "KeyE" => self.angle += 0.1,
                    _ => need_log = false,
                }
                if need_log {
                    log::debug!("App update KeyDown({}) {:?}", key.as_str(), self.position);
                }
                self.draw();
                false
            }
            Msg::ShapeLoaded(name, shape) => {
                self.shapes.insert(name, Rc::new(shape));
                self.currently_downloading -= 1;

                if self.currently_downloading == 0 {
                    self.on_downloaded();
                }

                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas
                style="width: 700px; height: 700px;"
                ref={self.canvas_ref.clone()}
            />
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.canvas_ref.cast::<HtmlCanvasElement>().unwrap();
            self.context = Some(GlContext::new(canvas, 1000, 1000));

            let callback = ctx
                .link()
                .clone()
                .callback(|(str, shape)| Msg::ShapeLoaded(str, shape));

            self.currently_downloading += 1;
            let gl = self.gl();
            spawn_local(async move {
                let text = download_text("resources/skull.obj").await;
                callback.emit(("Skull".to_string(), Shape::parse_obj_file(&text, &gl)));
            });
        }

        self.draw();
    }
}

impl App {
    fn gl(&self) -> WebGl2RenderingContext {
        self.context.as_ref().unwrap().gl()
    }

    fn draw(&mut self) {
        let mut matrix = Matrix::ident();
        matrix = matrix * Matrix::perspective(1.5, 1.0, 0.1, 2000.0);
        matrix = matrix * Matrix::rotation_y(self.angle);
        matrix = matrix * Matrix::translate(self.position);
        for obj in self.objects.iter_mut() {
            self.context.as_ref().unwrap().checkerboard(
                obj,
                matrix * obj.transform,
                20.0,
                Color::RED,
                Color::RED,
            );
        }
    }

    fn on_downloaded(&mut self) {
        self.objects.push(Object::new(
            self.shapes["Skull"].clone(),
            Matrix::scale(1.0),
        ))
    }
}

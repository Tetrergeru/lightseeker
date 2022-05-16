use gloo::{events::EventListener, utils::document};
use gloo_render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, KeyboardEvent, MouseEvent, WebGl2RenderingContext as Gl};
use yew::{html, Component, Context, Html, NodeRef};

use crate::{
    camera::Camera,
    controls::{ControlKey, Controls},
    download::{ResourceBatch, ResourceManager},
    geometry::{Transform, Vector2},
    gl_context::GlContext,
    world::World,
};

pub struct App {
    canvas_ref: NodeRef,
    context: Option<GlContext>,

    resources: ResourceManager,
    world: World,

    mouse_down: bool,
    controls: Controls,
    size: Vector2,

    timer_start: f64,
    last_tick: f64,
    frames: usize,

    _keydown_listener: EventListener,
    _keyup_listener: EventListener,
    _frame: Option<AnimationFrame>,
}

pub enum Msg {
    KeyDown(KeyboardEvent),
    KeyUp(KeyboardEvent),
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    MouseUp(MouseEvent),
    ResourcesLoaded(ResourceBatch),
    Timer(f64),
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
        let onkeyup = ctx.link().callback(Msg::KeyUp);
        let keyup_listener = EventListener::new(&document(), "keyup", move |e| {
            let e = e.clone().unchecked_into::<KeyboardEvent>();
            onkeyup.emit(e);
        });

        let size = Vector2::from_xy(1100.0, 800.0);

        let rm = ResourceManager::new();

        Self {
            canvas_ref: NodeRef::default(),

            resources: rm.clone(),
            world: World::new(
                rm,
                Camera::new(Transform::from_xyz(0.0, 0.0, 0.0)).with_aspect(size.x() / size.y()),
            ),
            mouse_down: false,
            controls: Controls::new(),
            size,

            timer_start: 0.0,
            last_tick: 0.0,
            frames: 0,

            context: None,
            _keydown_listener: keydown_listener,
            _keyup_listener: keyup_listener,
            _frame: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyDown(e) => {
                self.controls.down(ControlKey::from_string(&e.code()));
                false
            }
            Msg::KeyUp(e) => {
                self.controls.up(ControlKey::from_string(&e.code()));
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
                    self.world.camera.rotate_v(y / self.size.y() * 10.0);
                    self.world.camera.rotate_h(x / self.size.x() * 10.0);
                }
                false
            }
            Msg::MouseUp(_) => {
                self.mouse_down = false;
                false
            }
            Msg::Timer(t) => {
                if self.frames == 0 {
                    self.timer_start = t;
                    self.last_tick = t;
                    self.frames = 1;
                }

                self.frames += 1;
                if self.frames == 60 {
                    log::debug!(
                        "App udate Timer fps: {}",
                        ((self.frames as f64 - 1.0) / (t - self.timer_start)) * 1000.0
                    );
                    self.timer_start = t;
                    self.frames = 1;
                }

                self.world
                    .tick((t - self.last_tick) as f32 / 1000.0, &self.controls);
                self.last_tick = t;
                self.draw();
                self.request_frame(ctx);
                false
            }
            Msg::ResourcesLoaded(res) => {
                self.resources.merge(res);
                self.on_downloaded(ctx);
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

            let callback = ctx.link().clone().callback(Msg::ResourcesLoaded);

            let res = self.world.required_resources();
            let gl = self.gl();
            spawn_local(async move {
                let res = ResourceManager::download(res, gl).await;
                callback.emit(res);
            });
        }

        self.draw();
    }
}

impl App {
    fn gl(&self) -> Gl {
        self.context.as_ref().unwrap().gl()
    }

    fn draw(&mut self) {
        self.world.draw(self.context.as_ref().unwrap());
    }

    fn request_frame(&mut self, ctx: &Context<Self>) {
        self._frame = Some({
            let link = ctx.link().clone();
            request_animation_frame(move |time| link.send_message(Msg::Timer(time)))
        })
    }

    fn on_downloaded(&mut self, ctx: &Context<Self>) {
        self.world.init_0(self.context.as_ref().unwrap());
        self.request_frame(ctx);
    }
}

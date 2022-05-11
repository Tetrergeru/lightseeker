use std::rc::Rc;

use gloo::{events::EventListener, utils::document};
use gloo_render::{request_animation_frame, AnimationFrame};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, KeyboardEvent, MouseEvent, WebGl2RenderingContext as Gl};
use yew::{html, Component, Context, Html, NodeRef};

use crate::{
    camera::Camera,
    download::{DownloadManager, ResourceRequest},
    geometry::{Transform, Vector2, Vector3},
    gl_context::GlContext,
    light::Light,
    objects::{
        object::Object,
        parsers::{animation::Animation, skeleton::Skeleton, skinning::Skinning},
        shape::Shape,
    },
};

pub struct App {
    canvas_ref: NodeRef,
    context: Option<GlContext>,

    objects: Vec<Object>,
    picked_object: usize,
    resources: DownloadManager,

    camera: Camera,
    lights: Vec<Light>,
    animation: Option<Animation>,
    animation_frame: isize,

    mouse_down: bool,
    size: Vector2,

    timer_start: f64,
    frames: usize,

    _keydown_listener: EventListener,
    _frame: Option<AnimationFrame>,
}

pub enum Msg {
    KeyDown(KeyboardEvent),
    MouseDown(MouseEvent),
    MouseMove(MouseEvent),
    MouseUp(MouseEvent),
    ResourcesLoaded(DownloadManager),
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

        let size = Vector2::from_xy(1100.0, 800.0);

        Self {
            canvas_ref: NodeRef::default(),

            objects: vec![],
            picked_object: 0,
            resources: DownloadManager::new(),

            camera: Camera::new(Vector3::from_xyz(-8.0, 0.0, -8.0), 0.0, 0.0)
                .with_aspect(size.x() / size.y()),
            mouse_down: false,
            size,
            lights: vec![],
            animation: None,
            animation_frame: 0,

            timer_start: 0.0,
            frames: 0,

            context: None,
            _keydown_listener: keydown_listener,
            _frame: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyDown(e) => {
                let key = e.code();
                match key.as_str() {
                    "KeyW" => self.camera.move_h(Vector2::from_xy(0.0, 0.2)),
                    "KeyS" => self.camera.move_h(Vector2::from_xy(0.0, -0.2)),
                    "KeyA" => self.camera.move_h(Vector2::from_xy(0.2, 0.0)),
                    "KeyD" => self.camera.move_h(Vector2::from_xy(-0.2, 0.0)),
                    "KeyE" => self.camera.move_h(Vector2::from_xy(-0.2, 0.0)),
                    "ShiftLeft" => self.camera.move_v(-0.2),
                    "Space" => self.camera.move_v(0.2),
                    // "ArrowDown" => self.move_picked(Vector3::from_xyz(-0.2, 0.0, 0.0)),
                    // "ArrowUp" => self.move_picked(Vector3::from_xyz(0.2, 0.0, 0.0)),
                    "ArrowLeft" => self.move_picked(Vector3::from_xyz(0.0, 0.0, 0.2)),
                    "ArrowRight" => self.move_picked(Vector3::from_xyz(0.0, 0.0, -0.2)),
                    // "Digit1" => self.move_picked(Vector3::from_xyz(0.0, 0.2, 0.0)),
                    // "Digit2" => self.move_picked(Vector3::from_xyz(0.0, -0.2, 0.0)),
                    _ => (),
                }
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
                    self.frames = 1;
                }

                self.frames += 1;
                if self.frames == 60 {
                    // log::debug!(
                    //     "App udate Timer fps: {}",
                    //     ((self.frames as f64 - 1.0) / (t - self.timer_start)) * 1000.0
                    // );
                    self.timer_start = t;
                    self.frames = 1;
                }

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

            let callback = ctx.link().clone().callback(|res| Msg::ResourcesLoaded(res));

            let res = self.required_resources();
            let gl = self.gl();
            spawn_local(async move {
                let res = DownloadManager::download(res, gl).await;
                callback.emit(res);
            });
        }

        self.draw();
    }
}

impl App {
    fn move_picked(&mut self, d: Vector3) {
        if let Some(anim) = &self.animation {
            let delta = if d.z() > 0.0 { 1 } else { -1 };

            self.animation_frame = (self.animation_frame + delta + anim.frames.len() as isize)
                % anim.frames.len() as isize;

            log::debug!("App move_picked animation_frame = {}", self.animation_frame,);

            let picked = &self.picked_object();
            picked.set_pose(&anim.frames[self.animation_frame as usize])
        }
    }

    fn required_resources(&self) -> Vec<ResourceRequest> {
        use ResourceRequest as RR;
        vec![
            RR::text("resources/skull.obj", "Skull"),
            RR::text("resources/Crate1.obj", "Cube"),
            RR::text("resources/floor.obj", "Floor"),
            RR::text("resources/walk.skl", "Walk.skl"),
            RR::text("resources/walk.skin", "Walk.skin"),
            RR::text("resources/walk.anim", "Walk.anim"),
            RR::text("resources/walk.obj", "Walk"),
            RR::image("resources/skull.jpg", "Skull"),
            RR::image("resources/crate_1.jpg", "Grass"),
            RR::image("resources/carpet.jpg", "Carpet"),
        ]
    }

    fn gl(&self) -> Gl {
        self.context.as_ref().unwrap().gl()
    }

    fn draw(&mut self) {
        let gl = self.gl();
        let context = self.context.as_ref().unwrap();
        gl.clear(Gl::COLOR_BUFFER_BIT | Gl::DEPTH_BUFFER_BIT);

        for light in self.lights.iter() {
            if let Light::Directional(d) = light {
                context.wire_light(d.matrix(), self.camera.matrix());
            } else if let Light::Point(p) = light {
                for m in p.matrices_with_nf(0.3, 0.31) {
                    context.wire_light(m, self.camera.matrix());
                }
            }
            context.bind_framebuffer(light);
            context.clear();
            for (idx, obj) in self.objects.iter_mut().enumerate() {
                if idx == self.picked_object {
                    continue;
                }
                if obj.ignored_by_light {
                    continue;
                }
                context.render_light(obj, light);
            }
            context.unbind_framebuffer();
        }
        for (idx, obj) in self.objects.iter_mut().enumerate() {
            if idx == self.picked_object {
                // continue;
            }
            self.context
                .as_ref()
                .unwrap()
                .view(obj, &self.camera, &self.lights);
        }
    }

    fn request_frame(&mut self, ctx: &Context<Self>) {
        self._frame = Some({
            let link = ctx.link().clone();
            request_animation_frame(move |time| link.send_message(Msg::Timer(time)))
        })
    }

    fn picked_object(&self) -> &Object {
        &self.objects[self.picked_object]
    }

    fn on_downloaded(&mut self, ctx: &Context<Self>) {
        let gl = self.gl();
        let skull = Rc::new(Shape::parse(&self.resources.get_text("Skull"), &gl));
        let cube = Rc::new(Shape::parse(&self.resources.get_text("Cube"), &gl));
        let floor = Rc::new(Shape::parse(&self.resources.get_text("Floor"), &gl));

        let skl = Rc::new(Skeleton::from_file(&self.resources.get_text("Walk.skl")));
        let skin = Rc::new(Skinning::parse(&self.resources.get_text("Walk.skin")));
        let anim = Animation::parse(&self.resources.get_text("Walk.anim"));
        let bell = Rc::new(Shape::parse_with_skin(
            &self.resources.get_text("Walk"),
            &skin,
            &gl,
        ));
        let grass_texture = self.resources.get_texture("Grass");
        let skull_texture = self.resources.get_texture("Skull");
        let carpet_texture = self.resources.get_texture("Carpet");

        self.objects.push(
            Object::new(bell, grass_texture.clone(), {
                let t = Transform::from_xyz_hv(10.0, -2.0, 0.0, 0.0, 0.0);
                // t.rotate_h(-1.57);
                t
            })
            .with_skeleton(&skl),
        );
        self.picked_object = self.objects.len() - 1;

        self.objects
            .push(Object::new(skull, skull_texture.clone(), {
                let t = Transform::from_xyz(0.0, -0.3, 0.0);
                t.rotate_v(1.2 * std::f32::consts::PI / 2.0);
                t.scale(0.1);
                t
            }));
        self.objects.push(Object::new(
            cube.clone(),
            grass_texture.clone().clone(),
            Transform::from_xyz(5.0, -1.0, 5.0),
        ));
        self.objects.push(Object::new(
            cube.clone(),
            grass_texture.clone(),
            Transform::from_xyz(0.0, -1.0, 0.0),
        ));

        let carpet_transform = {
            let t = Transform::from_xyz(0.0, -2.0, 0.0);
            t.scale(5.0);
            t
        };
        self.objects.push(Object::new(
            floor.clone(),
            carpet_texture.clone(),
            carpet_transform.clone(),
        ));

        self.objects
            .push(Object::new(floor.clone(), carpet_texture.clone(), {
                let t = Transform::from_xyz(0.0, 0.0, -2.0);
                t.set_parent(carpet_transform);
                t
            }));
        self.objects
            .push(Object::new(floor.clone(), carpet_texture.clone(), {
                let t = Transform::from_xyz(0.0, 4.0, 0.0);
                t.scale(5.0);
                t.rotate_v(std::f32::consts::PI);
                t
            }));
        self.objects
            .push(Object::new(floor, carpet_texture.clone(), {
                let t = Transform::from_xyz(0.0, 4.0, -10.0);
                t.scale(5.0);
                t.rotate_v(std::f32::consts::PI);
                t
            }));
        self.objects.push(Object::new(
            cube,
            grass_texture.clone(),
            Transform::from_xyz(0.0, -1.0, -5.0),
        ));

        let light = Light::new_directional(
            &self.gl(),
            Transform::from_xyz_hv(0.0, 0.0, -5.0, std::f32::consts::PI * 1.0, -0.6),
        )
        .with_color(Vector3::from_xyz(0.0, 0.0, 1.0));
        self.lights.push(light);

        let light = Light::new_directional(
            &self.gl(),
            Transform::from_xyz_hv(0.0, 3.0, 5.0, std::f32::consts::PI * 0.0, 0.6),
        )
        .with_color(Vector3::from_xyz(1.0, 1.0, 0.0));
        self.lights.push(light);

        let light = Light::new_point(&self.gl(), Transform::from_xyz(0.0, 1.0, -3.0))
            .with_color(Vector3::from_xyz(0.8, 0.8, 0.3));
        self.lights.push(light);

        self.request_frame(ctx);

        self.animation = Some(anim);
    }
}

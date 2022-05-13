use std::rc::Rc;

use crate::{
    camera::Camera,
    controls::{ControlKey, Controls},
    download::{ResourceManager, ResourceRequest},
    geometry::{Transform, Vector2, Vector3},
    gl_context::GlContext,
    light::Light,
    objects::{
        object::Object,
        parsers::{animation::Animation, skeleton::Skeleton, skinning::Skinning},
        particles::Particles,
        rigid_body::RigidBody,
        shape::Shape,
    },
};

pub struct World {
    rm: ResourceManager,

    objects: Vec<Object>,
    particles: Vec<Particles>,
    bodies: Vec<RigidBody>,

    picked_object: isize,
    animation: Option<Animation>,
    animation_frame: isize,

    lights: Vec<Light>,
    pub camera: Camera,
}

impl World {
    pub fn new(rm: ResourceManager, camera: Camera) -> Self {
        Self {
            rm,

            objects: vec![],
            particles: vec![],
            bodies: vec![],

            picked_object: -1,
            animation: None,
            animation_frame: 0,

            lights: vec![],
            camera,
        }
    }

    pub fn required_resources(&self) -> Vec<ResourceRequest> {
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

    pub fn move_picked(&mut self, d: Vector3) {
        if let Some(anim) = &self.animation {
            let delta = if d.z() > 0.0 { 1 } else { -1 };

            self.animation_frame = (self.animation_frame + delta + anim.frames.len() as isize)
                % anim.frames.len() as isize;

            log::debug!("App move_picked animation_frame = {}", self.animation_frame,);

            for i in 0..10 {
                let picked = &self.objects[self.picked_object as usize + i];
                picked.set_pose(
                    &anim.frames[(self.animation_frame as usize + i + anim.frames.len())
                        % anim.frames.len()],
                )
            }
        }
    }

    pub fn tick(&mut self, delta_time: f32, controls: &Controls) {
        self.tick_controls(delta_time, controls);
    }

    fn tick_controls(&mut self, delta_time: f32, controls: &Controls) {
        use ControlKey::*;
        for key in controls.keys_down() {
            match key {
                Forward => self.camera.move_h(Vector2::from_xy(0.0, 0.2) * delta_time),
                Back => self.camera.move_h(Vector2::from_xy(0.0, -0.2) * delta_time),
                Right => self.camera.move_h(Vector2::from_xy(-0.2, 0.0) * delta_time),
                Left => self.camera.move_h(Vector2::from_xy(0.2, 0.0) * delta_time),
                Jump => self.camera.move_v(-0.2 * delta_time),
                Crouch => self.camera.move_v(0.2 * delta_time),
                Extra1 => self.move_picked(Vector3::from_xyz(-0.2, 0.0, 0.0)),
                Extra2 => self.move_picked(Vector3::from_xyz(0.2, 0.0, 0.0)),
                Extra3 => self.move_picked(Vector3::from_xyz(0.0, 0.0, 0.2)),
                Extra4 => self.move_picked(Vector3::from_xyz(0.0, 0.0, -0.2)),
                Extra5 => self.move_picked(Vector3::from_xyz(0.0, 0.2, 0.0)),
                Extra6 => self.bodies[0].collide(&self.bodies[1]),
                _ => (),
            }
        }
    }

    pub fn draw(&self, context: &GlContext) {
        for light in self.lights.iter() {
            if let Light::Directional(d) = light {
                context.wire_light(
                    d.matrix(),
                    self.camera.matrix(),
                    Vector3::from_xyz(0.0, 0.0, 1.0),
                );
            } else if let Light::Point(p) = light {
                for m in p.matrices_with_nf(0.3, 0.31) {
                    context.wire_light(m, self.camera.matrix(), Vector3::from_xyz(0.0, 0.0, 0.0));
                }
            }
            context.bind_framebuffer(light);
            context.clear();
            for obj in self.objects.iter() {
                if obj.ignored_by_light {
                    continue;
                }
                context.render_light(obj, light);
            }
            context.unbind_framebuffer();
        }
        for obj in self.objects.iter() {
            context.view(obj, &self.camera, &self.lights);
        }
        for part in self.particles.iter() {
            context.particles(part, &self.camera);
        }
        for body in self.bodies.iter() {
            context.wire_light(body.frame_matrix(), self.camera.matrix(), Vector3::from_xyz(0.2, 1.0, 0.2));
        }
    }

    pub fn init_0(&mut self, context: &GlContext) {
        let gl = context.gl();
        let skull = Rc::new(Shape::parse(&self.rm.get_text("Skull"), &gl));
        let cube = Rc::new(Shape::parse(&self.rm.get_text("Cube"), &gl));
        let floor = Rc::new(Shape::parse(&self.rm.get_text("Floor"), &gl));

        let skl = Rc::new(Skeleton::from_file(&self.rm.get_text("Walk.skl")));
        let skin = Rc::new(Skinning::parse(&self.rm.get_text("Walk.skin")));
        let anim = Animation::parse(&self.rm.get_text("Walk.anim"));
        let bell = Rc::new(Shape::parse_with_skin(
            &self.rm.get_text("Walk"),
            &skin,
            &gl,
        ));
        let grass_texture = self.rm.get_texture("Grass");
        let skull_texture = self.rm.get_texture("Skull");
        let carpet_texture = self.rm.get_texture("Carpet");

        // Rigid bodies

        let body = RigidBody::new(Vector3::from_xyz(1.0, 2.0, 1.0), Vector3::zero(), Transform::from_xyz(0.0, -0.5, -9.1)).as_movable();
        self.bodies.push(body);
        let body = RigidBody::new(Vector3::from_xyz(1.0, 1.0, 1.0), Vector3::zero(), Transform::from_xyz(0.0, 0.85, -10.0)).as_movable();
        self.bodies.push(body);

        // Particles

        let particles = Particles::new(&gl, skull.clone(), skull_texture.clone());
        particles.transform.translate(5.0, 0.0, 0.0);
        self.particles.push(particles);

        // Objects

        self.picked_object = self.objects.len() as isize;

        for i in 0..10 {
            self.objects.push(
                Object::new(bell.clone(), grass_texture.clone(), {
                    let t = Transform::from_xyz_hv(i as f32 - 6.0, -2.0, 0.0, 0.0, 0.0);
                    t.rotate_h(-1.57);
                    t
                })
                .with_skeleton(&skl),
            );
        }

        self.objects.push(Object::new(skull, skull_texture, {
            let t = Transform::from_xyz(0.0, 0.3, 0.0);
            t.rotate_v(1.2 * std::f32::consts::PI / 2.0);
            t.scale(0.1);
            t
        }));
        self.objects.push(Object::new(
            cube.clone(),
            grass_texture.clone(),
            Transform::from_xyz(5.0, -1.0, 5.0),
        ));
        // self.objects.push(Object::new(
        //     cube.clone(),
        //     grass_texture.clone(),
        //     Transform::from_xyz(0.0, -1.0, 0.0),
        // ));

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
        self.objects.push(Object::new(floor, carpet_texture, {
            let t = Transform::from_xyz(0.0, 4.0, -10.0);
            t.scale(5.0);
            t.rotate_v(std::f32::consts::PI);
            t
        }));
        self.objects.push(Object::new(
            cube,
            grass_texture,
            Transform::from_xyz(0.0, -1.0, -5.0),
        ));

        // === Lights ===

        let light = Light::new_directional(
            &gl,
            Transform::from_xyz_hv(0.0, 0.0, -5.0, std::f32::consts::PI * 1.0, -0.6),
        )
        .with_color(Vector3::from_xyz(0.0, 0.0, 1.0));
        self.lights.push(light);

        let light = Light::new_directional(
            &gl,
            Transform::from_xyz_hv(0.0, 3.0, 5.0, std::f32::consts::PI * 0.0, 0.6),
        )
        .with_color(Vector3::from_xyz(1.0, 1.0, 0.0));
        self.lights.push(light);

        let light = Light::new_point(&gl, Transform::from_xyz(0.0, 1.0, -3.0))
            .with_color(Vector3::from_xyz(0.8, 0.8, 0.3));
        self.lights.push(light);

        self.animation = Some(anim);
    }
}

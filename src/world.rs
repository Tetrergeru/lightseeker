use std::{collections::HashMap, rc::Rc};

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
    animations: HashMap<String, Rc<Animation>>,

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
            animations: HashMap::new(),

            lights: vec![],
            camera,
        }
    }

    pub fn required_resources(&self) -> Vec<ResourceRequest> {
        use ResourceRequest as RR;
        vec![
            RR::text("resources/skull.obj", "Skull"),
            RR::text("resources/gleb.obj", "Gleb"),
            RR::text("resources/Crate1.obj", "Cube"),
            RR::text("resources/floor.obj", "Floor"),
            RR::text("resources/walk.skl", "Walk.skl"),
            RR::text("resources/walk.skin", "Walk.skin"),
            RR::text("resources/walk.anim", "Walk.anim"),
            RR::text("resources/walk.obj", "Walk"),
            RR::text("resources/person.skl", "Person.skl"),
            RR::text("resources/person.skin", "Person.skin"),
            RR::text("resources/person.anim", "Person.anim"),
            RR::text("resources/person.obj", "Person"),
            RR::image("resources/skull.jpg", "Skull"),
            RR::image("resources/crate_1.jpg", "Grass"),
            RR::image("resources/carpet.jpg", "Carpet"),
        ]
    }

    pub fn tick(&mut self, delta_time: f32, controls: &Controls) {
        self.tick_controls(delta_time, controls);
        self.tick_animations(delta_time);
        self.tick_physics(delta_time);
    }

    fn tick_controls(&mut self, delta_time: f32, controls: &Controls) {
        use ControlKey::*;
        for key in controls.keys_down() {
            let player_speed = 3.0 * delta_time;
            match key {
                Forward => self.camera.move_h(Vector2::from_xy(0.0, player_speed)),
                Back => self.camera.move_h(Vector2::from_xy(0.0, -player_speed)),
                Right => self.camera.move_h(Vector2::from_xy(-player_speed, 0.0)),
                Left => self.camera.move_h(Vector2::from_xy(player_speed, 0.0)),
                Jump => self.camera.move_v(-player_speed),
                Crouch => self.camera.move_v(player_speed),
                Extra6 => self.bodies[0].collide(&self.bodies[1]),
                _ => (),
            }
        }
    }

    fn tick_animations(&mut self, delta_time: f32) {
        for obj in self.objects.iter_mut() {
            obj.tick_animation(delta_time);
        }
    }

    fn tick_physics(&mut self, _delta_time: f32) {
        for i in 0..self.bodies.len() {
            for j in (i + 1)..self.bodies.len() {
                self.bodies[i].collide(&self.bodies[j]);
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
            context.wire_light(
                body.frame_matrix(),
                self.camera.matrix(),
                Vector3::from_xyz(0.2, 1.0, 0.2),
            );
        }
    }

    pub fn init_0(&mut self, context: &GlContext) {
        let gl = context.gl();
        let skull = Rc::new(Shape::parse(&self.rm.get_text("Skull"), &gl));
        let cube = Rc::new(Shape::parse(&self.rm.get_text("Cube"), &gl));
        let floor = Rc::new(Shape::parse(&self.rm.get_text("Floor"), &gl));
        let gleb = Rc::new(Shape::parse(&self.rm.get_text("Gleb"), &gl));

        let table_skl = Rc::new(Skeleton::from_file(&self.rm.get_text("Walk.skl")));
        let table_skin = Rc::new(Skinning::parse(&self.rm.get_text("Walk.skin")));
        let table_anim = Rc::new(Animation::parse(&self.rm.get_text("Walk.anim")));
        let table = Rc::new(Shape::parse_with_skin(
            &self.rm.get_text("Walk"),
            &table_skin,
            &gl,
        ));
        self.animations.insert("Table".into(), table_anim.clone());

        let person_skl = Rc::new(Skeleton::from_file(&self.rm.get_text("Person.skl")));
        let person_skin = Rc::new(Skinning::parse(&self.rm.get_text("Person.skin")));
        let person_anim = Rc::new(Animation::parse(&self.rm.get_text("Person.anim")));
        let person = Rc::new(Shape::parse_with_skin(
            &self.rm.get_text("Person"),
            &person_skin,
            &gl,
        ));
        self.animations.insert("Person".into(), person_anim.clone());

        let grass_texture = self.rm.get_texture("Grass");
        let skull_texture = self.rm.get_texture("Skull");
        let carpet_texture = self.rm.get_texture("Carpet");

        // Rigid bodies

        let body = RigidBody::new(
            Vector3::from_xyz(1.0, 2.0, 1.0),
            Vector3::zero(),
            Transform::from_xyz(3.0, -0.5, -9.1),
        )
        .as_movable();
        self.bodies.push(body);
        let body = RigidBody::new(
            Vector3::from_xyz(1.0, 1.0, 1.0),
            Vector3::zero(),
            Transform::from_xyz(3.0, 0.85, -10.0),
        )
        .as_movable();
        self.bodies.push(body);

        // Particles

        let particles = Particles::new(&gl, skull.clone(), skull_texture.clone());
        particles.transform.translate(5.0, 0.0, 0.0);
        self.particles.push(particles);

        // Objects

        for i in 0..10 {
            self.objects.push(
                Object::new(table.clone(), grass_texture.clone(), {
                    let t = Transform::from_xyz_hv(i as f32 - 6.0, -2.0, 0.0, 0.0, 0.0);
                    t.rotate_h(-1.57);
                    t
                })
                .with_skeleton(&table_skl)
                .with_animation(table_anim.clone()),
            );
        }

        self.picked_object = self.objects.len() as isize;

        let person_object = Object::new(
            person,
            grass_texture.clone(),
            Transform::from_xyz(0.0, -2.0, -10.0),
        )
        .with_skeleton(&person_skl)
        .with_animation(person_anim);

        let bone_transform = person_object
            .get_bone_transform(person_skl.names["upfinger3.L"])
            .clone();

        self.objects.push(person_object);

        self.objects.push(Object::new(gleb, grass_texture.clone(), {
            let t = Transform::from_xyz(-5.0, -2.0, -5.0);
            t.scale(0.2);
            t
        }));

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

        self.objects
            .push(Object::new(floor.clone(), carpet_texture.clone(), {
                let t = Transform::from_xyz(0.0, 0.0, -15.0);
                t.scale(5.0);
                t.rotate_v(-std::f32::consts::PI / 2.0);
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

        let light = Light::new_directional(&gl, {
            let t = Transform::from_xyz(0.0, 0.0, 0.0);
            t.set_parent(bone_transform);
            t
        })
        .with_color(Vector3::from_xyz(1.0, 0.0, 0.0));
        self.lights.push(light);
    }
}

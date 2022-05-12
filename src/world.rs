use std::rc::Rc;

use crate::{
    camera::Camera,
    download::{ResourceManager, ResourceRequest},
    geometry::{Transform, Vector3},
    gl_context::GlContext,
    light::Light,
    objects::{
        object::Object,
        parsers::{animation::Animation, skeleton::Skeleton, skinning::Skinning},
        shape::Shape,
    },
};

pub struct World {
    rm: ResourceManager,

    objects: Vec<Object>,
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

        self.objects.push(
            Object::new(bell, grass_texture.clone(), {
                let t = Transform::from_xyz_hv(0.0, -2.0, 0.0, 0.0, 0.0);
                t.rotate_h(-1.57);
                t
            })
            .with_skeleton(&skl),
        );
        self.picked_object = self.objects.len() as isize - 1;

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

    pub fn move_picked(&mut self, d: Vector3) {
        if let Some(anim) = &self.animation {
            let delta = if d.z() > 0.0 { 1 } else { -1 };

            self.animation_frame = (self.animation_frame + delta + anim.frames.len() as isize)
                % anim.frames.len() as isize;

            log::debug!("App move_picked animation_frame = {}", self.animation_frame,);

            let picked = &self.picked_object();
            picked.set_pose(&anim.frames[self.animation_frame as usize])
        }
    }

    pub fn draw(&self, context: &GlContext) {
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
    }

    fn picked_object(&self) -> &Object {
        &self.objects[self.picked_object as usize]
    }
}

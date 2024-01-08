use serenity::{nalgebra_glm, petgraph, uuid, winit};

fn main() {
    serenity::app::App::new("Serenity", 1920, 1080).run(Game::default());
}

#[derive(Default)]
struct Game {
    player_node_index: petgraph::graph::NodeIndex<u32>,
}

impl serenity::app::State for Game {
    fn initialize(
        &mut self,
        context: &mut serenity::app::Context,
        renderer: &mut serenity::render::Renderer,
    ) {
        context.scene = serenity::gltf::import_gltf("resources/models/DamagedHelmet.glb").clone();
        let aspect_ratio = {
            let serenity::winit::dpi::PhysicalSize { width, height } = context.window.inner_size();
            width as f32 / height.max(1) as f32
        };
        let camera = crate::scene::Camera {
            projection: crate::scene::Projection::Perspective(crate::scene::PerspectiveCamera {
                aspect_ratio: Some(aspect_ratio),
                y_fov_rad: 90_f32.to_radians(),
                z_far: None,
                z_near: 0.01,
            }),
            orientation: Orientation {
                min_radius: 1.0,
                max_radius: 100.0,
                radius: 5.0,
                offset: nalgebra_glm::vec3(0.0, 0.0, 0.0),
                sensitivity: nalgebra_glm::vec2(1.0, 1.0),
                direction: nalgebra_glm::vec2(0_f32.to_radians(), 45_f32.to_radians()),
            },
        };

        renderer.view.import_scene(&context.scene, &renderer.gpu);

        context
            .scene
            .add_root_node(serenity::scene::create_camera_node(aspect_ratio));

        self.player_node_index = context.scene.add_root_node({
            serenity::scene::Node {
                id: uuid::Uuid::new_v4().to_string(),
                label: "Player".to_string(),
                transform: serenity::scene::Transform {
                    translation: nalgebra_glm::vec3(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                components: vec![serenity::scene::NodeComponent::Mesh("player".to_string())],
            }
        });
    }

    fn receive_event(
        &mut self,
        context: &mut serenity::app::Context,
        event: &serenity::winit::event::Event<()>,
    ) {
        if let winit::event::Event::WindowEvent {
            event:
                winit::event::WindowEvent::KeyboardInput {
                    input:
                        serenity::winit::event::KeyboardInput {
                            virtual_keycode: Some(keycode),
                            state,
                            ..
                        },
                    ..
                },
            ..
        } = *event
        {
            if let (winit::event::VirtualKeyCode::Escape, winit::event::ElementState::Pressed) =
                (keycode, state)
            {
                context.should_exit = true;
            }
        }
    }

    fn update(
        &mut self,
        context: &mut serenity::app::Context,
        _renderer: &mut serenity::render::Renderer,
    ) {
        if context.io.is_key_pressed(winit::event::VirtualKeyCode::W) {
            context.scene.graph[self.player_node_index]
                .transform
                .translation
                .x += 100.0;
        }
    }
}

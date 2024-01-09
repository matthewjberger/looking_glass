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
        // context.scene = serenity::gltf::import_gltf_resources("resources/models/DamagedHelmet.glb")
        //     .clone()
        //     .keys()
        //     .next()
        //     .unwrap();

        // let aspect_ratio = {
        //     let serenity::winit::dpi::PhysicalSize { width, height } = context.window.inner_size();
        //     width as f32 / height.max(1) as f32
        // };
        // renderer.view.import_scene(&context.scene, &renderer.gpu);

        // // context
        // //     .scene
        // //     .add_root_node(serenity::scene::create_camera_node(aspect_ratio));

        // // self.player_node_index = context.scene.add_root_node({
        // //     serenity::scene::Entity {
        // //         id: uuid::Uuid::new_v4().to_string(),
        // //         label: "Player".to_string(),
        // //         transform: serenity::scene::Transform {
        // //             translation: nalgebra_glm::vec3(0.0, 0.0, 0.0),
        // //             ..Default::default()
        // //         },
        // //         components: vec![serenity::scene::NodeComponent::Mesh("player".to_string())],
        // //     }
        // // });
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
            // context.scene.graph[self.player_node_index]
            //     .transform
            //     .translation
            //     .x += 100.0;
        }
    }
}

// pub fn create_camera_node(aspect_ratio: f32) -> Node {
//     crate::scene::Node {
//         id: uuid::Uuid::new_v4().to_string(),
//         label: "Main Camera".to_string(),
//         transform: crate::scene::Transform {
//             translation: nalgebra_glm::vec3(0.0, 0.0, 4.0),
//             ..Default::default()
//         },
//         components: vec![crate::scene::NodeComponent::Camera(crate::scene::Camera {
//             projection: crate::scene::Projection::Perspective(crate::scene::PerspectiveCamera {
//                 aspect_ratio: Some(aspect_ratio),
//                 y_fov_rad: 90_f32.to_radians(),
//                 z_far: None,
//                 z_near: 0.01,
//             }),
//             orientation: Orientation {
//                 min_radius: 1.0,
//                 max_radius: 100.0,
//                 radius: 5.0,
//                 offset: nalgebra_glm::vec3(0.0, 0.0, 0.0),
//                 sensitivity: nalgebra_glm::vec2(1.0, 1.0),
//                 direction: nalgebra_glm::vec2(0_f32.to_radians(), 45_f32.to_radians()),
//             },
//         })],
//     }
// }

// impl SceneGraph {
//     pub fn global_transform(&self, node_index: petgraph::graph::NodeIndex) -> nalgebra_glm::Mat4 {
//         match self
//             .0
//             .neighbors_directed(node_index, petgraph::Direction::Incoming)
//             .next()
//         {
//             Some(parent_node_index) => self.global_transform(parent_node_index) * transform,
//             None => transform,
//         }
//     }
// }

// impl Scene {
//     pub fn add_root_node(&mut self, node: crate::scene::Entity) -> petgraph::graph::NodeIndex {
//         let child = self.graph.add_node(node);
//         self.graph
//             .add_edge(petgraph::graph::NodeIndex::new(0), child, ());
//         child
//     }

//     pub fn walk_dfs(&self, mut visit_node: impl FnMut(&Entity, petgraph::graph::NodeIndex)) {
//         if self.graph.0.node_count() == 0 {
//             return;
//         }
//         let mut dfs = petgraph::visit::Dfs::new(&self.graph.0, petgraph::graph::NodeIndex::new(0));
//         while let Some(node_index) = dfs.next(&self.graph.0) {
//             visit_node(&self.graph.0[node_index], node_index);
//         }
//     }

//     pub fn walk_dfs_mut(
//         &mut self,
//         mut visit_node: impl FnMut(&mut Entity, petgraph::graph::NodeIndex),
//     ) {
//         if self.graph.0.node_count() == 0 {
//             return;
//         }
//         let mut dfs = petgraph::visit::Dfs::new(&self.graph.0, petgraph::graph::NodeIndex::new(0));
//         while let Some(node_index) = dfs.next(&self.graph.0) {
//             visit_node(&mut self.graph.0[node_index], node_index);
//         }
//     }

//     pub fn flatten_geometry(
//         &self,
//     ) -> (
//         Vec<crate::scene::Vertex>,
//         Vec<u16>,
//         std::collections::HashMap<String, Vec<PrimitiveDrawCommand>>,
//     ) {
//         let (vertices, indices, meshes) =
//             (Vec::new(), Vec::new(), std::collections::HashMap::new());

//         // TODO flatten geometry here
//         self.walk_dfs(|_node, _| {
//             // for component in node.components.iter() {
//             //     if let crate::scene::NodeComponent::Mesh(mesh_id) = component {
//             //         let commands = self.meshes[mesh_id]
//             //             .primitives
//             //             .iter()
//             //             .map(|primitive| {
//             //                 let primitive_vertices = primitive.vertices.to_vec();
//             //                 let vertex_offset = vertices.len();
//             //                 let number_of_vertices = primitive.vertices.len();
//             //                 vertices.extend_from_slice(&primitive_vertices);

//             //                 let primitive_indices = primitive
//             //                     .indices
//             //                     .iter()
//             //                     .map(|x| *x as u16)
//             //                     .collect::<Vec<_>>();
//             //                 let index_offset = indices.len();
//             //                 let number_of_indices = primitive.indices.len();
//             //                 indices.extend_from_slice(&primitive_indices);

//             //                 PrimitiveDrawCommand {
//             //                     vertex_offset,
//             //                     index_offset,
//             //                     vertices: number_of_vertices,
//             //                     indices: number_of_indices,
//             //                 }
//             //             })
//             //             .collect::<Vec<_>>();
//             //         meshes.insert(mesh_id.clone(), commands);
//             //     }
//             // }
//         });

//         (vertices, indices, meshes)
//     }
// }

pub fn import_gltf_resources(path: impl AsRef<std::path::Path>) -> crate::resource::ResourceMap {
    let (gltf, buffers, raw_images) = gltf::import(path.as_ref()).expect("Failed to import gltf");

    let mut resource_map = crate::resource::ResourceMap::default();

    let samplers = gltf
        .samplers()
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    gltf.samplers()
        .zip(&samplers)
        .for_each(|(sampler, sampler_id)| {
            resource_map.insert(
                sampler_id.to_string(),
                crate::resource::Resource::Sampler(sampler.into()),
            );
        });

    let images = gltf
        .images()
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    raw_images
        .into_iter()
        .zip(&images)
        .for_each(|(image, image_id)| {
            resource_map.insert(
                image_id.to_string(),
                crate::resource::Resource::Image(image.into()),
            );
        });

    let textures = gltf
        .textures()
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    gltf.textures()
        .zip(&textures)
        .for_each(|(texture, texture_id)| {
            resource_map.insert(
                texture_id.to_string(),
                crate::resource::Resource::Texture(crate::resource::Texture {
                    sampler: match texture.sampler().index() {
                        Some(sampler_index) => samplers[sampler_index].to_string(),
                        None => samplers[0].to_string(),
                    },
                    image: images[texture.source().index()].to_string(),
                }),
            );
        });

    let materials = gltf
        .materials()
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    gltf.materials()
        .into_iter()
        .zip(&materials)
        .for_each(|(material, material_id)| {
            resource_map.insert(
                material_id.to_string(),
                crate::resource::Resource::Material(crate::resource::Material {
                    base_color_factor: material.pbr_metallic_roughness().base_color_factor().into(),
                    base_color_texture: match material.pbr_metallic_roughness().base_color_texture()
                    {
                        Some(texture) => textures[texture.texture().index()].to_string(),
                        None => textures[0].to_string(),
                    },
                }),
            );
        });

    let cameras = gltf
        .cameras()
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    gltf.cameras()
        .zip(&cameras)
        .for_each(|(camera, camera_id)| {
            resource_map.insert(
                camera_id.to_string(),
                crate::resource::Resource::Camera(camera.into()),
            );
        });

    let meshes = gltf
        .meshes()
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    gltf.meshes().zip(&meshes).for_each(|(mesh, mesh_id)| {
        resource_map.insert(
            mesh_id.to_string(),
            crate::resource::Resource::Mesh(crate::resource::Mesh {
                primitives: mesh
                    .primitives()
                    .map(|primitive| {
                        let material = match primitive.material().index() {
                            Some(index) => materials[index].to_string(),
                            None => "default".to_string(),
                        };
                        crate::resource::Primitive {
                            mode: primitive.mode().into(),
                            material,
                            vertices: {
                                let reader = primitive.reader(|buffer| Some(&*buffers[buffer.index()]));

                                let mut positions = Vec::new();
                                let read_positions = reader
                                    .read_positions()
                                    .expect("Failed to read gltf vertex positions");
                                read_positions.for_each(|position| {
                                    positions.push(nalgebra_glm::Vec3::from(position));
                                });
                                let number_of_vertices = positions.len();
                                let normals = reader.read_normals().map_or(
                                    vec![nalgebra_glm::vec3(0.0, 0.0, 0.0); number_of_vertices],
                                    |normals| normals.map(nalgebra_glm::Vec3::from).collect::<Vec<_>>(),
                                );
                                let map_to_vec2 = |coords: gltf::mesh::util::ReadTexCoords| -> Vec<nalgebra_glm::Vec2> {
                                    coords
                                        .into_f32()
                                        .map(nalgebra_glm::Vec2::from)
                                        .collect::<Vec<_>>()
                                };
                                let uv_0 = reader.read_tex_coords(0).map_or(
                                    vec![nalgebra_glm::vec2(0.0, 0.0); number_of_vertices],
                                    map_to_vec2,
                                );
                                let uv_1 = reader.read_tex_coords(1).map_or(
                                    vec![nalgebra_glm::vec2(0.0, 0.0); number_of_vertices],
                                    map_to_vec2,
                                );
                                let convert_joints = |joints: gltf::mesh::util::ReadJoints| -> Vec<nalgebra_glm::Vec4> {
                                    joints
                                        .into_u16()
                                        .map(|joint| {
                                            nalgebra_glm::vec4(joint[0] as _, joint[1] as _, joint[2] as _, joint[3] as _)
                                        })
                                        .collect::<Vec<_>>()
                                };
                                let joints_0 = reader.read_joints(0).map_or(
                                    vec![nalgebra_glm::vec4(0.0, 0.0, 0.0, 0.0); number_of_vertices],
                                    convert_joints,
                                );
                                let convert_weights = |weights: gltf::mesh::util::ReadWeights| -> Vec<nalgebra_glm::Vec4> {
                                    weights.into_f32().map(nalgebra_glm::Vec4::from).collect()
                                };
                                let weights_0 = reader.read_weights(0).map_or(
                                    vec![nalgebra_glm::vec4(1.0, 0.0, 0.0, 0.0); number_of_vertices],
                                    convert_weights,
                                );
                                let convert_colors = |colors: gltf::mesh::util::ReadColors| -> Vec<nalgebra_glm::Vec3> {
                                    colors
                                        .into_rgb_f32()
                                        .map(nalgebra_glm::Vec3::from)
                                        .collect::<Vec<_>>()
                                };
                                let colors_0 = reader.read_colors(0).map_or(
                                    vec![nalgebra_glm::vec3(1.0, 1.0, 1.0); number_of_vertices],
                                    convert_colors,
                                );

                                // every vertex is guaranteed to have a position attribute,
                                // so we can use the position attribute array to index into the other attribute arrays

                                positions
                                    .into_iter()
                                    .enumerate()
                                    .map(|(index, position)| crate::resource::Vertex {
                                        position,
                                        normal: normals[index],
                                        uv_0: uv_0[index],
                                        uv_1: uv_1[index],
                                        joint_0: joints_0[index],
                                        weight_0: weights_0[index],
                                        color_0: colors_0[index],
                                    })
                                    .collect()
                            },
                            indices: primitive
                                .reader(|buffer| Some(&*buffers[buffer.index()]))
                                .read_indices()
                                .take()
                                .map(|read_indices| read_indices.into_u32().collect())
                                .unwrap_or_default(),
                        }
                    })
                    .collect(),
            }),
        );
    });

    let light_ids = gltf
        .lights()
        .map(|lights| lights.collect::<Vec<_>>())
        .unwrap_or_default()
        .iter()
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    let lights = gltf
        .lights()
        .map(|lights| lights.collect::<Vec<_>>())
        .unwrap_or_default();
    lights
        .into_iter()
        .zip(&light_ids)
        .for_each(|(light, light_id)| {
            resource_map.insert(
                light_id.to_string(),
                crate::resource::Resource::Light(light.into()),
            );
        });

    let nodes = gltf
        .nodes()
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    gltf.nodes().zip(&nodes).for_each(|(node, node_id)| {
        let mesh = node.mesh().map(|mesh| meshes[mesh.index()].to_string());
        let camera = node
            .camera()
            .map(|camera| cameras[camera.index()].to_string());
        let light = node
            .light()
            .map(|light| light_ids[light.index()].to_string());
        resource_map.insert(
            node_id.to_string(),
            crate::resource::Resource::Node(crate::resource::Node {
                transform: crate::resource::Transform::from(node.transform().decomposed()),
                resources: {
                    let mut resources = Vec::new();
                    if let Some(mesh) = mesh {
                        resources.push(mesh);
                    }
                    if let Some(camera) = camera {
                        resources.push(camera);
                    }
                    if let Some(light) = light {
                        resources.push(light);
                    }
                    resources
                },
            }),
        );
    });

    let skins = gltf
        .skins()
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    gltf.skins().zip(&skins).for_each(|(skin, skin_id)| {
        let reader = skin.reader(|buffer| Some(&buffers[buffer.index()]));
        let inverse_bind_matrices = reader
            .read_inverse_bind_matrices()
            .map_or(Vec::new(), |matrices| {
                matrices.map(nalgebra_glm::Mat4::from).collect::<Vec<_>>()
            });
        resource_map.insert(
            skin_id.to_string(),
            crate::resource::Resource::Skin(crate::resource::Skin {
                joints: skin
                    .joints()
                    .enumerate()
                    .map(|(index, joint_node)| {
                        let inverse_bind_matrix = *inverse_bind_matrices
                            .get(index)
                            .unwrap_or(&nalgebra_glm::Mat4::identity());
                        crate::resource::Joint {
                            inverse_bind_matrix,
                            target: nodes[joint_node.index()].to_string(),
                        }
                    })
                    .collect(),
            }),
        );
    });

    let animations = gltf
        .animations()
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    gltf.animations()
        .zip(&animations)
        .for_each(|(animation, animation_id)| {
            let channels = animation
                .channels()
                .map(|channel| {
                    let target_node = channel.target().node().index();
                    let target = nodes[target_node].to_string();
                    let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));
                    let inputs = reader
                        .read_inputs()
                        .expect("Failed to read animation channel inputs!")
                        .collect::<Vec<_>>();
                    let outputs = reader
                        .read_outputs()
                        .expect("Failed to read animation channel outputs!");
                    let transformations = match outputs {
                        gltf::animation::util::ReadOutputs::Translations(translations) => {
                            let translations = translations
                                .map(nalgebra_glm::Vec3::from)
                                .collect::<Vec<_>>();
                            crate::resource::TransformationSet::Translations(translations)
                        }
                        gltf::animation::util::ReadOutputs::Rotations(rotations) => {
                            let rotations = rotations
                                .into_f32()
                                .map(nalgebra_glm::Vec4::from)
                                .collect::<Vec<_>>();
                            crate::resource::TransformationSet::Rotations(rotations)
                        }
                        gltf::animation::util::ReadOutputs::Scales(scales) => {
                            let scales = scales.map(nalgebra_glm::Vec3::from).collect::<Vec<_>>();
                            crate::resource::TransformationSet::Scales(scales)
                        }
                        gltf::animation::util::ReadOutputs::MorphTargetWeights(weights) => {
                            let morph_target_weights = weights.into_f32().collect::<Vec<_>>();
                            crate::resource::TransformationSet::MorphTargetWeights(
                                morph_target_weights,
                            )
                        }
                    };
                    crate::resource::Channel {
                        target,
                        inputs,
                        transformations,
                        interpolation: crate::resource::Interpolation::default(),
                    }
                })
                .collect::<Vec<_>>();

            resource_map.insert(
                animation_id.to_string(),
                crate::resource::Resource::Animation(crate::resource::Animation {
                    time: 0.0,
                    channels: animation
                        .channels()
                        .map(|channel| {
                            let target_node = channel.target().node().index();
                            let target = nodes[target_node].to_string();
                            let reader = channel.reader(|buffer| Some(&buffers[buffer.index()]));
                            let inputs = reader
                                .read_inputs()
                                .expect("Failed to read animation channel inputs!")
                                .collect::<Vec<_>>();
                            let outputs = reader
                                .read_outputs()
                                .expect("Failed to read animation channel outputs!");
                            let transformations = match outputs {
                                gltf::animation::util::ReadOutputs::Translations(translations) => {
                                    let translations = translations
                                        .map(nalgebra_glm::Vec3::from)
                                        .collect::<Vec<_>>();
                                    crate::resource::TransformationSet::Translations(translations)
                                }
                                gltf::animation::util::ReadOutputs::Rotations(rotations) => {
                                    let rotations = rotations
                                        .into_f32()
                                        .map(nalgebra_glm::Vec4::from)
                                        .collect::<Vec<_>>();
                                    crate::resource::TransformationSet::Rotations(rotations)
                                }
                                gltf::animation::util::ReadOutputs::Scales(scales) => {
                                    let scales =
                                        scales.map(nalgebra_glm::Vec3::from).collect::<Vec<_>>();
                                    crate::resource::TransformationSet::Scales(scales)
                                }
                                gltf::animation::util::ReadOutputs::MorphTargetWeights(weights) => {
                                    let morph_target_weights =
                                        weights.into_f32().collect::<Vec<_>>();
                                    crate::resource::TransformationSet::MorphTargetWeights(
                                        morph_target_weights,
                                    )
                                }
                            };
                            crate::resource::Channel {
                                target,
                                inputs,
                                transformations,
                                interpolation: crate::resource::Interpolation::default(),
                            }
                        })
                        .collect::<Vec<_>>(),
                    max_animation_time: channels
                        .iter()
                        .flat_map(|channel| channel.inputs.iter().copied())
                        .fold(0.0, f32::max),
                }),
            );
        });

    let scenegraphs = gltf
        .scenes()
        .map(|_| uuid::Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    gltf.scenes()
        .zip(&scenegraphs)
        .for_each(|(scene, scenegraph_id)| {
            let mut scenegraph = crate::resource::EntityGraph::default();
            scene.nodes().for_each(|node| {
                let root_node = nodes[node.index()].to_string();
                let root_node_index = scenegraph.add_node(root_node);
                fn import_node(
                    parent: petgraph::graph::NodeIndex,
                    node: gltf::Node,
                    graph: &mut crate::resource::EntityGraph,
                    nodes: &[String],
                ) {
                    let entity = nodes[node.index()].to_string();
                    let index = graph.add_node(entity);
                    if parent != index {
                        graph.add_edge(parent, index, ());
                    }
                    node.children().for_each(|child_index| {
                        import_node(index, child_index, graph, nodes);
                    });
                }
                import_node(root_node_index, node, &mut scenegraph, &nodes);
            });
            resource_map.insert(
                scenegraph_id.to_string(),
                crate::resource::Resource::Graph(scenegraph),
            );
        });

    resource_map
}

impl From<gltf::texture::Sampler<'_>> for crate::resource::Sampler {
    fn from(sampler: gltf::texture::Sampler<'_>) -> Self {
        let min_filter = sampler
            .min_filter()
            .map(|filter| match filter {
                gltf::texture::MinFilter::Linear
                | gltf::texture::MinFilter::LinearMipmapLinear
                | gltf::texture::MinFilter::LinearMipmapNearest => crate::resource::Filter::Linear,
                gltf::texture::MinFilter::Nearest
                | gltf::texture::MinFilter::NearestMipmapLinear
                | gltf::texture::MinFilter::NearestMipmapNearest => {
                    crate::resource::Filter::Nearest
                }
            })
            .unwrap_or_default();

        let mag_filter = sampler
            .mag_filter()
            .map(|filter| match filter {
                gltf::texture::MagFilter::Linear => crate::resource::Filter::Linear,
                gltf::texture::MagFilter::Nearest => crate::resource::Filter::Nearest,
            })
            .unwrap_or_default();

        let wrap_s = match sampler.wrap_s() {
            gltf::texture::WrappingMode::ClampToEdge => crate::resource::WrappingMode::ClampToEdge,
            gltf::texture::WrappingMode::MirroredRepeat => {
                crate::resource::WrappingMode::MirroredRepeat
            }
            gltf::texture::WrappingMode::Repeat => crate::resource::WrappingMode::Repeat,
        };

        let wrap_t = match sampler.wrap_t() {
            gltf::texture::WrappingMode::ClampToEdge => crate::resource::WrappingMode::ClampToEdge,
            gltf::texture::WrappingMode::MirroredRepeat => {
                crate::resource::WrappingMode::MirroredRepeat
            }
            gltf::texture::WrappingMode::Repeat => crate::resource::WrappingMode::Repeat,
        };

        Self {
            min_filter,
            mag_filter,
            wrap_s,
            wrap_t,
        }
    }
}

impl From<gltf::material::AlphaMode> for crate::resource::AlphaMode {
    fn from(mode: gltf::material::AlphaMode) -> Self {
        match mode {
            gltf::material::AlphaMode::Opaque => crate::resource::AlphaMode::Opaque,
            gltf::material::AlphaMode::Mask => crate::resource::AlphaMode::Mask,
            gltf::material::AlphaMode::Blend => crate::resource::AlphaMode::Blend,
        }
    }
}

impl From<gltf::image::Data> for crate::resource::Image {
    fn from(data: gltf::image::Data) -> Self {
        Self {
            pixels: data.pixels.to_vec(),
            format: data.format.into(),
            width: data.width,
            height: data.height,
        }
    }
}

impl From<gltf::image::Format> for crate::resource::ImageFormat {
    fn from(value: gltf::image::Format) -> Self {
        match value {
            gltf::image::Format::R8 => crate::resource::ImageFormat::R8,
            gltf::image::Format::R8G8 => crate::resource::ImageFormat::R8G8,
            gltf::image::Format::R8G8B8 => crate::resource::ImageFormat::R8G8B8,
            gltf::image::Format::R8G8B8A8 => crate::resource::ImageFormat::R8G8B8A8,
            gltf::image::Format::R16 => crate::resource::ImageFormat::R16,
            gltf::image::Format::R16G16 => crate::resource::ImageFormat::R16G16,
            gltf::image::Format::R16G16B16 => crate::resource::ImageFormat::R16G16B16,
            gltf::image::Format::R16G16B16A16 => crate::resource::ImageFormat::R16G16B16A16,
            gltf::image::Format::R32G32B32FLOAT => crate::resource::ImageFormat::R32G32B32,
            gltf::image::Format::R32G32B32A32FLOAT => crate::resource::ImageFormat::R32G32B32A32,
        }
    }
}

impl From<gltf::Camera<'_>> for crate::resource::Camera {
    fn from(camera: gltf::Camera) -> Self {
        Self {
            projection: match camera.projection() {
                gltf::camera::Projection::Perspective(camera) => {
                    crate::resource::Projection::Perspective(crate::resource::PerspectiveCamera {
                        aspect_ratio: camera.aspect_ratio(),
                        y_fov_rad: camera.yfov(),
                        z_far: camera.zfar(),
                        z_near: camera.znear(),
                    })
                }
                gltf::camera::Projection::Orthographic(camera) => {
                    crate::resource::Projection::Orthographic(crate::resource::OrthographicCamera {
                        x_mag: camera.xmag(),
                        y_mag: camera.ymag(),
                        z_far: camera.zfar(),
                        z_near: camera.znear(),
                    })
                }
            },
            orientation: crate::resource::Orientation::default(),
        }
    }
}

impl From<gltf::khr_lights_punctual::Light<'_>> for crate::resource::Light {
    fn from(light: gltf::khr_lights_punctual::Light) -> Self {
        Self {
            color: light.color().into(),
            intensity: light.intensity(),
            range: light.range().unwrap_or(0.0),
            kind: light.kind().into(),
        }
    }
}

impl From<gltf::khr_lights_punctual::Kind> for crate::resource::LightKind {
    fn from(kind: gltf::khr_lights_punctual::Kind) -> Self {
        match kind {
            gltf::khr_lights_punctual::Kind::Directional => crate::resource::LightKind::Directional,
            gltf::khr_lights_punctual::Kind::Point => crate::resource::LightKind::Point,
            gltf::khr_lights_punctual::Kind::Spot {
                inner_cone_angle,
                outer_cone_angle,
            } => crate::resource::LightKind::Spot {
                inner_cone_angle,
                outer_cone_angle,
            },
        }
    }
}

impl From<gltf::mesh::Mode> for crate::resource::PrimitiveMode {
    fn from(mode: gltf::mesh::Mode) -> Self {
        match mode {
            gltf::mesh::Mode::Points => crate::resource::PrimitiveMode::Points,
            gltf::mesh::Mode::Lines => crate::resource::PrimitiveMode::Lines,
            gltf::mesh::Mode::LineLoop => crate::resource::PrimitiveMode::LineLoop,
            gltf::mesh::Mode::LineStrip => crate::resource::PrimitiveMode::LineStrip,
            gltf::mesh::Mode::Triangles => crate::resource::PrimitiveMode::Triangles,
            gltf::mesh::Mode::TriangleStrip => crate::resource::PrimitiveMode::TriangleStrip,
            gltf::mesh::Mode::TriangleFan => crate::resource::PrimitiveMode::TriangleFan,
        }
    }
}

#[cfg(test)]
mod tests {
    #[ignore]
    #[test]
    fn import() {
        let _resources = crate::gltf::import_gltf_resources("resources/models/DamagedHelmet.glb");
    }
}

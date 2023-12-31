pub(crate) struct App {
    event_loop: winit::event_loop::EventLoop<()>,
    window: winit::window::Window,
    gpu: crate::gpu::Gpu,
    gui: crate::gui::Gui,
    scene: crate::render::SceneRender,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = winit::event_loop::EventLoop::new();
        let window = winit::window::WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .with_transparent(true)
            .build(&event_loop)
            .expect("Failed to create winit window!");

        let gpu = crate::gpu::Gpu::new(&window, width, height);
        let gui = crate::gui::Gui::new(&window, &gpu);
        let scene = crate::render::SceneRender::new(&gpu);

        Self {
            event_loop,
            window,
            gpu,
            gui,
            scene,
        }
    }

    pub fn run(self) {
        let Self {
            event_loop,
            window,
            mut gui,
            mut scene,
            mut gpu,
        } = self;

        event_loop.run(move |event, _, control_flow| {
            if gui.consumed_event(&event, &window) {
                return;
            }

            match event {
                winit::event::Event::MainEventsCleared => {
                    scene.render(&window, &gpu, &mut gui);
                }

                winit::event::Event::WindowEvent { event, window_id }
                    if window_id == window.id() =>
                {
                    Self::route_window_event(event, control_flow, &mut gpu, &mut scene);
                }

                _ => {}
            }
        });
    }

    fn route_window_event(
        event: winit::event::WindowEvent,
        control_flow: &mut winit::event_loop::ControlFlow,
        gpu: &mut crate::gpu::Gpu,
        scene: &mut crate::render::SceneRender,
    ) {
        match event {
            winit::event::WindowEvent::CloseRequested => {
                *control_flow = winit::event_loop::ControlFlow::Exit
            }

            winit::event::WindowEvent::KeyboardInput { input, .. } => {
                if let (
                    Some(winit::event::VirtualKeyCode::Escape),
                    winit::event::ElementState::Pressed,
                ) = (input.virtual_keycode, input.state)
                {
                    *control_flow = winit::event_loop::ControlFlow::Exit;
                }

                if let Some(_keycode) = input.virtual_keycode.as_ref() {
                    // Handle a key press
                }
            }

            winit::event::WindowEvent::MouseInput {
                button: _button,
                state: _state,
                ..
            } => {
                // Handle a mouse button press
            }

            winit::event::WindowEvent::Resized(winit::dpi::PhysicalSize { width, height }) => {
                gpu.resize(width, height);
                scene.resize(&gpu.device, width, height);
            }
            _ => {}
        }
    }
}

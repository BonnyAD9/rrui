use std::sync::Arc;

use minlin::{MapExt, Vec2};
use smol_str::SmolStr;
use winit::{
    application::ApplicationHandler,
    error::{EventLoopError, OsError},
    event::{
        ElementState, KeyEvent, Modifiers, MouseButton, MouseScrollDelta,
        WindowEvent,
    },
    event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy},
    keyboard::ModifiersKeyState,
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    event::{Event, EventType, ScrollDelta},
    AppCtrl, AppState, Application, RenderState,
};

impl Event for WindowEvent {
    fn get_type(&self) -> EventType {
        match self {
            Self::Resized(s) => {
                EventType::Resize(Vec2::new(s.width, s.height))
            }
            Self::CloseRequested => EventType::CloseRequest,
            Self::Focused(f) => EventType::Focus(*f),
            Self::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => EventType::KeyPress,
            Self::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Released,
                        ..
                    },
                ..
            } => EventType::KeyRelease,
            Self::ModifiersChanged(m) => {
                EventType::ModifiersChange((*m).into())
            }
            Self::CursorMoved { position, .. } => {
                EventType::MouseMove(Vec2::new(position.x, position.y).cast())
            }
            Self::MouseWheel { delta, .. } => {
                EventType::MouseScroll((*delta).into())
            }
            Self::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => EventType::MousePress((*button).into()),
            Self::MouseInput {
                state: ElementState::Released,
                button,
                ..
            } => EventType::MouseRelease((*button).into()),
            Self::ScaleFactorChanged { scale_factor, .. } => {
                EventType::ScaleFactorChange(*scale_factor as f32)
            }
            Self::RedrawRequested => EventType::RedrawRequest,
            _ => EventType::Other,
        }
    }

    fn key_char(&self) -> Option<SmolStr> {
        match self {
            Self::KeyboardInput {
                event: KeyEvent { text, .. },
                ..
            } => (*text).clone(),
            _ => None,
        }
    }

    fn is_window(&self) -> bool {
        matches!(
            self,
            Self::Resized(_)
                | Self::Moved(_)
                | Self::CloseRequested
                | Self::Destroyed
                | Self::Focused(_)
                | Self::CursorEntered { .. }
                | Self::CursorLeft { .. }
                | Self::ScaleFactorChanged { .. }
                | Self::ThemeChanged(_)
                | Self::Occluded(_)
                | Self::RedrawRequested
        )
    }

    fn is_keyboard(&self) -> bool {
        matches!(self, Self::KeyboardInput { .. } | Self::ModifiersChanged(_))
    }

    fn is_mouse(&self) -> bool {
        matches!(
            self,
            Self::DroppedFile(_)
                | Self::HoveredFile(_)
                | Self::HoveredFileCancelled
                | Self::CursorMoved { .. }
                | Self::CursorEntered { .. }
                | Self::CursorLeft { .. }
                | Self::MouseWheel { .. }
                | Self::MouseInput { .. }
        )
    }

    fn is_input(&self) -> bool {
        matches!(
            self,
            Self::CloseRequested
                | Self::KeyboardInput { .. }
                | Self::ModifiersChanged(_)
                | Self::Ime(_)
                | Self::CursorMoved { .. }
                | Self::MouseWheel { .. }
                | Self::MouseInput { .. }
                | Self::PinchGesture { .. }
                | Self::PanGesture { .. }
                | Self::DoubleTapGesture { .. }
                | Self::RotationGesture { .. }
                | Self::TouchpadPressure { .. }
                | Self::AxisMotion { .. }
                | Self::Touch(..)
        )
    }
}

impl From<Modifiers> for crate::event::Modifiers {
    fn from(value: winit::event::Modifiers) -> Self {
        let mut res = Self::from_bits_truncate(value.state().bits());

        if value.lshift_state() == ModifiersKeyState::Pressed {
            res |= Self::LSHIFT;
        }
        if value.rshift_state() == ModifiersKeyState::Pressed {
            res |= Self::RSHIFT;
        }
        if value.lcontrol_state() == ModifiersKeyState::Pressed {
            res |= Self::LCONTROL;
        }
        if value.rcontrol_state() == ModifiersKeyState::Pressed {
            res |= Self::RCONTROL;
        }
        if value.lalt_state() == ModifiersKeyState::Pressed {
            res |= Self::LALT;
        }
        if value.ralt_state() == ModifiersKeyState::Pressed {
            res |= Self::RALT;
        }
        if value.lsuper_state() == ModifiersKeyState::Pressed {
            res |= Self::LSUPER;
        }
        if value.rsuper_state() == ModifiersKeyState::Pressed {
            res |= Self::RSUPER;
        }

        res
    }
}

impl From<MouseButton> for crate::event::MouseButton {
    fn from(value: winit::event::MouseButton) -> Self {
        match value {
            MouseButton::Left => Self::Left,
            MouseButton::Right => Self::Right,
            MouseButton::Middle => Self::Middle,
            MouseButton::Back => Self::Back,
            MouseButton::Forward => Self::Forward,
            MouseButton::Other(n) => Self::Other(n),
        }
    }
}

impl From<MouseScrollDelta> for ScrollDelta {
    fn from(value: MouseScrollDelta) -> Self {
        match value {
            MouseScrollDelta::LineDelta(x, y) => Self::Lines(Vec2::new(x, y)),
            MouseScrollDelta::PixelDelta(o) => {
                Self::Pixels(Vec2::new(o.x, o.y).cast())
            }
        }
    }
}

impl crate::wgpu::Window for Arc<Window> {
    fn inner_size(&self) -> Vec2<u32> {
        let s = winit::window::Window::inner_size(self);
        Vec2::new(s.width, s.height)
    }

    fn get_target(&self) -> wgpu::SurfaceTarget<'static> {
        self.clone().into()
    }

    fn request_redraw(&self) {
        winit::window::Window::request_redraw(self);
    }
}

impl crate::Window for Arc<Window> {
    type Config = WindowAttributes;

    fn size(&self) -> Vec2<u32> {
        let s = self.inner_size();
        Vec2::new(s.width, s.height)
    }
}

impl AppCtrl for &ActiveEventLoop {
    type Window = Arc<Window>;
    type Error = OsError;

    fn exit(self) {
        self.exit();
    }

    fn create_window(
        self,
        cfg: <Self::Window as crate::Window>::Config,
    ) -> Result<Self::Window, Self::Error> {
        self.create_window(cfg).map(Arc::new)
    }
}

impl<App, Rend, RendState> ApplicationHandler<App::Message>
    for AppState<App, Rend, RendState, WindowEvent, Arc<Window>>
where
    App: Application<Rend, WindowEvent>,
    Rend: crate::Renderer,
    RendState: RenderState<Arc<Window>, Rend> + 'static,
{
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.init(event_loop);
    }

    fn user_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        event: App::Message,
    ) {
        self.message(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        self.event(event, event_loop);
    }
}

impl<App, Rend, RendState>
    crate::EventLoop<
        App::Message,
        AppState<App, Rend, RendState, WindowEvent, Arc<Window>>,
    > for EventLoop<App::Message>
where
    App: Application<Rend, WindowEvent>,
    Rend: crate::Renderer,
    RendState: RenderState<Arc<Window>, Rend> + 'static,
{
    type Event = WindowEvent;
    type Proxy = EventLoopProxy<App::Message>;
    type Window = Arc<Window>;
    type Error = EventLoopError;

    fn create() -> Result<Self, Self::Error> {
        Self::with_user_event().build()
    }

    fn run(
        self,
        app: &mut AppState<App, Rend, RendState, WindowEvent, Arc<Window>>,
    ) -> Result<(), Self::Error> {
        self.run_app(app)
    }
}

impl<M: Clone + Send + Sync> crate::EventLoopProxy<M> for EventLoopProxy<M> {
    fn send(&self, m: M) {
        _ = self.send_event(m);
    }
}

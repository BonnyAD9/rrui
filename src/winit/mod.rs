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
    keyboard::{KeyCode, ModifiersKeyState, PhysicalKey},
    window::{Window, WindowAttributes, WindowId},
};

use crate::{
    AppCtrl, AppState, Application, RenderState,
    event::{Event, EventKind, ScrollDelta},
};

impl Event for WindowEvent {
    fn get_kind(&self) -> EventKind {
        match self {
            Self::Resized(s) => {
                EventKind::Resize(Vec2::new(s.width, s.height))
            }
            Self::CloseRequested => EventKind::CloseRequest,
            Self::Focused(f) => EventKind::WindowFocus(*f),
            Self::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key,
                        ..
                    },
                ..
            } => EventKind::KeyPress((*physical_key).into()),
            Self::KeyboardInput {
                event:
                    KeyEvent {
                        state: ElementState::Released,
                        physical_key,
                        ..
                    },
                ..
            } => EventKind::KeyRelease((*physical_key).into()),
            Self::ModifiersChanged(m) => {
                EventKind::ModifiersChange((*m).into())
            }
            Self::CursorMoved { position, .. } => {
                EventKind::MouseMove(Vec2::new(position.x, position.y).cast())
            }
            Self::MouseWheel { delta, .. } => {
                EventKind::MouseScroll((*delta).into())
            }
            Self::MouseInput {
                state: ElementState::Pressed,
                button,
                ..
            } => EventKind::MousePress((*button).into()),
            Self::MouseInput {
                state: ElementState::Released,
                button,
                ..
            } => EventKind::MouseRelease((*button).into()),
            Self::ScaleFactorChanged { scale_factor, .. } => {
                EventKind::ScaleFactorChange(*scale_factor as f32)
            }
            Self::RedrawRequested => EventKind::RedrawRequest,
            Self::CursorLeft { .. } => EventKind::MouseLeaveWindow,
            Self::CursorEntered { .. } => EventKind::MouseEnterWindow,
            _ => EventKind::Other,
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

    fn get_flags(&self) -> crate::event::EventFlags {
        match self {
            Self::Resized(_)
            | Self::Focused(_)
            | Self::ScaleFactorChanged { .. }
            | Self::RedrawRequested => crate::event::EventFlags::WINDOW,
            Self::Moved(_)
            | Self::Destroyed
            | Self::ThemeChanged(_)
            | Self::Occluded(_) => {
                crate::event::EventFlags::WINDOW
                    | crate::event::EventFlags::OTHER
            }
            Self::CloseRequested => {
                crate::event::EventFlags::INPUT
                    | crate::event::EventFlags::WINDOW
            }
            Self::DroppedFile(_)
            | Self::HoveredFile(_)
            | Self::HoveredFileCancelled => {
                crate::event::EventFlags::WINDOW
                    | crate::event::EventFlags::MOUSE
                    | crate::event::EventFlags::OTHER
                    | crate::event::EventFlags::FOR_WIDGETS
            }
            Self::KeyboardInput { .. } | Self::ModifiersChanged(_) => {
                crate::event::EventFlags::KEYBOARD
                    | crate::event::EventFlags::INPUT
                    | crate::event::EventFlags::FOR_WIDGETS
            }
            Self::Ime(_)
            | Self::PinchGesture { .. }
            | Self::PanGesture { .. }
            | Self::DoubleTapGesture { .. }
            | Self::RotationGesture { .. }
            | Self::TouchpadPressure { .. }
            | Self::AxisMotion { .. }
            | Self::Touch { .. } => {
                crate::event::EventFlags::INPUT
                    | crate::event::EventFlags::OTHER
                    | crate::event::EventFlags::FOR_WIDGETS
            }
            Self::CursorMoved { .. }
            | Self::MouseWheel { .. }
            | Self::MouseInput { .. } => {
                crate::event::EventFlags::MOUSE
                    | crate::event::EventFlags::INPUT
                    | crate::event::EventFlags::FOR_WIDGETS
            }
            Self::CursorEntered { .. } => {
                crate::event::EventFlags::MOUSE
                    | crate::event::EventFlags::WINDOW
            }
            Self::CursorLeft { .. } => {
                crate::event::EventFlags::MOUSE
                    | crate::event::EventFlags::WINDOW
                    | crate::event::EventFlags::FOR_WIDGETS
            }
            _ => crate::event::EventFlags::OTHER,
        }
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

impl From<KeyCode> for crate::event::KeyCode {
    fn from(value: KeyCode) -> Self {
        match value {
            KeyCode::Backquote => Self::Backquote,
            KeyCode::Backslash => Self::Backslash,
            KeyCode::BracketLeft => Self::BracketLeft,
            KeyCode::BracketRight => Self::BracketRight,
            KeyCode::Comma => Self::Comma,
            KeyCode::Digit0 => Self::Digit0,
            KeyCode::Digit1 => Self::Digit1,
            KeyCode::Digit2 => Self::Digit2,
            KeyCode::Digit3 => Self::Digit3,
            KeyCode::Digit4 => Self::Digit4,
            KeyCode::Digit5 => Self::Digit5,
            KeyCode::Digit6 => Self::Digit6,
            KeyCode::Digit7 => Self::Digit7,
            KeyCode::Digit8 => Self::Digit8,
            KeyCode::Digit9 => Self::Digit9,
            KeyCode::Equal => Self::Equal,
            KeyCode::IntlBackslash => Self::IntlBackslash,
            KeyCode::IntlRo => Self::IntlRo,
            KeyCode::IntlYen => Self::IntlYen,
            KeyCode::KeyA => Self::KeyA,
            KeyCode::KeyB => Self::KeyB,
            KeyCode::KeyC => Self::KeyC,
            KeyCode::KeyD => Self::KeyD,
            KeyCode::KeyE => Self::KeyE,
            KeyCode::KeyF => Self::KeyF,
            KeyCode::KeyG => Self::KeyG,
            KeyCode::KeyH => Self::KeyH,
            KeyCode::KeyI => Self::KeyI,
            KeyCode::KeyJ => Self::KeyJ,
            KeyCode::KeyK => Self::KeyK,
            KeyCode::KeyL => Self::KeyL,
            KeyCode::KeyM => Self::KeyM,
            KeyCode::KeyN => Self::KeyN,
            KeyCode::KeyO => Self::KeyO,
            KeyCode::KeyP => Self::KeyP,
            KeyCode::KeyQ => Self::KeyQ,
            KeyCode::KeyR => Self::KeyR,
            KeyCode::KeyS => Self::KeyS,
            KeyCode::KeyT => Self::KeyT,
            KeyCode::KeyU => Self::KeyU,
            KeyCode::KeyV => Self::KeyV,
            KeyCode::KeyW => Self::KeyW,
            KeyCode::KeyX => Self::KeyX,
            KeyCode::KeyY => Self::KeyY,
            KeyCode::KeyZ => Self::KeyZ,
            KeyCode::Minus => Self::Minus,
            KeyCode::Period => Self::Period,
            KeyCode::Quote => Self::Quote,
            KeyCode::Semicolon => Self::Semicolon,
            KeyCode::Slash => Self::Slash,
            KeyCode::AltLeft => Self::AltLeft,
            KeyCode::AltRight => Self::AltRight,
            KeyCode::Backspace => Self::Backspace,
            KeyCode::CapsLock => Self::CapsLock,
            KeyCode::ContextMenu => Self::ContextMenu,
            KeyCode::ControlLeft => Self::ControlLeft,
            KeyCode::ControlRight => Self::ControlRight,
            KeyCode::Enter => Self::Enter,
            KeyCode::SuperLeft => Self::SuperLeft,
            KeyCode::SuperRight => Self::SuperRight,
            KeyCode::ShiftLeft => Self::ShiftLeft,
            KeyCode::ShiftRight => Self::ShiftRight,
            KeyCode::Space => Self::Space,
            KeyCode::Tab => Self::Tab,
            KeyCode::Convert => Self::Convert,
            KeyCode::KanaMode => Self::KanaMode,
            KeyCode::Lang1 => Self::Lang1,
            KeyCode::Lang2 => Self::Lang2,
            KeyCode::Lang3 => Self::Lang3,
            KeyCode::Lang4 => Self::Lang4,
            KeyCode::Lang5 => Self::Lang5,
            KeyCode::NonConvert => Self::NonConvert,
            KeyCode::Delete => Self::Delete,
            KeyCode::End => Self::End,
            KeyCode::Help => Self::Help,
            KeyCode::Home => Self::Home,
            KeyCode::Insert => Self::Insert,
            KeyCode::PageDown => Self::PageDown,
            KeyCode::PageUp => Self::PageUp,
            KeyCode::ArrowDown => Self::ArrowDown,
            KeyCode::ArrowLeft => Self::ArrowLeft,
            KeyCode::ArrowRight => Self::ArrowRight,
            KeyCode::ArrowUp => Self::ArrowUp,
            KeyCode::NumLock => Self::NumLock,
            KeyCode::Numpad0 => Self::Numpad0,
            KeyCode::Numpad1 => Self::Numpad1,
            KeyCode::Numpad2 => Self::Numpad2,
            KeyCode::Numpad3 => Self::Numpad3,
            KeyCode::Numpad4 => Self::Numpad4,
            KeyCode::Numpad5 => Self::Numpad5,
            KeyCode::Numpad6 => Self::Numpad6,
            KeyCode::Numpad7 => Self::Numpad7,
            KeyCode::Numpad8 => Self::Numpad8,
            KeyCode::Numpad9 => Self::Numpad9,
            KeyCode::NumpadAdd => Self::NumpadAdd,
            KeyCode::NumpadBackspace => Self::NumpadBackspace,
            KeyCode::NumpadClear => Self::NumpadClear,
            KeyCode::NumpadClearEntry => Self::NumpadClearEntry,
            KeyCode::NumpadComma => Self::NumpadComma,
            KeyCode::NumpadDecimal => Self::NumpadDecimal,
            KeyCode::NumpadDivide => Self::NumpadDivide,
            KeyCode::NumpadEnter => Self::NumpadEnter,
            KeyCode::NumpadEqual => Self::NumpadEqual,
            KeyCode::NumpadHash => Self::NumpadHash,
            KeyCode::NumpadMemoryAdd => Self::NumpadMemoryAdd,
            KeyCode::NumpadMemoryClear => Self::NumpadMemoryClear,
            KeyCode::NumpadMemoryRecall => Self::NumpadMemoryRecall,
            KeyCode::NumpadMemoryStore => Self::NumpadMemoryStore,
            KeyCode::NumpadMemorySubtract => Self::NumpadMemorySubtract,
            KeyCode::NumpadMultiply => Self::NumpadMultiply,
            KeyCode::NumpadParenLeft => Self::NumpadParenLeft,
            KeyCode::NumpadParenRight => Self::NumpadParenRight,
            KeyCode::NumpadStar => Self::NumpadStar,
            KeyCode::NumpadSubtract => Self::NumpadSubtract,
            KeyCode::Escape => Self::Escape,
            KeyCode::Fn => Self::Fn,
            KeyCode::FnLock => Self::FnLock,
            KeyCode::PrintScreen => Self::PrintScreen,
            KeyCode::ScrollLock => Self::ScrollLock,
            KeyCode::Pause => Self::Pause,
            KeyCode::BrowserBack => Self::BrowserBack,
            KeyCode::BrowserFavorites => Self::BrowserFavorites,
            KeyCode::BrowserForward => Self::BrowserForward,
            KeyCode::BrowserHome => Self::BrowserHome,
            KeyCode::BrowserRefresh => Self::BrowserRefresh,
            KeyCode::BrowserSearch => Self::BrowserSearch,
            KeyCode::BrowserStop => Self::BrowserStop,
            KeyCode::Eject => Self::Eject,
            KeyCode::LaunchApp1 => Self::LaunchApp1,
            KeyCode::LaunchApp2 => Self::LaunchApp2,
            KeyCode::LaunchMail => Self::LaunchMail,
            KeyCode::MediaPlayPause => Self::MediaPlayPause,
            KeyCode::MediaSelect => Self::MediaSelect,
            KeyCode::MediaStop => Self::MediaStop,
            KeyCode::MediaTrackNext => Self::MediaTrackNext,
            KeyCode::MediaTrackPrevious => Self::MediaTrackPrevious,
            KeyCode::Power => Self::Power,
            KeyCode::Sleep => Self::Sleep,
            KeyCode::AudioVolumeDown => Self::AudioVolumeDown,
            KeyCode::AudioVolumeMute => Self::AudioVolumeMute,
            KeyCode::AudioVolumeUp => Self::AudioVolumeUp,
            KeyCode::WakeUp => Self::WakeUp,
            KeyCode::Meta => Self::Meta,
            KeyCode::Hyper => Self::Hyper,
            KeyCode::Turbo => Self::Turbo,
            KeyCode::Abort => Self::Abort,
            KeyCode::Resume => Self::Resume,
            KeyCode::Suspend => Self::Suspend,
            KeyCode::Again => Self::Again,
            KeyCode::Copy => Self::Copy,
            KeyCode::Cut => Self::Cut,
            KeyCode::Find => Self::Find,
            KeyCode::Open => Self::Open,
            KeyCode::Paste => Self::Paste,
            KeyCode::Props => Self::Props,
            KeyCode::Select => Self::Select,
            KeyCode::Undo => Self::Undo,
            KeyCode::Hiragana => Self::Hiragana,
            KeyCode::Katakana => Self::Katakana,
            KeyCode::F1 => Self::F1,
            KeyCode::F2 => Self::F2,
            KeyCode::F3 => Self::F3,
            KeyCode::F4 => Self::F4,
            KeyCode::F5 => Self::F5,
            KeyCode::F6 => Self::F6,
            KeyCode::F7 => Self::F7,
            KeyCode::F8 => Self::F8,
            KeyCode::F9 => Self::F9,
            KeyCode::F10 => Self::F10,
            KeyCode::F11 => Self::F11,
            KeyCode::F12 => Self::F12,
            KeyCode::F13 => Self::F13,
            KeyCode::F14 => Self::F14,
            KeyCode::F15 => Self::F15,
            KeyCode::F16 => Self::F16,
            KeyCode::F17 => Self::F17,
            KeyCode::F18 => Self::F18,
            KeyCode::F19 => Self::F19,
            KeyCode::F20 => Self::F20,
            KeyCode::F21 => Self::F21,
            KeyCode::F22 => Self::F22,
            KeyCode::F23 => Self::F23,
            KeyCode::F24 => Self::F24,
            KeyCode::F25 => Self::F25,
            KeyCode::F26 => Self::F26,
            KeyCode::F27 => Self::F27,
            KeyCode::F28 => Self::F28,
            KeyCode::F29 => Self::F29,
            KeyCode::F30 => Self::F30,
            KeyCode::F31 => Self::F31,
            KeyCode::F32 => Self::F32,
            KeyCode::F33 => Self::F33,
            KeyCode::F34 => Self::F34,
            KeyCode::F35 => Self::F35,
            _ => Self::Unknown,
        }
    }
}

impl From<PhysicalKey> for crate::event::KeyCode {
    fn from(value: PhysicalKey) -> Self {
        match value {
            PhysicalKey::Code(key_code) => key_code.into(),
            PhysicalKey::Unidentified(_) => Self::Unknown,
        }
    }
}

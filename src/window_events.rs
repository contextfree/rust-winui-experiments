// The code in this file is derived from the winit project ( https://github.com/rust-windowing/winit ), 
// created by the winit contributors including Pierre Krieger and Francesca Plebani. 
// It has been extensively modified to remove most functionality not needed by the present project.
// winit is licensed under Apache License 2.0 which can be found in this project as "LICENSE_winit"

/// Describes an event from a `Window`.
#[derive(Clone, Debug, PartialEq)]
pub enum WindowEvent {
    /// The size of the window has changed. Contains the client area's new dimensions.
    Resized(LogicalSize),
    /// The position of the window has changed. Contains the window's new position.
    Moved(LogicalPosition),
    /// The window has been requested to close.
    CloseRequested,
    /// The window has been destroyed.
    Destroyed,
    /// A file has been dropped into the window.
    /// 
    /// When the user drops multiple files at once, this event will be emitted for each file
    /// separately.
    DroppedFile(PathBuf),
    /// A file is being hovered over the window.
    /// 
    /// When the user hovers multiple files at once, this event will be emitted for each file
    /// separately.
    HoveredFile(PathBuf),
    /// A file was hovered, but has exited the window.
    /// 
    /// There will be a single `HoveredFileCancelled` event triggered even if multiple files were
    /// hovered.
    HoveredFileCancelled,
    /// The window received a unicode character.
    ReceivedCharacter(char),
    /// The window gained or lost focus.
    ///
    /// The parameter is true if the window has gained focus, and false if it has lost focus.
    Focused(bool),
    /// An event from the keyboard has been received.
    KeyboardInput { device_id: DeviceId, input: KeyboardInput },
    /// The cursor has moved on the window.
    CursorMoved {
        device_id: DeviceId,
        /// (x,y) coords in pixels relative to the top-left corner of the window. Because the range of this data is
        /// limited by the display area and it may have been transformed by the OS to implement effects such as cursor
        /// acceleration, it should not be used to implement non-cursor-like interactions such as 3D camera control.
        position: LogicalPosition,
        modifiers: ModifiersState
    },
    /// The cursor has entered the window.
    CursorEntered { device_id: DeviceId },
    /// The cursor has left the window.
    CursorLeft { device_id: DeviceId },
    /// A mouse wheel movement or touchpad scroll occurred.
    MouseWheel { device_id: DeviceId, delta: MouseScrollDelta, phase: TouchPhase, modifiers: ModifiersState },
    /// An mouse button press has been received.
    MouseInput { device_id: DeviceId, state: ElementState, button: MouseButton, modifiers: ModifiersState },
    /// Touchpad pressure event.
    ///
    /// At the moment, only supported on Apple forcetouch-capable macbooks.
    /// The parameters are: pressure level (value between 0 and 1 representing how hard the touchpad
    /// is being pressed) and stage (integer representing the click level).
    TouchpadPressure { device_id: DeviceId, pressure: f32, stage: i64 },
    /// Motion on some analog axis. May report data redundant to other, more specific events.
    AxisMotion { device_id: DeviceId, axis: AxisId, value: f64 },
    /// The window needs to be redrawn.
    Refresh,
    /// Touch event has been received
    Touch(Touch),
    /// The DPI factor of the window has changed.
    ///
    /// The following user actions can cause DPI changes:
    ///
    /// * Changing the display's resolution.
    /// * Changing the display's DPI factor (e.g. in Control Panel on Windows).
    /// * Moving the window to a display with a different DPI factor.
    ///
    /// For more information about DPI in general, see the [`dpi`](dpi/index.html) module.
    HiDpiFactorChanged(f64),
}

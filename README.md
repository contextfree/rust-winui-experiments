# rust-winui-experiments
Rust + Windows Composition and WinUI experiments and samples

Currently just contains a basic sample that uses Windows.UI.Composition to draw some squares in a Win32 window.

* Requires Rust nightly

* Currently depends on the experimental `combase-macro` branch of `winrt-rust`: https://github.com/contextfree/winrt-rust/tree/combase-macro . That repository needs to be cloned into a sibling directory to the one `rust-winui-experiments` is cloned into (so they have the same parent directory, e.g. `/repos/winrt-rust` and `/repos/rust-winui-experiments`) 

Some of this code (specifically the `window.rs` and `window_events.rs` files) is derived from the winit project 
( https://github.com/rust-windowing/winit ), created by the winit contributors including Pierre Krieger and Francesca Plebani. 
It has been extensively modified to remove most functionality not needed by the present project.

winit is licensed under Apache License 2.0 which can be found in this project as "LICENSE_winit"

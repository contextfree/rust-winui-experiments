# rust-winui-experiments
Rust + Windows Composition and WinUI experiments and samples

Currently just contains a basic sample that uses Windows.UI.Composition to draw some squares in a Win32 window.

* Requires Rust nightly

* Currently depends on the experimental `combase-macro` branch of `winrt-rust`: https://github.com/contextfree/winrt-rust/tree/combase-macro . That repository needs to be cloned into a sibling directory to the one `rust-winui-experiments` is cloned into (so they have the same parent directory, e.g. `/repos/winrt-rust` and `/repos/rust-winui-experiments`) 

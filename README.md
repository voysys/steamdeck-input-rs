# Steam Deck input

Reads Steam Deck buttons and axes through HIDAPI's `linux-static-hidraw` backend.
Building requires libudev headers and pkg-config. The running process must have
access to the Deck's gamepad hidraw node. Enable only one HIDAPI Linux backend
across the dependency graph.

`SteamdeckInput::try_new()` starts a background worker and returns an error if the
thread cannot be created. Create the reader when input is needed and keep it
owned by the input service. Dropping it stops and joins the worker, closing the
device. Process-global statics are not dropped at exit.

`fetch()` returns the latest axes and retains button presses between fetches,
so a press followed by release between calls is still observable. `peek()` reads
the same state without consuming retained presses. Both return `None` before
input becomes available or when the latest valid report is at least 100 ms old.
Unavailable state is cleared so old presses cannot carry across a reporting gap.

Discovery selects one distinct `28de:1205` device path with interface 2, usage
page `ffff`, and usage 1. Multiple matching paths are rejected. After validating
a Deck input report, the reader clears digital mappings and maps pad presses to
mouse buttons. Configuration runs once per opened device; input is published
starting with the next valid report.

Timeouts, unrelated reports, and malformed reports keep the device open. Device
errors clear input and trigger another discovery attempt after a delay, increasing
from 100 ms to a maximum of 2 seconds. Shutdown interrupts this delay. The hidraw
backend leaves the USB kernel driver bound. Avoid concurrent controller
configuration by other readers, including SDL's Steam Deck backend.

Button values are 0 (released) or 1 (pressed), in this order:

| Indices | Controls |
| --- | --- |
| 0–3 | A, B, X, Y |
| 4–5 | Left bumper, right bumper |
| 6–8 | View, Menu, Quick Access |
| 9–10 | Left stick press, right stick press |
| 11–14 | D-pad up, right, down, left |
| 15–18 | R4, R5, L4, L5 |
| 19–21 | Steam, left pad press, right pad press |

Axes 0–5 are left X/Y, right X/Y, and left/right triggers. Values range from
-1 to +1; triggers are -1 at rest and +1 when fully pressed.

Run `cargo test --locked` and `cargo clippy --locked -- -D warnings` for parser,
button/axis, retention, timeout/disconnect, and shutdown tests.

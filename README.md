# Steam Deck input

The Linux reader uses HIDAPI's `linux-static-hidraw` backend. Build with libudev
headers and pkg-config; runtime access to the Deck's gamepad hidraw node is
required. Do not enable another HIDAPI Linux backend through feature unification.

Create a reader with `SteamdeckInput::try_new()` only when input is requested.
Keep it owned by the input service and drop it on shutdown: drop stops, wakes,
and joins the worker, closing hidraw. Process-global statics are not dropped at
exit. `fetch()` consumes retained short presses; `peek()` checks availability
without consuming them. Both return `None` until fresh state has arrived and
after state has been stale for 100 ms.

Discovery selects one deduplicated `28de:1205` path with interface 2, usage page
`ffff`, usage 1. A valid Deck state is required before sending the existing
clear-digital-mappings and two-pad-click mapping sequence. The sequence runs once
per connection. No settings/default resets or USB driver detachment are used.
Timeouts and unrelated reports do not reopen the device. Actual errors clear
state and retry with bounded, interruptible backoff. Do not configure the same
controller concurrently through another reader (including SDL's Deck backend).

Button indices 0–14 retain the original gamepad layout, with Quick Access at 8.
Indices 15–21 remain R4, R5, L4, L5, Steam, left pad press, right pad press.
Axes remain left X/Y, right X/Y, left/right triggers, with trigger rest at -1.

An isolated hidraw experiment inside OdenVR's existing Flatpak permissions on
SteamOS 3.8.16, kernel `6.16.12-valve24.5-1-neptune-616-gb2f7cfe85e45`, verified
concurrent raw gamepad reports and operator-confirmed desktop input before and
after the mapping sequence with Steam stopped. That experiment did not exercise
this complete worker implementation. Integrated hardware testing, Steam-client
coexistence, suspend/resume, and full application compatibility remain pending.

Run `cargo test --locked` and `cargo clippy --locked -- -D warnings` for parser,
button/axis, retention, timeout/disconnect, and shutdown tests.

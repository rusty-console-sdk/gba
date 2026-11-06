# Input

`gba_sdk::input` reads the ten buttons: A, B, Select, Start, the four directions, L and R.

```rust
use gba_sdk::input::{Button, Buttons};

let buttons = Buttons::read();
if buttons.held(Button::A) {
    // jump
}
if buttons.any() {
    // skip the title screen
}
```

`Buttons::read` reads `KEYINPUT` once and returns the state at that moment. The hardware register has a bit clear while a button is held; `Buttons` flips that, so a set bit means held and the type is a plain set of what is down. Read it once per frame and keep the value, so every check in the frame sees the same state.

Edge detection (pressed this frame, released this frame) is not in yet; keep the previous frame's `Buttons` and compare.

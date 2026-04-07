# rrui

Retained Rust User Interface. RRUI is rust crate user interface with retained
widget tree. Many rust user interface crates use immidate or mixed widget tree,
but that can be inefficient and it is sometimes less convinient. Because of
this I decided to make my own crate with retained widget tree.

Note that the project is in very early stage. See [examples](examples) for
example usage.

## Backend

The implementation is agnostic of the backend used for rendering. The
implemented backend uses [winit](https://github.com/rust-windowing/winit) for
windowing, [wgpu](https://github.com/gfx-rs/wgpu) as graphics api and
[iced_wgpu](https://github.com/iced-rs/iced/tree/master/wgpu) as renderer.

## Widgets

These are the currently implemented widgets:
- `Button`: Clickable container.
- `Container`: Simple container with relative and absolute padding for its
  child.
- `Debug`: Debbugging widget. Draws border and background of the given child
  widget. Useful when debugging layouting. Doesn't modify the child behaviour
  in any way.
- `Grid`: Layouting widget showing children in absolute or relative sized rows
  and columns (basically 2D `Layout`) and allows children to overlap with given
  z index.
- `Layout`: Layouting widget aligning childern horizontaly or verticaly with
  absolute or relative sizes and the given spacing. (basically 1D `Grid` +
  spacing)
- `Margin`: Simple layouting container that adds absolute padding to its child.
- `TextBlock`: Widget for displaying text.
- `Image`: Widget showing image.
- `Nothing`: The simplest widget. Does nothing, takes no space.
- `Rectangle`: Colored rectangle with border.
- `Stack`: Layouting widget that stacks its children with spacing in one of
  four directions.

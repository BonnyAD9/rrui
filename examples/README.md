# Examples

This folder contains examples for rrui:
- [hello_world.rs](hello_world.rs): Window with text in the center.
    - Showcases how to create the simplest rrui application.
- [counter.rs](counter.rs): Button tied to counter. Showcases:
    - how to modify some properties of widgets
    - how to use basic widgets
- [fonts.rs](fonts.rs): Buttons to change font of text. Showcases:
    - how to use the widget `Variable` to modify child widgets
    - how set font of a text
    - layouting with `Container`, `Layout` and `Stack`
- [fonts2.rs](fonts2.rs): Simmilar to `fonts.rs` but uses `Grid` with spanning
  text instead of `Layout` and `Stack`.
- [image.rs](image.rs): Image inside window with different filling options.
  Showcases:
    - how to use image
    - how to use grid with overlap and spanning
- [svg.rs](svg.rs): Same as image but with svg.
- [no_rrui.rs](no_rrui.rs): This is not really rrui example. It shows how to
  create window and draw to it without using rrui. It served as reference for
  implementing the backend.

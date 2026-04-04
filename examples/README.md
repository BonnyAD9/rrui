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
    - layouting with `Container`, `Grid` and `Stack`
- [no_rrui.rs](no_rrui.rs): This is not really rrui example. It shows how to
  create window and draw to it without using rrui. It served as reference for
  implementing the backend.

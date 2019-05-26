# TRGUI
Approach for a GUI Toolkit on Rust. Based on Widget trait (Dynamic Dispatch) and Containers that can handle Widget trait objects. Inspired by IMGUIs, MLIB (AzPainter) and Love2D

## What TRGUI has?
* Widget trait that has draw, update functions and can handle mouse and keyboard events using States.
* Containers for trait objects that implement Widget trait.
* Layouts and Decorators for Containers
* Bindings for share external data to widgets.
* Groups for communication between widgets based on IDs.

## What TRGUI doesn't have?
* Renderer
* Window Handling
* Main Loop
* Default Widgets

You need implement those by yourself or from other crates.

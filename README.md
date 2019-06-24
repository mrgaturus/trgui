# trgui: minimalist and independent gui crate
Inspired by IMGUIs, MLIB (AzPainter) and Love2D

![diagram](https://raw.githubusercontent.com/mrgaturus/trgui/master/diagram.png)

## What trgui has?
* Generic Widget trait for easy integration with rendering contexts and point types
* Containers for dispatch function calls to Widget trait objects.
* Focus, Grab, Hover handling.
* Layouts and Decorators for Containers.
* RefProxy for share external data to widgets. (this is unsafe, You can use `Rc<RefCell<T>>` instead)
* Groups for communication between widgets based on IDs.

## Goals of trgui
* Extremely flexible with other crates like path renderers, window managers, etc.
* Very fast performance with very low memory and CPU usage.
* Easy to understand and maintain
* No dependencies

## Limitations
* You aren't able to remove or modify widgets directly from Containers after moving it. Its better 
create a SoA widget than a widget for each data.
* Focus can't be modified by update and hover_out

## What trgui doesn't have?
* Renderer
* Window Handling
* Main Loop
* Default Widgets

You need implement those by yourself or from other crates.

## Examples

* Calculator with sdl2: https://www.youtube.com/watch?v=ybhRPkA7wtI

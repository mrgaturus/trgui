use crate::decorator::Decorator;
use crate::layout::Layout;
use crate::state::{KeyState, MouseState};
use crate::widget::flags::*;
use crate::widget::{Boundaries, Dimensions, Flags, Widget, WidgetInternal};
use crate::binding::{BindID, BindType};
use crate::Boxed;

use std::ops::{Add, Sub};

type WidgetList<P, D> = Vec<Box<dyn Widget<P, D>>>;
pub type InternalList<P, D> = Vec<WidgetInternal<P, D>>;

pub struct Container<P, D> {
    widgets_i: InternalList<P, D>,
    widgets: WidgetList<P, D>,
    decorator: Box<dyn Decorator<P, D>>,
    layout: Box<dyn Layout<P, D>>,
    focus_id: Option<usize>,
    grab_id: Option<usize>,
    hover_id: Option<usize>,
}

impl<P: Sized + Copy + Clone, D: Sized + Copy + Clone> Container<P, D>
where
    D: PartialOrd + Default,
    P: Add<Output = P> + Sub<Output = P> + PartialOrd + From<D> + Default,
{
    pub fn new(decorator: Box<dyn Decorator<P, D>>, layout: Box<dyn Layout<P, D>>) -> Self {
        Container {
            widgets_i: InternalList::new(),
            widgets: WidgetList::new(),
            focus_id: Option::None,
            grab_id: Option::None,
            hover_id: Option::None,
            decorator,
            layout,
        }
    }

    pub fn set_layout(&mut self, layout: Box<dyn Layout<P, D>>) {
        self.layout = layout;
    }

    pub fn set_decorator(&mut self, decorator: Box<dyn Decorator<P, D>>) {
        self.decorator = decorator;
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget<P, D>>, flags: Flags, bind: BindType) {
        let mut internal = WidgetInternal::new(flags, bind);
        internal.off(FOCUS | GRAB | HOVER);
        internal.set_min_dimensions(widget.compute_min());

        self.widgets_i.push(internal);
        self.widgets.push(widget);
    }

    pub fn add_widget_b(
        &mut self,
        widget: Box<dyn Widget<P, D>>,
        bounds: Boundaries<P, D>,
        flags: Flags,
        bind: BindType,
    ) {
        let mut internal =
            WidgetInternal::new_with((bounds.0, bounds.1), (bounds.2, bounds.3), flags, bind);
        internal.off(FOCUS | GRAB | HOVER);
        internal.set_min_dimensions(widget.compute_min());

        self.widgets_i.push(internal);
        self.widgets.push(widget);
    }

    fn step(&mut self, back: bool) -> (bool, usize) {
        match self.focus_id {
            Some(mut id) => {
                let len = self.widgets.len() - 1;

                if back {
                    if id == 0 {
                        self.focus_id = None;
                        return (false, len);
                    }
                    id -= 1;
                } else {
                    if id == len {
                        self.focus_id = None;
                        return (false, 0);
                    }
                    id += 1;
                }

                self.focus_id = Some(id);
                (true, id)
            }
            None => {
                let mut val = 0;
                if back {
                    val = self.widgets.len() - 1;
                }
                self.focus_id = Some(val);

                (true, val)
            }
        }
    }
}

impl<P: Sized + Copy + Clone, D: Sized + Copy + Clone> Widget<P, D> for Container<P, D>
where
    D: PartialOrd + Default,
    P: Add<Output = P> + Sub<Output = P> + PartialOrd + From<D> + Default,
{
    fn draw(&mut self, internal: &WidgetInternal<P, D>) -> bool {
        let count: usize;

        self.decorator.before(internal);

        count = self
            .widgets_i
            .iter_mut()
            .zip(self.widgets.iter_mut())
            .filter(|(w_internal, _)| w_internal.check(DRAW))
            .fold(0, |_, (w_internal, widget)| {
                let draw = widget.draw(w_internal);
                if !draw {
                    w_internal.off(DRAW);
                }

                draw as usize
            });

        self.decorator.after(internal);

        count > 0
    }

    fn update(&mut self, internal: &mut WidgetInternal<P, D>) {
        let count: usize;

        count = self
            .widgets_i
            .iter_mut()
            .zip(self.widgets.iter_mut())
            .filter(|(w_internal, _)| w_internal.check(UPDATE))
            .fold(0, |_, (w_internal, widget)| {
                let backup = w_internal.val(FOCUS | GRAB | HOVER);
                widget.update(w_internal);

                if w_internal.changed() {
                    internal.on(w_internal.val(DRAW));

                    w_internal.on(backup);
                    w_internal.unchange();
                }

                w_internal.check(UPDATE) as usize
            });

        if let Some(id) = self.focus_id {
            if !self.widgets_i[id].check(ENABLED | VISIBLE) {
                self.unfocus(internal);
            }
        }

        internal.set(UPDATE, count > 0);
    }

    fn bind(&mut self, internal: &mut WidgetInternal<P, D>, bind: BindID) {
        self.widgets_i
            .iter_mut()
            .zip(self.widgets.iter_mut())
            .filter(|(w_internal, _)| w_internal.check_bind(bind))
            .for_each(|(w_internal, widget)| {
                let backup = w_internal.val(FOCUS | GRAB | HOVER);
                widget.bind(w_internal, bind);

                if w_internal.changed() {
                    internal.on(w_internal.val(DRAW));

                    w_internal.on(backup);
                    w_internal.unchange();
                }
            });

        if let Some(id) = self.focus_id {
            if !self.widgets_i[id].check(ENABLED | VISIBLE) {
                self.unfocus(internal);
            }
        }
    }

    fn layout(&mut self, internal: &mut WidgetInternal<P, D>) {
        self.layout
            .layout(&mut self.widgets_i, &internal.dimensions());

        if let Some(id) = self.focus_id {
            let widget_id = &mut self.widgets_i[id];
            if !widget_id.check(VISIBLE | ENABLED) {
                self.unfocus(internal);
            }
        }

        self.widgets_i
            .iter_mut()
            .zip(self.widgets.iter_mut())
            .for_each(|(w_internal, widget)| {
                w_internal.compute_absolute(internal.absolute_pos());
                widget.layout(w_internal);

                if w_internal.changed() {
                    internal.on(w_internal.val(DRAW | UPDATE));
                }
            });

        self.decorator.update(internal);
    }

    fn handle_mouse(&mut self, internal: &mut WidgetInternal<P, D>, mouse: &MouseState<P>) {
        if self.grab_id.is_some() || !internal.check(GRAB) {
            let widget_n = self
                .grab_id
                .or(self
                    .hover_id
                    .filter(|&n| self.widgets_i[n].on_area(mouse.absolute_pos())))
                .or_else(|| {
                    self.widgets_i
                        .iter()
                        .enumerate()
                        .find_map(|(n, w_internal)| {
                            if w_internal.on_area(mouse.absolute_pos()) {
                                Some(n)
                            } else {
                                None
                            }
                        })
                });

            if let Some(n) = widget_n {
                let w_internal = unsafe {
                    &mut *(self.widgets_i.get_unchecked_mut(n) as *mut WidgetInternal<P, D>)
                };

                if w_internal.check(GRAB) {
                    w_internal.set(HOVER, w_internal.on_area(mouse.absolute_pos()))
                } else {
                    if let Some(id) = self.hover_id {
                        if id != n {
                            self.unhover(internal);
                        }
                    }
                    self.hover_id = Some(n);
                    w_internal.on(HOVER);
                }
                w_internal.unchange();

                self.widgets[n].handle_mouse(w_internal, mouse);

                if w_internal.changed() {
                    internal.on(w_internal.val(DRAW | UPDATE));

                    internal.set(GRAB, w_internal.check(GRAB));
                    self.grab_id = Some(n).filter(|_| w_internal.check(GRAB));

                    if w_internal.check(FOCUS) {
                        if let Some(id) = self.focus_id {
                            if id != n {
                                self.unfocus(internal);
                            }
                        }

                        self.focus_id = Some(n);
                        internal.on(FOCUS);
                    }
                }
            } else {
                self.unhover(internal);

                if self.grab_id.is_none() {
                    internal.set(GRAB, mouse.clicked());
                }
            }
        } else {
            internal.set(GRAB, mouse.clicked());
        }
    }

    fn handle_keys(&mut self, internal: &mut WidgetInternal<P, D>, key: &KeyState) {
        if let Some(id) = self.focus_id {
            let w_internal = &mut self.widgets_i[id];

            self.widgets[id].handle_keys(w_internal, key);

            if w_internal.changed() {
                internal.on(w_internal.val(DRAW | UPDATE));

                if w_internal.check(GRAB) {
                    self.grab_id = Some(id);
                    internal.on(GRAB);
                }

                if !w_internal.check(FOCUS | ENABLED | VISIBLE) {
                    self.unfocus(internal);
                }
            }
        }
    }

    /// Set Widget Bounds (x, y, width, height)
    fn compute_min(&self) -> Dimensions<D> {
        self.layout.minimum_size(&self.widgets_i)
    }

    /// Step focus on Widget array
    fn step_focus(&mut self, internal: &mut WidgetInternal<P, D>, back: bool) -> bool {
        if !self.widgets.is_empty() {
            if let Some(id) = self.focus_id {
                let widget = (&mut self.widgets_i[id], &mut self.widgets[id]);
                let focus = widget.1.step_focus(widget.0, back);

                if widget.0.changed() {
                    internal.on(widget.0.val(DRAW | UPDATE));
                }

                if focus {
                    return true;
                } else {
                    widget.1.unfocus(widget.0);
                    if widget.0.changed() {
                        internal.on(widget.0.val(DRAW | UPDATE));
                    }

                    widget.0.off(FOCUS);
                }
            }
            let mut step = self.step(back);

            if step.0 {
                while let Some(widget) = self.widgets.get_mut(step.1) {
                    let w_internal = &mut self.widgets_i[step.1];
                    let focus =
                        w_internal.check(ENABLED | VISIBLE) && widget.step_focus(w_internal, back);

                    if w_internal.changed() {
                        internal.on(w_internal.val(DRAW | UPDATE));
                    }

                    if focus {
                        w_internal.on(FOCUS);
                        return focus;
                    } else {
                        step = self.step(back);
                        if !step.0 {
                            break;
                        }
                    }
                }
            }
        }

        false
    }

    fn unhover(&mut self, internal: &mut WidgetInternal<P, D>) {
        if let Some(id) = self.hover_id {
            let w_internal = &mut self.widgets_i[id];

            self.widgets[id].unhover(w_internal);
            if w_internal.changed() {
                internal.on(w_internal.val(DRAW | UPDATE));
            }

            w_internal.off(HOVER);
            self.hover_id = Option::None;
        }
    }

    fn unfocus(&mut self, internal: &mut WidgetInternal<P, D>) {
        if let Some(id) = self.focus_id {
            let w_internal = &mut self.widgets_i[id];

            self.widgets[id].unfocus(w_internal);
            if w_internal.changed() {
                internal.on(w_internal.val(DRAW | UPDATE));
            }

            w_internal.off(FOCUS);
            self.focus_id = Option::None;
        }
    }
}

impl<P, D> Boxed for Container<P, D>
where
    D: Sized + PartialOrd + From<u8>,
    P: Sized + Add<Output = P> + PartialOrd + From<D> + From<u8>,
{
    #[inline]
    fn boxed(mut self) -> Box<Self>
    where
        Self: Sized,
    {
        self.widgets.shrink_to_fit();
        self.widgets_i.shrink_to_fit();

        Box::new(self)
    }
}

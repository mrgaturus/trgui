use crate::widget::{Widget, WidgetInternal, Boundaries, FocusAction};
use crate::state::{MouseState, KeyState};

type WidgetList = Vec<Box<dyn Widget>>;

pub struct Container {
    focus_id: Option<usize>,
    layout: Option<Box<dyn Layout>>,
    widgets: WidgetList,
    internal: WidgetInternal
    
}

pub trait Layout {
    fn layout(&self, container: &mut WidgetList);
    fn set_container(&self, container: &Container);
    fn get_container(&self) -> &Container;
}

impl Container {
    pub fn new() -> Self {
        Container {
            focus_id: Option::None,
            layout: Option::None,
            widgets: WidgetList::new(),
            internal: WidgetInternal::new((0, 0, 0, 0))
            
        }
    }

    pub fn new_with_layout(layout: Box<dyn Layout>) -> Self {
        Container {
            focus_id: Option::None,
            layout: Option::Some(layout),
            widgets: WidgetList::new(),
            internal: WidgetInternal::new((0, 0, 0, 0))
            
        }
    }

    pub fn set_layout(&mut self, layout: Box<dyn Layout>) {
        self.layout = Option::Some(layout);
    }

    pub fn add_widget(&mut self, widget: Box<dyn Widget>) {
        self.widgets.push(widget);
    }

    fn step_focus(&mut self, back: bool) -> (bool, usize) {
        if let None = self.focus_id {
            let mut val = 0;
            if back {
                val = self.widgets.len() - 1;
            }
            self.focus_id = Some(val);

            (true, val)
        } else if let Some(id) = self.focus_id {
            if id > self.widgets.len() - 2 {
                self.focus_id = None;
                (false, 0)
            } else {
                let val;
                if back {
                    val = id - 1
                } else {
                    val = id + 1
                }
                self.focus_id = Some(val);
                (true, val)
            }
        } else {
            (false, 0)
        }
    }

    fn step(&self, back: bool, n: &mut usize) {
        if back {
            *n -= 1;
        } else {
            *n += 1;
        }
    }
}

impl Widget for Container {
    fn draw(&self) {
        for widget in self.widgets.iter() {
            (*widget).draw();
        }
    }

    fn update(&mut self) {
        if let Some(layout) = &self.layout {
            (*layout).layout(&mut self.widgets);
        }

        for widget in self.widgets.iter_mut() {
            (*widget).update();
        }
    }

    fn handle(&mut self, mouse: &MouseState, key: &KeyState) {
        let mut relative = mouse.clone();
        relative.set_relative(self.get_bounds());

        for widget in self.widgets.iter_mut() {
            (*widget).handle(&relative, key);
        }
    }

    fn get_bounds(&self) -> Boundaries {
        self.internal.boundaries()
    }

    fn set_bounds(&mut self, bounds: Boundaries) {
        self.internal.set_boundaries(bounds);
    }

    /// Focus the current widget
    fn focus(&mut self, back: bool) -> FocusAction {
        //println!("--{:?} {}--", self.focus_id, self.widgets.len());
        if !self.widgets.is_empty() {
            if let None = self.focus_id {
                self.step_focus(back);
                println!("EXTREME {} {}", back, self.focus_id.unwrap());
                return self.widgets[self.focus_id.unwrap()].focus(back);
            } else if self.focus_id.is_some() {
                let mut step = self.step_focus(back);
                if step.0 {
                    while let Some(widget) = self.widgets.get_mut(step.1) {
                        println!("CALL");
                        let focus = widget.focus(back);
                        match focus {
                            FocusAction::Ok => {
                                println!("OK {:?}", step);
                                break;
                            },
                            FocusAction::False => {
                                println!("FALSE {:?}", step);
                                step = self.step_focus(back);
                                if step.0 {
                                    continue;
                                } else {
                                    break;
                                }
                            },
                            FocusAction::Next => {
                                println!("NEXT {:?}", step);
                                self.step_focus(!back);
                                break;
                            }
                        }
                    }

                    return FocusAction::Next;
                }

            }
        }

        FocusAction::False
    }

    /// Unfocus the current widget
    fn unfocus(&mut self) {
        self.focus_id = Option::None;

        for widget in self.widgets.iter_mut() {
            (*widget).unfocus();
        }
    }
}
pub mod layout;
pub mod input;
mod render;

use std::time::{Duration, Instant};
use std::collections::BTreeMap;

pub use layout::Layout;

pub type Point = euclid::default::Point2D<f32>;
pub type Rect = euclid::default::Rect<f32>;
pub type Size = euclid::default::Size2D<f32>;
pub type Vector = euclid::default::Vector2D<f32>;

type Pool = std::collections::HashMap<Key, Control>;

use input::{ClickFSM, Input, InputFlags};

pub use render::Render;

/// Lower values appear below higher values. Can be considered a Z position.
pub type Layer = u8;
pub const LAYER_DEFAULT: Layer = 255;

/// A UI tree.
pub struct Ui {
    /// Control pool/arena. Holds the control tree in a flat format.
    pool: Pool,

    /// Current frame number, increases by 1 each frame.
    frame_no: u8,

    /// The current parent control. References the root node if at the top of the tree.
    parent: Key,

    /// The previous sibling control.
    prev_sibling: Option<Key>, // TODO: consider next_child_index

    /// The display dimensions.
    screen: Rect,

    /// The time at which the previous update occurred.
    most_recent_update: Instant,

    /// Controls that care about whether the mouse intersects them, indexed by layer.
    cares_about_mouse_intersect: BTreeMap<Layer, Vec<Key>>,
}

/// Interface for adding controls to the UI tree.
pub struct UiFrame<'ui> {
    ui: &'ui mut Ui,
    pub delta_time: Duration,
}

type UserKey = u8;

/// A key that uniquely identifies a control.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Key {
    /// User-provided ID, uniquely identifies control between its *possible* siblings. The actual value doesn't matter.
    user: UserKey,

    /// Key of parent(s). This means you don't have to worry about creating _globally_ unique `user` values, and allows
    /// traversing up the tree.
    parent: Option<Box<Key>>,
}

/// Absolutely-positioned region on-screen, used for input and layout.
#[derive(Debug, Clone)]
pub struct Region {
    pub rect: Rect,
    pub layer: Layer,
}

/// A UI element.
#[derive(Debug)]
pub struct Control {
    /// Unique identifier for this control; also allows access to parent control(s).
    pub key: Key,

    /// The behaviour that this control inhibits.
    pub widget: Widget,

    /// The children of this control, if any.
    pub children: Vec<Key>,

    /// The frame number this control was most recently touched on.
    /// After each update, we can garbage-collect all the controls with `updated_frame_no`s not equal to `Ui::frame_no`.
    updated_frame_no: u8,

    /// Layout parameters of self (and children, if any).
    layout: Layout,

    /// The rectangle of space this control takes up, calculated via layout parameters.
    pub region: Region,

    /// The input state, where flags are set for as long as that input is held.
    pub inputs_active: InputFlags,

    /// The inputs that this control requires a UI tree update to handle, i.e. those that this control 'cares about' for
    /// things other than state.
    inputs_trigger_update: InputFlags,

    pub left_click: ClickFSM,
    pub right_click: ClickFSM,
    pub middle_click: ClickFSM,
}

/// A widget is the 'type' of a control. They are effectively bags of style properties intended to inform the
/// renderer how a particular control should look.
#[derive(Debug)]
pub enum Widget {
    /// Just holds children, like an HTML `<div>`.
    Group,

    /// A text label.
    Text(String),

    /// A button.
    Button,
}

impl Ui {
    pub fn new() -> Self {
        let mut ui = Self {
            pool: Pool::with_capacity(1),
            cares_about_mouse_intersect: BTreeMap::new(),
            frame_no: 0,
            parent: Key::root(),
            prev_sibling: None,
            screen: Rect {
                origin: Point::new(0.0, 0.0),
                size: Size::new(800.0, 600.0),
            },
            most_recent_update: Instant::now(),
        };

        // Create omnipresent root node.
        ui.pool.insert(Key::root(), Control::new(ui.frame_no, Key::root(), Widget::Group));

        ui
    }

    /// Re-create the tree.
    pub fn update<F: FnOnce(&mut UiFrame<'_>), R: Render>(&mut self, f: F, renderer: &mut R) {
        self.begin_frame();

        let now = Instant::now();
        let delta_time = {
            let delta = now.duration_since(self.most_recent_update);
            self.most_recent_update = now;
            delta
        };

        f(&mut UiFrame {
            ui: self,
            delta_time,
        });

        self.end_frame();
        layout::compute(&mut self.pool, &Key::root(), self.screen.clone(), renderer);
    }

    /// Returns the number of controls, besides the root, in the tree.
    pub fn len(&self) -> usize {
        self.pool.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn resize<R: Render>(&mut self, screen: Rect, renderer: &mut R) {
        self.screen = screen;
        layout::compute(&mut self.pool, &Key::root(), self.screen.clone(), renderer);
    }

    #[must_use = "if true is returned, call update"]
    pub fn set_mouse_pos(&mut self, pos: Point) -> bool {
        let mut needs_update = false;

        // TODO: raycast via layer (perhaps have layout::compute return a sorted Vec<Key>)

        self.iter_mut_depth_first(&Key::root(), &mut |ctrl: &mut Control| {
            let is_hit = ctrl.region.rect.contains(pos);
            let was_hit = ctrl.inputs_active.contains(Input::MouseOver);

            if is_hit != was_hit {
                ctrl.inputs_active.toggle(Input::MouseOver);

                if ctrl.inputs_trigger_update.contains(Input::MouseOver) {
                    needs_update = true;
                }
            }
        });

        needs_update
    }

    fn set_input_flag_on_controls_if(&mut self, set: bool, flag: InputFlags, mask: InputFlags) -> bool {
        let mut needs_update = false;

        for (_, ctrl) in &mut self.pool {
            let to_set;
            if ctrl.inputs_active.contains(mask) {
                to_set = set;
            } else {
                to_set = false;
            }

            let was_set = ctrl.inputs_active.contains(flag);

            if to_set != was_set {
                ctrl.inputs_active.toggle(flag);

                if ctrl.inputs_trigger_update.contains(flag) {
                    needs_update = true;
                }
            }
        }

        needs_update
    }

    #[must_use = "if true is returned, call update"]
    pub fn set_left_mouse(&mut self, is_down: bool) -> bool {
        self.set_input_flag_on_controls_if(is_down, Input::LeftMouseDown.into(), Input::MouseOver.into())
    }

    /// Iterate through a tree and its children, depth-first AKA post-order.
    pub fn iter_depth_first<D: FnMut(&Control)>(&self, key: &Key, f: &mut D) {
        let control = self.pool.get(key).unwrap();

        for child in control.children.iter() {
            self.iter_depth_first(child, f);
        }

        f(control);
    }

    pub fn iter_mut_depth_first<D: FnMut(&mut Control)>(&mut self, key: &Key, f: &mut D) {
        let control = self.pool.get(key).unwrap();

        for child in control.children.clone() {
            self.iter_mut_depth_first(&child, f);
        }

        let control = self.pool.get_mut(key).unwrap();

        f(control);
    }

    /// Iterate through a tree and its children, pre-order.
    pub fn iter_breadth_first<D: FnMut(&Control)>(&self, key: &Key, f: &mut D) {
        let control = self.pool.get(key).unwrap();

        f(control);

        for child in control.children.iter() {
            self.iter_breadth_first(child, f);
        }
    }

    pub fn render<R: Render>(&mut self, renderer: &mut R) {
        self.iter_breadth_first(&Key::root(), &mut |ctrl| {
            let Control { widget, region, layout, .. } = ctrl;

            let mut region = region.clone();

            if layout.center_x {
                let parent = &self.pool[ctrl.key.parent.as_ref().unwrap()];

                region.rect.origin.x += parent.region.rect.size.width / 2.0;
                region.rect.origin.x -= region.rect.size.width / 2.0;
            }

            if layout.center_y {
                let parent = &self.pool[ctrl.key.parent.as_ref().unwrap()];

                region.rect.origin.y += parent.region.rect.size.height / 2.0;
                region.rect.origin.y -= region.rect.size.height / 2.0;
            }

            match widget {
                Widget::Group => {}
                Widget::Button => renderer.render_button(&region, ctrl.left_click.is_press()),
                Widget::Text(text) => renderer.render_text(&region, text),
            }
        });
    }

    fn begin_frame(&mut self) {
        self.frame_no = self.frame_no.wrapping_add(1);

        // Touch the root so it isn't removed by end_frame() later.
        self.pool.get_mut(&Key::root()).unwrap().updated_frame_no = self.frame_no;

        self.cares_about_mouse_intersect.clear();

        self.parent = Key::root();
        self.prev_sibling = None;
    }

    fn end_frame(&mut self) {
        assert!(self.parent == Key::root(), "begin/end mismatch");

        // Garbage collection: remove old (untouched during update) controls.
        let frame_no = self.frame_no;
        self.forget_old_children(&Key::root());
        self.pool.retain(|_, control| !control.is_old(frame_no));
    }

    fn key(&self, user: UserKey) -> Key {
        Key {
            user,
            parent: Some(Box::new(self.parent.clone())),
        }
    }

    fn begin_control(&mut self, key: Key, widget: Widget) {
        let parent = self.pool.get(&self.parent).expect("missing parent in pool");

        // Figure out where this new control needs to be placed in the parent's children.
        let child_index = if let Some(prev_sibling) = &self.prev_sibling {
            // Find where prev_sibling is, and just return the index after it.
            parent
                .children
                .iter()
                .position(|child| child == prev_sibling)
                .expect("prev_sibling is not actually a sibling")
                + 1
        } else {
            // This is the first child.
            0
        };

        // Insert control into children at child_index.
        if let Some(previous) = self.pool.get(&key) {
            // The control existed on the previous frame, and therefore needs moving.

            assert!(
                previous.updated_frame_no != self.frame_no,
                "controls must not share keys; verify keys are unique among siblings",
            );

            // Where was it previously?
            let prev_index = parent
                .children
                .iter()
                .position(|child| child == &previous.key)
                .expect("control changed parents");

            if prev_index != child_index {
                // Move the control to the right place by swapping. This is quicker than a remove then reinsert as only one
                // element needs to be shifted, and is also a no-op if the indices are equal (very likely).

                // Verify swap isn't a logical error: the control that was previously at child_index must be old.
                debug_assert!(self.pool.get(&parent.children[child_index]).unwrap().is_old(self.frame_no));

                let parent = self.pool.get_mut(&self.parent).unwrap();
                parent.children.swap(child_index, prev_index);
            }
        } else {
            // This control is new on this frame!
            let parent = self.pool.get_mut(&self.parent).unwrap();
            parent.children.insert(child_index, key.clone());
        }

        // Set up (potentially new) control.
        let frame_no = self.frame_no;
        if let Some(ctrl) = self.pool.get_mut(&key) {
            ctrl.touch(frame_no);
            ctrl.accept_widget(widget);
        } else {
            self.pool.insert(key.clone(), Control::new(frame_no, key.clone(), widget));
        }

        // Enter into this control.
        self.parent = key;
        self.prev_sibling = None;
    }

    fn end_control(&mut self) {
        let old_parent = self.parent.clone();
        self.forget_old_children(&old_parent);

        // Move up.
        self.parent = *(self.parent.parent.take().unwrap());
        self.prev_sibling = Some(old_parent);
    }

    fn forget_old_children(&mut self, control_key: &Key) {
        let control = &self.pool[control_key];

        // Find the first child that was not updated ('old'), and truncate from then on.
        // This works because all of the new children will populate the start of the vec, and have swap-shifted the old
        // ones to the right - like a bubble sort partition.
        if let Some(first_old) = control.children
            .iter()
            .position(|child| {
                self.pool[child].is_old(self.frame_no)
            })
        {
            let control = self.pool.get_mut(control_key).unwrap();

            if cfg!(debug_assertions) {
                let removed: Vec<Key> = control.children.drain(first_old..).collect();

                drop(control);

                // Verify the removed children are ALL old.
                for child in removed {
                    assert!(self.pool[&child].is_old(self.frame_no));
                }
            } else {
                // Equivalent, but unchecked.
                control.children.truncate(first_old);
            }
        }
    }
}

impl UiFrame<'_> {
    #[allow(unused)]
    fn current(&self) -> &Control {
        let key;
        if let Some(prev_sibling) = self.ui.prev_sibling.as_ref() {
            key = prev_sibling;
        } else {
            key = &self.ui.parent;
        }

        &self.ui.pool[key]
    }

    fn current_mut(&mut self) -> &mut Control {
        let key;
        if let Some(prev_sibling) = self.ui.prev_sibling.as_ref() {
            key = prev_sibling;
        } else {
            key = &self.ui.parent;
        }

        self.ui.pool.get_mut(key).unwrap()
    }

    /// Create a group of controls laid out horizontally, left-to-right.
    pub fn hbox<K: Into<UserKey>, F: FnOnce(&mut Self)>(&mut self, key: K, f: F) {
        let key = self.ui.key(key.into());

        self.ui.begin_control(key, Widget::Group);
        self.current_mut().layout.direction = layout::Dir::LeftRight { wrap: true };
        f(self);
        self.ui.end_control();
    }

    /// Create a group of controls laid out vertically, top-to-bottom.
    pub fn vbox<K: Into<UserKey>, F: FnOnce(&mut Self)>(&mut self, key: K, f: F) {
        let key = self.ui.key(key.into());

        self.ui.begin_control(key, Widget::Group);
        self.current_mut().layout.direction = layout::Dir::TopBottom { wrap: false };
        f(self);
        self.ui.end_control();
    }

    pub fn known_size<K: Into<UserKey>, F: FnOnce(&mut Self)>(&mut self, key: K, width: f32, height: f32, f: F) {
        let key = self.ui.key(key.into());

        self.ui.begin_control(key, Widget::Group);

        let ctrl = self.current_mut();
        ctrl.layout.direction = layout::Dir::BackFront;
        ctrl.layout.width = width..=width;
        ctrl.layout.height = height..=height;

        f(self);

        self.ui.end_control();
    }

    pub fn pad<K: Into<UserKey>>(&mut self, key: K, padding: f32) {
        let key = self.ui.key(key.into());

        let parent_direction = self.ui.pool[&self.ui.parent].layout.direction;

        self.ui.begin_control(key, Widget::Group);

        let ctrl = self.current_mut();
        ctrl.layout.direction = layout::Dir::BackFront;

        ctrl.layout.width = 0.0..=0.0;
        ctrl.layout.height = 0.0..=0.0;

        match parent_direction {
            layout::Dir::BackFront => (),
            layout::Dir::LeftRight { .. } => ctrl.layout.width = padding..=padding,
            layout::Dir::TopBottom { .. } => ctrl.layout.height = padding..=padding,
        }

        self.ui.end_control();
    }

    /// Create a simple block of text.
    pub fn text<'a, K: Into<UserKey>, S: Into<String>>(&'a mut self, key: K, string: S) -> Text<'a> {
        self.ui.begin_control(self.ui.key(key.into()), Widget::Text(string.into()));
        self.ui.end_control();

        Text {
            ctrl: self.current_mut(),
        }
    }

    /// A button with a label.
    pub fn button<'a, K: Into<UserKey>, S: Into<String>>(&'a mut self, key: K, label: S) -> Button<'a> {
        let key = self.ui.key(key.into());

        self.ui.begin_control(key, Widget::Button);
        self.text(0, label).center_x().center_y();
        self.ui.end_control();

        let ctrl = self.current_mut();

        ctrl.layout.width = 100.0..=100.0;
        ctrl.layout.height = 32.0..=32.0;

        Button {
            is_click: ctrl.advance_left_click().is_click(),
            ctrl,
        }
    }
}

impl Key {
    /// Returns the key of the root control. The root is guaranteed to always exist in `Ui::pool`.
    pub const fn root() -> Self {
        Self {
            user: 0,
            parent: None,
        }
    }
}

impl Control {
    fn new(frame_no: u8, key: Key, widget: Widget) -> Self {
        Self {
            key,
            widget,
            children: Vec::new(),
            updated_frame_no: frame_no,
            layout: Layout::default(),
            region: Region {
                rect: Rect::zero(),
                layer: LAYER_DEFAULT,
            },
            inputs_active: InputFlags::empty(),
            inputs_trigger_update: InputFlags::empty(),
            left_click: Default::default(),
            right_click: Default::default(),
            middle_click: Default::default(),
        }
    }

    fn is_old(&self, frame_no: u8) -> bool {
        frame_no != self.updated_frame_no
    }

    fn touch(&mut self, frame_no: u8) {
        self.updated_frame_no = frame_no;
    }

    /// Advances the left_click FSM and sets the relevant inputs_trigger_update flags.
    fn advance_left_click(&mut self) -> ClickFSM {
        self.inputs_trigger_update |= Input::LeftMouseDown | Input::MouseOver;
        self.left_click = self.left_click.advance(Input::LeftMouseDown, self.inputs_active);
        self.left_click
    }
}

impl Control {
    /// Accept a new widget configuration, merging the previous widget's properties where possible to preserve state.
    fn accept_widget(&mut self, new: Widget) {
        match (&mut self.widget, new) {
            (_, new) => self.widget = new,
        }
    }
}

impl Default for Widget {
    fn default() -> Self {
        Widget::Group
    }
}

pub struct Button<'a> {
    ctrl: &'a mut Control,
    is_click: bool,
}

impl Button<'_> {
    pub fn clicked(&self) -> bool {
        self.is_click
    }

    pub fn with_width(&mut self, width: f32) -> &mut Self {
        self.ctrl.layout.width = width..=width;
        self
    }

    pub fn with_height(&mut self, height: f32) -> &mut Self {
        self.ctrl.layout.height = height..=height;
        self
    }
}

pub struct Text<'a> {
    ctrl: &'a mut Control,
}

impl Text<'_> {
    pub fn center_x(&mut self) -> &mut Self {
        self.ctrl.layout.center_x = true;
        self
    }

    pub fn center_y(&mut self) -> &mut Self {
        self.ctrl.layout.center_y = true;
        self
    }

    pub fn with_width(&mut self, width: f32) -> &mut Self {
        self.ctrl.layout.width = width..=width;
        self
    }

    pub fn with_height(&mut self, height: f32) -> &mut Self {
        self.ctrl.layout.height = height..=height;
        self
    }
}
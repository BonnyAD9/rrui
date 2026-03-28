use std::{
    cell::{Cell, RefCell},
    fmt::Debug,
    rc::{self, Rc},
};

use minlin::{Rect, RectExt, Vec2};

#[derive(Debug)]
pub struct RelPosSrc(Rc<RelPosInner>);

#[derive(Debug, Clone, Default)]
pub struct RelPos(Option<Rc<RelPosInner>>);

#[derive(Debug)]
pub struct RelPosInner {
    children: RefCell<Vec<rc::Weak<RelPosInner>>>,
    /// Position relative to parent
    rel: Cell<Vec2<f32>>,
    /// Absolute position
    pos: Cell<Vec2<f32>>,
}

impl RelPos {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RelPosSrc {
    pub fn update(&self, pos: RelPos) {
        if let Some(v) = pos.0.as_ref() {
            v.add_child(self);
        }
    }

    pub fn move_by(&self, rel: impl Into<Vec2<f32>>) {
        self.0.move_by(rel.into());
    }

    pub fn move_to(&self, pos: impl Into<Vec2<f32>>) {
        self.0.move_to(pos.into());
    }

    pub fn get(&self) -> Vec2<f32> {
        self.0.pos.get()
    }

    pub fn rel_pos(&self) -> RelPos {
        RelPos(Some(self.0.clone()))
    }

    pub fn position_rect(&self, mut rect: Rect<f32>) -> Rect<f32> {
        rect.move_to(rect.pos() + self.get());
        rect
    }
}

impl RelPos {
    pub fn update(&mut self, pos: RelPos) {
        self.0 = pos.0.clone();
    }

    pub fn get(&self) -> Vec2<f32> {
        self.0.as_ref().map(|a| a.pos.get()).unwrap_or_default()
    }

    pub fn relate(&self, by: impl Into<Vec2<f32>>) -> RelPosSrc {
        let by = by.into();
        self.0.as_ref().map(|a| a.relate(by)).unwrap_or_else(|| {
            RelPosSrc(Rc::new(RelPosInner {
                children: vec![].into(),
                pos: by.into(),
                rel: by.into(),
            }))
        })
    }

    pub fn position_rect(&self, mut rect: Rect<f32>) -> Rect<f32> {
        rect.move_to(rect.pos() + self.get());
        rect
    }
}

impl RelPosInner {
    fn move_by(&self, rel: Vec2<f32>) {
        self.pos.set(self.pos.get() + rel);
        self.rel.set(rel);
        self.notify_children(rel);
    }

    fn move_to(&self, pos: Vec2<f32>) {
        let rel = pos - self.rel.get();
        self.move_by(rel);
    }

    fn notify_children(&self, rel: Vec2<f32>) {
        let mut children = self.children.borrow_mut();
        children.retain(|a| {
            if let Some(a) = a.upgrade() {
                a.move_by(rel);
                true
            } else {
                false
            }
        });
    }

    fn relate(&self, by: Vec2<f32>) -> RelPosSrc {
        let pos = self.pos.get() + by;
        let res = RelPosSrc(Rc::new(RelPosInner {
            children: vec![].into(),
            pos: pos.into(),
            rel: by.into(),
        }));
        let child = Rc::downgrade(&res.0);
        let mut children = self.children.borrow_mut();
        children.retain(|a| a.strong_count() != 0);
        children.push(child);
        res
    }

    fn add_child(&self, child: &RelPosSrc) {
        let child = Rc::downgrade(&child.0);
        let mut children = self.children.borrow_mut();
        children
            .retain(|a| a.strong_count() != 0 && a.as_ptr() != child.as_ptr());
        children.push(child);
    }
}

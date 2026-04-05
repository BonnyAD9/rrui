use std::{
    cell::{Cell, RefCell},
    fmt::Debug,
    marker::PhantomData,
    ops::Deref,
    rc::Rc,
};

use minlin::{Rect, RectExt, Vec2};

#[derive(Debug)]
pub struct RelPosSrc(Rc<RelPosInner>);

#[derive(Debug, Clone, Default)]
pub struct RelPos(Option<Rc<RelPosInner>>);

#[derive(Debug)]
pub struct RelPosVal<'a> {
    value: Vec2<f32>,
    inner: Rc<RelPosInner>,
    _p: PhantomData<&'a ()>,
}

#[derive(Debug)]
struct RelPosInner {
    parent: RefCell<RelPos>,
    /// Position relative to parent
    rel: Cell<Vec2<f32>>,
    /// Absolute position
    pos: Cell<Option<Vec2<f32>>>,
}

pub fn update_rel_pos(
    cur: &mut Option<RelPosSrc>,
    new: RelPos,
    rel: Vec2<f32>,
) -> RelPos {
    if let Some(rp) = cur {
        rp.update(new);
        rp.move_to(rel);
        rp.rel_pos()
    } else {
        let rp = new.relate(rel);
        let res = rp.rel_pos();
        *cur = Some(rp);
        res
    }
}

pub fn reposition(cur: &Option<RelPosSrc>, pos: Vec2<f32>) {
    if let Some(rp) = cur {
        rp.move_to(pos);
    }
}

pub fn reposition_off(
    cur: &Option<RelPosSrc>,
    pos: Vec2<f32>,
    offset: Vec2<f32>,
) {
    if let Some(rp) = cur {
        rp.move_to(pos + offset);
    }
}

pub fn get_pos(cur: &mut Option<RelPosSrc>) -> RelPosVal<'_> {
    cur.as_mut().unwrap().get()
}

impl RelPos {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RelPosSrc {
    pub fn update(&mut self, pos: RelPos) {
        self.0.parent.replace(pos);
    }

    pub fn move_by(&self, rel: impl Into<Vec2<f32>>) {
        self.0.move_by(rel.into());
    }

    pub fn move_to(&self, pos: impl Into<Vec2<f32>>) {
        self.0.move_to(pos.into());
    }

    pub fn get(&mut self) -> RelPosVal<'_> {
        let inner = self.0.clone();
        let value = self.0.get();
        inner.pos.set(Some(value));
        RelPosVal {
            value,
            inner,
            _p: PhantomData,
        }
    }

    pub fn rel_pos(&self) -> RelPos {
        RelPos(Some(self.0.clone()))
    }
}

impl RelPos {
    pub fn update(&mut self, pos: RelPos) {
        self.0 = pos.0.clone();
    }

    pub fn get(&self) -> Vec2<f32> {
        self.0.as_ref().map(|a| a.get()).unwrap_or_default()
    }

    pub fn relate(&self, by: impl Into<Vec2<f32>>) -> RelPosSrc {
        RelPosSrc(Rc::new(RelPosInner {
            parent: self.clone().into(),
            rel: by.into().into(),
            pos: None.into(),
        }))
    }

    pub fn position_rect(&self, mut rect: Rect<f32>) -> Rect<f32> {
        rect.move_to(rect.pos() + self.get());
        rect
    }
}

impl<'a> RelPosVal<'a> {
    pub fn position_rect(&self, mut rect: Rect<f32>) -> Rect<f32> {
        rect.move_to(rect.pos() + self.value);
        rect
    }
}

impl<'a> Deref for RelPosVal<'a> {
    type Target = Vec2<f32>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'a> Drop for RelPosVal<'a> {
    fn drop(&mut self) {
        self.inner.pos.set(None);
    }
}

impl RelPosInner {
    fn move_by(&self, rel: Vec2<f32>) {
        self.rel.set(self.rel.get() + rel);
    }

    fn move_to(&self, pos: Vec2<f32>) {
        self.rel.set(pos);
    }

    fn get(&self) -> Vec2<f32> {
        self.pos
            .get()
            .unwrap_or_else(|| self.parent.borrow_mut().get() + self.rel.get())
    }
}

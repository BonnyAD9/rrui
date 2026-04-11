use minlin::{Rect, RectExt};

use crate::{
    Direction, LayoutBounds, LayoutFlags, LayoutParams, RelPos, Widget,
};

pub fn stack<W, Rend, Msg, Evt, Theme>(
    children: &mut [W],
    spacing: f32,
    direction: Direction,
    lp: &mut LayoutParams<'_, Rend, Msg, Evt, Theme>,
    bounds: &LayoutBounds,
    rel_pos: RelPos,
    flags: LayoutFlags,
) -> Rect<f32>
where
    W: Widget<Rend, Msg, Evt, Theme>,
{
    match direction {
        Direction::Left => {
            stack_from_left(children, spacing, lp, bounds, rel_pos, flags)
        }
        Direction::Top => {
            stack_from_top(children, spacing, lp, bounds, rel_pos, flags)
        }
        Direction::Right => {
            stack_from_right(children, spacing, lp, bounds, rel_pos, flags)
        }
        Direction::Bottom => {
            stack_from_bottom(children, spacing, lp, bounds, rel_pos, flags)
        }
    }
}

pub fn stack_from_left<W, Rend, Msg, Evt, Theme>(
    children: &mut [W],
    spacing: f32,
    lp: &mut LayoutParams<'_, Rend, Msg, Evt, Theme>,
    bounds: &LayoutBounds,
    rel_pos: RelPos,
    flags: LayoutFlags,
) -> Rect<f32>
where
    W: Widget<Rend, Msg, Evt, Theme>,
{
    let mut used = Rect::from_pos_size(bounds.pos, [0., 0.]);
    if children.is_empty() {
        return used;
    }

    let mut remaining = *bounds;
    remaining.filly();

    let last = children.len() - 1;
    for child in &mut children[..last] {
        let cu = child.layout(lp, &remaining, rel_pos.clone(), flags);
        let shr = cu.width() + spacing;
        used.set_height(used.height().max(cu.height()));
        remaining.shrink_left(shr);
        used.extend_right(shr);
    }

    let cu = children[last].layout(lp, &remaining, rel_pos, flags);
    used.set_height(used.height().max(cu.height()));
    remaining.shrink_left(cu.width());
    used.extend_right(cu.width());

    used
}

pub fn stack_from_top<W, Rend, Msg, Evt, Theme>(
    children: &mut [W],
    spacing: f32,
    lp: &mut LayoutParams<'_, Rend, Msg, Evt, Theme>,
    bounds: &LayoutBounds,
    rel_pos: RelPos,
    flags: LayoutFlags,
) -> Rect<f32>
where
    W: Widget<Rend, Msg, Evt, Theme>,
{
    let mut used = Rect::from_pos_size(bounds.pos, [0., 0.]);
    if children.is_empty() {
        return used;
    }

    let mut remaining = *bounds;
    remaining.fillx();

    let last = children.len() - 1;
    for child in &mut children[..last] {
        let cu = child.layout(lp, &remaining, rel_pos.clone(), flags);
        let shr = cu.height() + spacing;
        used.set_width(used.width().max(cu.width()));
        remaining.shrink_top(shr);
        used.extend_bot(shr);
    }

    let cu = children[last].layout(lp, &remaining, rel_pos, flags);
    used.set_width(used.width().max(cu.width()));
    remaining.shrink_top(cu.height());
    used.extend_bot(cu.height());

    used
}

pub fn stack_from_right<W, Rend, Msg, Evt, Theme>(
    children: &mut [W],
    spacing: f32,
    lp: &mut LayoutParams<'_, Rend, Msg, Evt, Theme>,
    bounds: &LayoutBounds,
    rel_pos: RelPos,
    flags: LayoutFlags,
) -> Rect<f32>
where
    W: Widget<Rend, Msg, Evt, Theme>,
{
    let mut used = Rect::new(bounds.max_right(), bounds.pos.y, 0., 0.);
    if children.is_empty() {
        return used;
    }

    let mut remaining = *bounds;
    remaining.filly();

    let last = children.len() - 1;
    for child in &mut children[..last] {
        let cu = child.layout(lp, &remaining, rel_pos.clone(), flags);
        child.reposition(
            lp.theme,
            (remaining.max_right() - cu.width(), cu.y).into(),
        );
        let shr = cu.width() + spacing;
        used.set_height(used.height().max(cu.height()));
        remaining.shrink_right(shr);
        used.extend_left(shr);
    }

    let cu = children[last].layout(lp, &remaining, rel_pos, flags);
    children[last].reposition(
        lp.theme,
        (remaining.max_right() - cu.width(), cu.y).into(),
    );
    used.set_height(used.height().max(cu.height()));
    remaining.shrink_right(cu.width());
    used.extend_left(cu.width());

    used
}

pub fn stack_from_bottom<W, Rend, Msg, Evt, Theme>(
    children: &mut [W],
    spacing: f32,
    lp: &mut LayoutParams<'_, Rend, Msg, Evt, Theme>,
    bounds: &LayoutBounds,
    rel_pos: RelPos,
    flags: LayoutFlags,
) -> Rect<f32>
where
    W: Widget<Rend, Msg, Evt, Theme>,
{
    let mut used = Rect::new(bounds.pos.x, bounds.max_bot(), 0., 0.);
    if children.is_empty() {
        return used;
    }

    let mut remaining = *bounds;
    remaining.fillx();

    let last = children.len() - 1;
    for child in &mut children[..last] {
        let cu = child.layout(lp, &remaining, rel_pos.clone(), flags);
        child.reposition(
            lp.theme,
            (cu.x, remaining.max_bot() - cu.height()).into(),
        );
        let shr = cu.height() + spacing;
        used.set_width(used.width().max(cu.width()));
        remaining.shrink_bot(shr);
        used.extend_top(shr);
    }

    let cu = children[last].layout(lp, &remaining, rel_pos, flags);
    children[last].reposition(
        lp.theme,
        (cu.x, remaining.max_bot() - cu.height()).into(),
    );
    used.set_width(used.width().max(cu.width()));
    remaining.shrink_bot(cu.height());
    used.extend_top(cu.height());

    used
}

mod layout_bounds;
mod layout_params;
mod layout_size;
mod rel_pos;

use minlin::{Rect, RectExt};

use crate::{Direction, Widget};

pub use self::{
    layout_bounds::*, layout_params::*, layout_size::*, rel_pos::*,
};

pub fn stack<W, Rend, Msg, Evt, Theme>(
    children: &mut [W],
    spacing: f32,
    direction: Direction,
    lp: &mut LayoutParams<'_, Rend, Msg, Theme>,
    bounds: &LayoutBounds,
    rel_pos: RelPos,
) -> Rect<f32>
where
    W: Widget<Rend, Msg, Evt, Theme>,
{
    match direction {
        Direction::Left => todo!(),
        Direction::Top => {
            stack_from_top(children, spacing, lp, bounds, rel_pos)
        }
        Direction::Right => todo!(),
        Direction::Bottom => todo!(),
    }
}

pub fn stack_from_top<W, Rend, Msg, Evt, Theme>(
    children: &mut [W],
    spacing: f32,
    lp: &mut LayoutParams<'_, Rend, Msg, Theme>,
    bounds: &LayoutBounds,
    rel_pos: RelPos,
) -> Rect<f32>
where
    W: Widget<Rend, Msg, Evt, Theme>,
{
    let mut used =
        Rect::from_pos_size(bounds.pos, [bounds.size.best_max().x, 0.]);
    if children.is_empty() {
        return used;
    }

    let mut remaining = *bounds;
    remaining.fillx();

    let last = children.len() - 1;
    for child in &mut children[..last] {
        let cu = child.layout(lp, &remaining, rel_pos.clone());
        let shr = cu.height() + spacing;
        used.set_width(used.width().max(cu.width()));
        remaining.shrink_top(shr);
        used.extend_bot(shr);
    }

    let cu = children[last].layout(lp, &remaining, rel_pos);
    remaining.shrink_top(cu.height());
    used.extend_bot(cu.height());

    used
}

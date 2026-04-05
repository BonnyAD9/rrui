use std::ops::{Add, Range, RangeInclusive, Sub};

use minlin::{One, Rect, RectExt, Vec2};

#[derive(Debug, Clone, Copy)]
pub struct GridSpan<T = usize>(pub Rect<T>);

impl<T> From<Rect<T>> for GridSpan<T> {
    fn from(value: Rect<T>) -> Self {
        Self(value)
    }
}

impl<T> From<(T, T, T, T)> for GridSpan<T> {
    fn from(value: (T, T, T, T)) -> Self {
        Self(value.into())
    }
}

impl<T> From<[T; 4]> for GridSpan<T> {
    fn from(value: [T; 4]) -> Self {
        Self(value.into())
    }
}

impl<T> From<(Vec2<T>, Vec2<T>)> for GridSpan<T> {
    fn from(value: (Vec2<T>, Vec2<T>)) -> Self {
        Self(value.into())
    }
}

impl<T> From<[Vec2<T>; 2]> for GridSpan<T> {
    fn from([p, s]: [Vec2<T>; 2]) -> Self {
        Self(Rect::new(p.x, p.y, s.x, s.y))
    }
}

impl<T> From<Range<Vec2<T>>> for GridSpan<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd,
{
    fn from(value: Range<Vec2<T>>) -> Self {
        Self(value.into())
    }
}

impl<T> From<Range<[T; 2]>> for GridSpan<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd,
{
    fn from(value: Range<[T; 2]>) -> Self {
        Self(Rect::from_points(value.start, value.end))
    }
}

impl<T> From<Range<(T, T)>> for GridSpan<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd,
{
    fn from(value: Range<(T, T)>) -> Self {
        Self(Rect::from_points(value.start, value.end))
    }
}

impl<T> From<RangeInclusive<Vec2<T>>> for GridSpan<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd + One,
{
    fn from(value: RangeInclusive<Vec2<T>>) -> Self {
        Self(Rect::from_points(
            *value.start(),
            *value.end() + Vec2::new(T::ONE, T::ONE),
        ))
    }
}

impl<T> From<RangeInclusive<[T; 2]>> for GridSpan<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd + One,
{
    fn from(value: RangeInclusive<[T; 2]>) -> Self {
        Self(Rect::from_points(
            *value.start(),
            Vec2::new(T::ONE, T::ONE) + *value.end(),
        ))
    }
}

impl<T> From<RangeInclusive<(T, T)>> for GridSpan<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd + One,
{
    fn from(value: RangeInclusive<(T, T)>) -> Self {
        Self(Rect::from_points(
            *value.start(),
            Vec2::new(T::ONE, T::ONE) + *value.end(),
        ))
    }
}

impl<T> From<(Range<T>, Range<T>)> for GridSpan<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd,
{
    fn from(value: (Range<T>, Range<T>)) -> Self {
        Self(value.into())
    }
}

impl<T> From<[Range<T>; 2]> for GridSpan<T>
where
    T: Copy + Add<Output = T> + Sub<Output = T> + PartialOrd,
{
    fn from([rx, ry]: [Range<T>; 2]) -> Self {
        Self(Rect::from_ranges(rx, ry))
    }
}

impl<T: One> From<Vec2<T>> for GridSpan<T> {
    fn from(value: Vec2<T>) -> Self {
        Self(Rect::new(value.x, value.y, T::ONE, T::ONE))
    }
}

impl<T: One> From<(T, T)> for GridSpan<T> {
    fn from(value: (T, T)) -> Self {
        Self(Rect::new(value.0, value.1, T::ONE, T::ONE))
    }
}

impl<T: One> From<[T; 2]> for GridSpan<T> {
    fn from([x, y]: [T; 2]) -> Self {
        Self(Rect::new(x, y, T::ONE, T::ONE))
    }
}

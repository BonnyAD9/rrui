use minlin::Vec2;

use crate::Size;

pub fn aligned_sizes<S: AsRef<Size>, I: Iterator<Item = S> + Clone>(
    best: f32,
    sizes: impl IntoIterator<IntoIter = I>,
) -> (Vec2<f32>, Vec<f32>) {
    let mut res = vec![];
    let total = align_sizes(best, sizes, &mut res);
    (total, res)
}

pub fn aligned_sizes0<S: AsRef<Size>, I: Iterator<Item = S> + Clone>(
    best: f32,
    sizes: impl IntoIterator<IntoIter = I>,
) -> (Vec2<f32>, Vec<f32>) {
    let mut res = vec![];
    let total = align_sizes0(best, sizes, &mut res);
    (total, res)
}

pub fn align_sizes<S: AsRef<Size>, I: Iterator<Item = S> + Clone>(
    best: f32,
    sizes: impl IntoIterator<IntoIter = I>,
    res: &mut Vec<f32>,
) -> Vec2<f32> {
    let sizes = sizes.into_iter();
    res.reserve(sizes.size_hint().0);
    align_sizes_inner(best, sizes, res)
}

pub fn align_sizes0<S: AsRef<Size>, I: Iterator<Item = S> + Clone>(
    best: f32,
    sizes: impl IntoIterator<IntoIter = I>,
    res: &mut Vec<f32>,
) -> Vec2<f32> {
    let sizes = sizes.into_iter();
    res.reserve(sizes.size_hint().0 + 1);
    res.push(0.);
    align_sizes_inner(best, sizes, res)
}

fn align_sizes_inner<S, I>(
    best: f32,
    sizes: I,
    res: &mut Vec<f32>,
) -> Vec2<f32>
where
    S: AsRef<Size>,
    I: Iterator<Item = S> + Clone,
{
    let bl = res.len();
    let mut total = Vec2::<f32>::ZERO;

    for s in sizes.clone() {
        let parts = s.as_ref().to_parts();
        res.push(parts.x);
        total += parts;
    }

    update_align_sizes(best, sizes, &mut res[bl..], total);

    total
}

pub fn update_align_sizes<S: AsRef<Size>, I: Iterator<Item = S>>(
    best: f32,
    sizes: impl IntoIterator<IntoIter = I>,
    res: &mut [f32],
    total: Vec2<f32>,
) {
    let unit = if total.y == 0. {
        0.
    } else {
        (best - total.x).max(0.) / total.y
    };

    let mut sum = 0.;
    for (s, r) in sizes.into_iter().zip(res) {
        match s.as_ref() {
            Size::Relative(s) => sum += s * unit,
            Size::Absolute(s) => sum += s,
        }
        *r = sum;
    }
}

pub fn update_align_sizes0<S: AsRef<Size>, I: Iterator<Item = S>>(
    best: f32,
    sizes: impl IntoIterator<IntoIter = I>,
    res: &mut [f32],
    total: Vec2<f32>,
) {
    update_align_sizes(best, sizes, &mut res[1..], total);
}

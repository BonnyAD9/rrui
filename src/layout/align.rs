use minlin::Vec2;

use crate::Size;

pub fn aligned_sizes<S: AsRef<Size>, I: Iterator<Item = S> + Clone>(
    best: f32,
    sizes: impl IntoIterator<IntoIter = I>,
) -> (Vec2<f32>, Vec<f32>) {
    let mut res = vec![0.];
    let total = align_sizes(best, sizes, &mut res);
    (total, res)
}

pub fn align_sizes<S: AsRef<Size>, I: Iterator<Item = S> + Clone>(
    best: f32,
    sizes: impl IntoIterator<IntoIter = I>,
    res: &mut Vec<f32>,
) -> Vec2<f32> {
    let sizes = sizes.into_iter();
    res.reserve(sizes.size_hint().0);
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

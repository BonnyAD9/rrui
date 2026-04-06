use std::ops::Range;

use minlin::RangeExt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Align {
    Start,
    Center,
    End,
}

impl Align {
    pub fn calculate(&self, range: Range<f32>, size: f32) -> Range<f32> {
        match self {
            Align::Start => range.start..range.start + size,
            Align::Center => {
                let start = range.start + range.size() / 2.;
                start..start + size
            }
            Align::End => range.end - size..range.end,
        }
    }

    pub fn calculate_start(&self, range: Range<f32>, size: f32) -> f32 {
        match self {
            Align::Start => range.start,
            Align::Center => range.start + (range.size() - size) / 2.,
            Align::End => range.end - size,
        }
    }
}

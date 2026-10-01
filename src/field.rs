use crate::r#gen::{
    check::Check, choice::Choice, pattern::Pattern, range_float::RangeFloat, range_int::RangeInt,
    sequential::Sequential, sequential_float::SequentialFloat,
};

pub struct Field;

impl Field {
    pub fn range_int(min: i64, max: i64) -> RangeInt {
        RangeInt::new(min, max)
    }

    pub fn range_float(min: f64, max: f64, decimals: Option<u32>) -> RangeFloat {
        RangeFloat::new(min, max, decimals)
    }

    pub fn pattern(template: impl Into<String>) -> Pattern {
        Pattern::new(template)
    }

    pub fn choice<I, S>(options: I) -> Choice
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Choice::new(options.into_iter().map(Into::into).collect())
    }

    pub fn check() -> Check {
        Check::new()
    }

    pub fn sequential(current: i64, step: i64) -> Sequential {
        Sequential::new(current, step)
    }

    pub fn sequential_float(current: f64, step: f64, decimals: Option<u32>) -> SequentialFloat {
        SequentialFloat::new(current, step, decimals)
    }
}

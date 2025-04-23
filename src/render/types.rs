use plotters::{
    chart::ChartContext,
    coord::types::RangedCoordf64,
    prelude::{BitMapBackend, Cartesian2d},
};

pub type ChartType =
    ChartContext<'static, BitMapBackend<'static>, Cartesian2d<RangedCoordf64, RangedCoordf64>>;
pub type ErrorType = Box<dyn std::error::Error>;
pub type ChartResult = Result<ChartType, ErrorType>;

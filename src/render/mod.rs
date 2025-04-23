use plotters::{
    chart::{ChartBuilder, ChartContext},
    coord::types::RangedCoordf64,
    prelude::{
        BLUE, BitMapBackend, Cartesian2d, Circle, EmptyElement, IntoDrawingArea, RED, Text, WHITE,
    },
    series::{LineSeries, PointSeries},
};

pub fn run(
    a: i32,
    b: i32,
    c: i32,
    roots: (i32, i32),
    vertex: (i32, i32),
) -> Result<(), Box<dyn std::error::Error>> {
    let mut chart = draw_window()?;
    let quadratic = |x: f64| {
        let a = a as f64;
        let b = b as f64;
        let c = c as f64;
        a * x * x + b * x + c
    };
    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        (-10..=10).map(|x| (x as f64, 0.0)), // Eixo X
        &RED,
    ))?;
    chart.draw_series(LineSeries::new(
        (-10..=10).map(|y| (0.0, y as f64)), // Eixo Y
        &RED,
    ))?;

    chart.draw_series(LineSeries::new(
        (-100..=100).map(|x| {
            let x = x as f64 / 10.0;
            (x, quadratic(x))
        }),
        &BLUE,
    ))?;

    chart.draw_series(PointSeries::of_element(
        vec![
            (roots.0 as f64, 0.0),
            (roots.1 as f64, 0.0),
            (vertex.0 as f64, vertex.1 as f64),
        ],
        3,
        &BLUE,
        &|coord, size, style| {
            EmptyElement::at(coord)
                + Circle::new((0, 0), size, style.filled())
                + Text::new(
                    format!("({:.1}),({:.1})", coord.0, coord.1),
                    (10, 0),
                    ("sans-serif", 15),
                )
        },
    ))?;

    Ok(())
}

fn draw_window() -> Result<
    ChartContext<'static, BitMapBackend<'static>, Cartesian2d<RangedCoordf64, RangedCoordf64>>,
    Box<dyn std::error::Error>,
> {
    let root = BitMapBackend::new("scatter_plot.png", (600, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let chart = ChartBuilder::on(&root)
        .caption("Scatter plot", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-10.0..10.0, -10.0..10.0)?;

    Ok(chart)
}

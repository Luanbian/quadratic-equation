mod types;
use plotters::{
    chart::ChartBuilder,
    prelude::{BLUE, BitMapBackend, Circle, EmptyElement, IntoDrawingArea, RED, Text, WHITE},
    series::{LineSeries, PointSeries},
};
use types::{ChartResult, ChartType, ErrorType};

pub fn run(a: f64, b: f64, c: f64, roots: (f64, f64), vertex: (f64, f64)) -> Result<(), ErrorType> {
    let mut chart = draw_window()?;

    let quadratic = |x: f64| {
        let a = a;
        let b = b;
        let c = c;
        a * x * x + b * x + c
    };

    chart.configure_mesh().draw()?;

    spotlight_x_y_axle(&mut chart)?;

    draw_curve(&mut chart, quadratic)?;

    spotlight_roots_vertex(&mut chart, roots, vertex)?;

    Ok(())
}

fn draw_window() -> ChartResult {
    let root = BitMapBackend::new("scatter_plot.png", (800, 800)).into_drawing_area();
    root.fill(&WHITE)?;

    let chart = ChartBuilder::on(&root)
        .caption("Scatter plot", ("sans-serif", 40))
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-10.0..10.0, -10.0..10.0)?;

    Ok(chart)
}

fn spotlight_x_y_axle(chart: &mut ChartType) -> Result<(), ErrorType> {
    chart.draw_series(LineSeries::new((-10..=10).map(|x| (x as f64, 0.0)), &RED))?;
    chart.draw_series(LineSeries::new((-10..=10).map(|y| (0.0, y as f64)), &RED))?;

    Ok(())
}

fn draw_curve(chart: &mut ChartType, quadratic: impl Fn(f64) -> f64) -> Result<(), ErrorType> {
    chart.draw_series(LineSeries::new(
        (-100..=100).map(|x| {
            let x = x as f64 / 10.0;
            (x, quadratic(x))
        }),
        &BLUE,
    ))?;

    Ok(())
}

fn spotlight_roots_vertex(
    chart: &mut ChartType,
    roots: (f64, f64),
    vertex: (f64, f64),
) -> Result<(), ErrorType> {
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

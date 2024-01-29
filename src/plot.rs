use plotters::{prelude::*, style::full_palette::ORANGE};

use chrono::{Date, TimeZone, Utc};

use std::{collections::HashMap, error::Error};

use crate::data::BenchData;

//const OUT_FILE_NAME: &str = "plotters-doc-data/bench-plot.png";
const COLORS: [RGBColor; 3] = [BLUE, RED, ORANGE];
pub fn generate_plots(data: &Plots) -> Result<(), Box<dyn Error>> {
    for plot in data.iter() {
        let out_file_name = format!("plotters-doc-data/bench-plot-{}.png", plot.0);
        let root = BitMapBackend::new(&out_file_name, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .caption(plot.0, ("sans-serif", 40))
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(
                (Utc.ymd(2023, 1, 1)..Utc::now().date()).monthly(),
                0.0f64..2.0f64,
            )?;

        chart
            .configure_mesh()
            .disable_x_mesh()
            .disable_y_mesh()
            .x_labels(10)
            .max_light_lines(4)
            .x_desc("Commit Date")
            .y_desc("Time (ns)")
            .draw()?;

        for (line, color) in plot.1.iter().zip(COLORS) {
            chart.draw_series(LineSeries::new(
                line.1.iter().map(|p| (str_to_utc(&p.x), p.y)),
                &color,
            ))?;

            chart.draw_series(
                line.1
                    .iter()
                    .map(|p| Circle::new((str_to_utc(&p.x), p.y), 3, color.filled())),
            )?;
        }

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
        println!("Result has been saved to {}", out_file_name);
    }

    Ok(())
}

// Convert <sha>-year-month-day to `Utc` object, discarding the commit
fn str_to_utc(date_str: &str) -> Date<Utc> {
    let date: Vec<u32> = date_str
        .split('-')
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect();
    Utc.ymd(date[0] as i32, date[1], date[2])
}

struct Plot {
    // Name of the plot aka benchmark group
    name: String,
    // Plot can contain multiple lines based on benchmark params
    lines: Vec<Line>,
}
struct Line {
    // E.g. `rc=100`
    name: Option<String>,
    points: Vec<Point>,
}

// Historical benchmark result, showing the performance at a given Git commit
#[derive(Debug)]
pub struct Point {
    // Commit & date associated with benchmark
    x: String,
    // Benchmark time (avg.)
    y: f64,
}

// Plots of benchmark results over time/Git history
// Plots are separated by benchmark input e.g. Fibonacci `num-100`
// It doesn't reveal much information to view multiple input results on the same graph, since they are expected to be different.
// Instead, we group different benchmark parameters (e.g. `rc` value) to compare/contrast their impact on performance
pub type Plots = HashMap<String, HashMap<String, Vec<Point>>>;

// TODO: Write plot data to disk as JSON
// TODO: Load plot data and add points
// TODO: Sort points by date, might not be needed
// TODO: Figure out how to include the commit hash as a label on the point or X-axis
pub fn prepare_plots(benches: &Vec<BenchData>) -> Plots {
    let mut plots: Plots = HashMap::new();

    for bench in benches {
        let point = Point {
            x: bench.id.bench_name.to_owned(),
            y: bench.result.time,
        };
        if plots.get(&bench.id.group_name).is_none() {
            plots.insert(bench.id.group_name.to_owned(), HashMap::new());
        }
        let lines = plots.get_mut(&bench.id.group_name).unwrap();
        if lines.get(&bench.id.params).is_none() {
            lines.insert(bench.id.params.to_owned(), vec![]);
        }
        lines.get_mut(&bench.id.params).unwrap().push(point);
    }
    println!("{:?}", plots);
    plots
}

//#[test]
//fn entry_point() {
//    main().unwrap()
//}

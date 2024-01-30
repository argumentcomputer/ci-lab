use plotters::prelude::*;

use chrono::{Date, TimeZone, Utc};
use serde::{Deserialize, Serialize};

use std::{collections::HashMap, error::Error};

use crate::json::BenchData;

// TODO: Figure out how to include the commit hash as a label on the point or X-axis
// TODO: Potentially account for commits on the same day by adding commit time to the benchmark ID
pub fn generate_plots(data: &Plots) -> Result<(), Box<dyn Error>> {
    for plot in data.0.iter() {
        let out_file_name = format!("./{}.png", plot.0);
        let root = BitMapBackend::new(&out_file_name, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .margin(10)
            .caption(plot.0, ("sans-serif", 40))
            .set_label_area_size(LabelAreaPosition::Left, 60)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            // TODO: Automatically adjust scales based on input
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

        // Draws the lines of benchmark data points, one line/color per set of bench ID params e.g. `rc=100`
        for (i, line) in plot.1 .0.iter().enumerate() {
            // Draw lines between each point
            chart
                .draw_series(LineSeries::new(
                    line.1.iter().map(|p| (str_to_utc(&p.x), p.y)),
                    Palette99::pick(i),
                ))?
                .label(line.0)
                // TODO: Move the legend out of the plot area
                .legend(move |(x, y)| {
                    Rectangle::new(
                        [(x - 5, y - 5), (x + 5, y + 5)],
                        Palette99::pick(i).filled(),
                    )
                });

            // Draw dots on each point
            chart.draw_series(
                line.1
                    .iter()
                    .map(|p| Circle::new((str_to_utc(&p.x), p.y), 3, Palette99::pick(i).filled())),
            )?;
            chart
                .configure_series_labels()
                .background_style(WHITE)
                .border_style(BLACK)
                .draw()?;
        }

        // To avoid the IO failure being ignored silently, we manually call the present function
        root.present().expect("Unable to write result to file");
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

// Historical benchmark result, showing the performance at a given Git commit
#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    // Commit & date associated with benchmark
    x: String,
    // Benchmark time (avg.)
    y: f64,
}

// Plots of benchmark results over time/Git history. This data structure is persistent between runs,
// saved to disk in `plot-data.json`, and is meant to be append-only to preserve historical results.
//
// Note:
// Plots are separated by benchmark input e.g. Fibonacci `num-100`. It doesn't reveal much
// information to view multiple benchmark input results on the same graph (e.g. fib-10 and fib-20),
// since they are expected to be different. Instead, we group different benchmark parameters
// (e.g. `rc` value) onto the same graph to compare/contrast their impact on performance.
#[derive(Debug, Serialize, Deserialize)]
pub struct Plots(HashMap<String, Lines>);

impl Plots {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    // Converts a list of deserialized Criterion benchmark results into plotting-friendly format
    pub fn add_data(&mut self, bench_data: &Vec<BenchData>) {
        for bench in bench_data {
            let point = Point {
                x: bench.id.bench_name.to_owned(),
                y: bench.result.time,
            };
            if self.0.get(&bench.id.group_name).is_none() {
                self.0
                    .insert(bench.id.group_name.to_owned(), Lines(HashMap::new()));
            }
            let lines = self.0.get_mut(&bench.id.group_name).unwrap();
            if lines.0.get(&bench.id.params).is_none() {
                lines.0.insert(bench.id.params.to_owned(), vec![]);
            }
            lines.0.get_mut(&bench.id.params).unwrap().push(point);
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lines(HashMap<String, Vec<Point>>);

mod data;
mod plot;

use data::read_json_from_file;

use crate::plot::{generate_plots, prepare_plots};

// TODO: Switch to camino
fn get_paths() -> std::io::Result<Vec<std::path::PathBuf>> {
    let entries = std::fs::read_dir("./history")?
        .flatten()
        .filter_map(|e| {
            let ext = e.path();
            if let Some("json") = ext.extension()?.to_str() {
                Some(ext)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    Ok(entries)
}

fn main() {
    let paths = get_paths().expect("FS read error");

    let mut bench_data = vec![];
    for path in paths {
        let mut data = read_json_from_file(path).expect("JSON serde error");
        bench_data.append(&mut data);
    }
    let plot_data = prepare_plots(&bench_data);
    generate_plots(&plot_data).unwrap();
}

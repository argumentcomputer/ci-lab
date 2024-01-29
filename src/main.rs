mod data;
mod plot;

//use data::read_json_from_file;
//use plot::plot;
use data::read_json_from_file;

use crate::plot::{generate_plots, prepare_plots};

// TODO: Switch to camino
fn get_paths() -> std::io::Result<Vec<std::path::PathBuf>> {
    let entries = std::fs::read_dir("./benches")?
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
    //let path_a = std::path::Path::new("benches/87651f2.json");
    let paths = get_paths().unwrap();
    println!("{:?}", paths[1]);

    for path in paths {
        let data = read_json_from_file(paths).unwrap();
    }
    //let bench_data = read_json_from_file(&paths[1]).unwrap();
    println!("{:?}", bench_data);
    let plot_data = prepare_plots(&bench_data);
    generate_plots(&plot_data).unwrap();
}

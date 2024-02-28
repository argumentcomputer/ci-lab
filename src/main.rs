// TODO: Unzip each tar.gz into folder, check if it has Fibonacci bench (e.g. starts with LEM), if not print name for more inspection
// If so, rewrite the ID as below using `sed`
// On error, print the filename and continue
// On success, re-archive and remove uncompressed files
use std::{process::Command};
use std::io::Read;

fn read_file() -> String {
    let hash = "3b9c434a37ed04b4f0fc048af15bde7e6536350a";
    let tar_gz = format!("{}.tar.gz", hash);
    Command::new("mkdir").args(["-p", hash]).output().expect("mkdir error");
    Command::new("tar").args(["-xvzf", &tar_gz, "-C", hash]).output().expect("tar error");
    let path = std::path::Path::new(&format!("{}/{}.json", hash, hash));


    let mut file = std::fs::File::open(path).expect("IO Error");

    let mut s = String::new();
    file.read_to_string(&mut s).expect("IO Error");
    s

}


fn main() {
    //let input="LEM Fibonacci Prove - rc = 100/fib/num-100-114c92e9-2024-02-14";
    let id: Vec<&str> = input.split('/').collect();
    println!("{:?}", id);
    let rc = id[0].split_whitespace().rev().next().unwrap();
    let name = "Prove";
    let params: Vec<&str> = id[2].split('-').collect();
    let group = format!("Fibonacci-num={}", params[1]);

    let mut commit_hash = params[2].to_owned();
    commit_hash.truncate(8);


    // let num = word after `num`
    // let rc = word after `rc`
    let mut commit_timestamp = Command::new("git").args(["show", "--no-patch", "--pretty=format:'%cI'", &commit_hash]).output().expect("Git error").stdout;
    // Remove leading and trailing `'`
    commit_timestamp.drain(..1);
    commit_timestamp.pop();
    let commit_timestamp = String::from_utf8(commit_timestamp).unwrap();

    let bench_id = format!("{}/{}/{}-{}-rc-{}", group, name, commit_hash, commit_timestamp, rc);
    println!("ID: {}", bench_id);

    let json = read_file();
    // TODO: jq script to find and replace `"id": "*LEM*"` with new 
    // E.g. `jq '. | . as o | if .id | test(".*LEM.*") then .id = "NEW_ID" else . end | o'`
    // But need to figure out how to pass a file as input
    // If that works, loop over all .tar.gz files in directory. See top-level TODOs

    // Once all this works, run the script on gh-pages for ci-lab to test
    // Then generate a fresh plot on website

    // Then use it for Lurk and Arecibo as well (note the "name" param needs to be changed to a variable not just "Prove")
    // Once the files are up to date, test the plot generation works (but don't write to gh-pages)
    // Then open PRs to Lurk and Arecibo as companions to ci-workflows

}
    

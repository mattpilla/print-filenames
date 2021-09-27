use glob::glob;
use std::fs;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let pattern = matches.value_of("pattern").unwrap();
    let output = matches.value_of("output").unwrap();
    let (filelist, total) = get_filelist(pattern);
    print_filenames(filelist, output).expect("something went wrong printing to file");
    println!("done ({} results)", total);
}

fn get_filelist(pattern: &str) -> (String, u32) {
    let mut filelist = "".to_owned();
    let mut total = 0;
    for entry in glob(pattern).expect("failed to read glob pattern") {
        match entry {
            Ok(path) => {
                filelist.push_str(path.to_str().expect("probably bad filename"));
                filelist.push('\n');
                total = total + 1;
            },
            Err(e) => println!("{:?}", e),
        }
    }
    (filelist, total)
}

fn print_filenames(filelist: String, out_path: &str) -> std::io::Result<()> {
    fs::write(out_path, filelist)?;
    Ok(())
}

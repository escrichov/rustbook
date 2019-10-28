use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let paths = fs::read_dir(&Path::new(&env::current_dir().unwrap())).unwrap();

    let names = paths
        .map(|entry| {
            let entry = entry.unwrap();

            let entry_path = entry.path();

            let file_name = entry_path.file_name().unwrap();

            let file_name_as_str = file_name.to_str().unwrap();

            let file_name_as_string = String::from(file_name_as_str);

            file_name_as_string
        })
        .collect::<Vec<String>>();

    println!("names: {:?}", names);
}

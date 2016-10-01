use std::fs;
use std::path::Path;
use util;

pub fn versions() {
    let groonga_versioned_dir = util::obtain_groonga_versioned_path();
    let paths = fs::read_dir(&Path::new(&groonga_versioned_dir)).unwrap();

    let names = paths.filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.path()
                    .file_name()
                    .and_then(|n| n.to_str().map(|s| String::from(s)))
            })
        })
        .collect::<Vec<String>>();

    println!("Installed Groonga:");
    println!("\tsystem");
    for entry in names {
        println!("\t{}", entry);
    }
}

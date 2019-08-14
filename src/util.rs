
use crate::FileFormat;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::create_dir_all;

pub fn write_file(source: &str, res: Vec<String>, level: i32, f: FileFormat) {
    
    let ext = match f {
        FileFormat::Sql => "sql",
        FileFormat::Csv => "csv",
        _ => "json"
    };

    let file_dir = format!("./data/{}", source);

    let p = Path::new(&file_dir);

    if !p.exists(){
        create_dir_all(p);
    }

    let file_name = format!("{}/areas-level{}.{}", file_dir, level, ext);

    let mut file = match File::create(file_name.clone()) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    let res_str = match f {
        FileFormat::Sql=> res.join(",\n"),
        FileFormat::Json => {
            let mut x: String = String::from("[");
            x.push_str(&res.join(",\n"));
            x.push_str("]");
            x
        },
        FileFormat::Csv => res.join("\n")
    };

    match file.write_all(res_str.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to : {}", why)
        },
        Ok(_) => {println!("successfully wrote to {}", file_name)},
    };
}
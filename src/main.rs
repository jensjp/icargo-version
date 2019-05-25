extern crate zip;

use zip::*;
use std::fs;
use std::io::*;
use std::io::BufReader;
use std::convert::TryInto;


fn main() {
    std::process::exit(real_main());
}

fn real_main() -> i32 {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return 1;
    }
    let fname = std::path::Path::new(&*args[1]);
    let zip_file = fs::File::open(&fname).unwrap();
    let reader = BufReader::with_capacity(10 * 1024 * 1024, zip_file);
    let mut archive = ZipArchive::new(reader).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = file.sanitized_name();
        let file_name = outpath.as_path().display().to_string();
        if file_name == "icargo-web.war" {
           let mut buff = Vec::with_capacity(file.size().try_into().unwrap());
           copy(&mut file, &mut buff).unwrap();
           let war_zip_file = Cursor::new(buff);
           let mut war_archive = ZipArchive::new(war_zip_file).unwrap();
           
           for j in 0..war_archive.len() {
               let mut war_file = war_archive.by_index(j).unwrap();
               let war_outpath = war_file.sanitized_name();
               let war_file_name = war_outpath.as_path().display().to_string();
               if war_file_name == "WEB-INF/web.xml" {
                   let mut buff_xml = Vec::with_capacity(war_file.size().try_into().unwrap());
                   copy(&mut war_file, &mut buff_xml).unwrap();
                   let web_xml = String::from_utf8_lossy(&buff_xml);
                   println!("{}",web_xml);
                   return 0;
               }
           }
           
        }
        
    }
    return 1;
}

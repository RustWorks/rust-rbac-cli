mod my;

use my::repl::*;
use my::util::*;
use std::str::FromStr;
use std::io::Write;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        writeln!(std::io::stderr(), 
            "Usage: {} amount_of_users amount_of_resources", args[0])
            .unwrap();
        std::process::exit(1);
    }

    let users = usize::from_str(&args[1]).unwrap();
    let rs_amount = usize::from_str(&args[2]).unwrap();
    let folder_handle = init_resource_folder("res");
    let system_files = init_resources(rs_amount, folder_handle.as_path());

    begin_loop(users, folder_handle.as_path(), &system_files)
}

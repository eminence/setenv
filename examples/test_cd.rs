extern crate tempdir;
extern crate setenv;

use setenv::get_shell;
use tempdir::TempDir;
use std::fs::create_dir;
use std::env::args;

use std::io::{Write};

fn main() {
    let args : Vec<String> = args().collect();
    //let mut stderr = std::io::stderr();
    //for a in &args {
    //    writeln!(stderr, "arg: {:?}", a);
    //}
    assert_eq!(args[1], "alpha");
    assert_eq!(args[2], "beta");
    assert_eq!(args[3], "three four");

    let s = get_shell();
    let td = TempDir::new_in(std::env::current_dir().unwrap(), "setenv_test_dir").unwrap();
    let p = td.into_path().join("test_dir_test");
    create_dir(&p).unwrap();
    s.cd(p);

}

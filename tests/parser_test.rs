extern crate mesos_statistics_publisher;

use mesos_statistics_publisher::parser;

use std::env;
use std::fs::File;
use std::io::prelude::*;

use std::path::{Path, PathBuf};

#[test]
fn it_parses_json() {
    let mut p = env::current_dir().unwrap();
    p.push("tests/data/master-state.json");
    assert_eq!(PathBuf::from("/home/cosmin/Zoo/02-rust/mesos-statistics-publisher/tests/data/master-state.json"), p);

    let mut f = File::open(p).expect("file not found");

    let mut json = String::new();
    f.read_to_string(&mut json)
        .expect("something went wrong reading the file");

    let state = parser::parse(json).unwrap();
    assert_eq!("e456dbcf-b4e2-4506-9040-21a1669152db".to_string(), state.id);

    let frameworks = state.frameworks;

    assert_eq!(1, frameworks.len());
    let marathon = &frameworks[0];

    assert_eq!("marathon", marathon.name);
}

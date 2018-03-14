extern crate mesos_statistics_publisher;

use mesos_statistics_publisher::parser;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_file(relative_path: &str) -> String {
    let mut p = env::current_dir().unwrap();
    p.push(relative_path);
    let mut f = File::open(p).expect("file not found");
    let mut json = String::new();
    f.read_to_string(&mut json).expect("something went wrong reading the file");
    json
}

#[test]
fn it_parses_json() {
    let json = read_file("tests/data/master-state.json");
    let state = parser::parse(json).unwrap();
    assert_eq!("e456dbcf-b4e2-4506-9040-21a1669152db".to_string(), state.id);

    let frameworks = state.frameworks;
    assert_eq!(1, frameworks.len());

    let marathon = &frameworks[0];
    assert_eq!("marathon", marathon.name);
    assert_eq!(0, marathon.tasks.len());

    let slaves = state.slaves;
    assert_eq!(1, slaves.len());

    let slave = &slaves[0];
    assert_eq!("e456dbcf-b4e2-4506-9040-21a1669152db-S0", slave.id);
    assert_eq!("172.17.0.6", slave.hostname);
}

#[test]
fn it_parses_tasks() {
    let json = read_file("tests/data/master-state-with-tasks.json");
    let state = parser::parse(json).unwrap();
    assert_eq!("26c5dc7e-1450-41d6-8b86-56b9af06fdc2".to_string(), state.id);

    let frameworks = state.frameworks;
    let marathon = &frameworks[0];
    assert_eq!("marathon", marathon.name);
    assert_eq!(1, marathon.tasks.len());
    assert_eq!("postgres.6c0ad1b7-26be-11e8-a7a4-0242ac110006", marathon.tasks[0].id);
    assert_eq!("26c5dc7e-1450-41d6-8b86-56b9af06fdc2-S0", marathon.tasks[0].slave_id);

    let slaves = state.slaves;
    assert_eq!(1, slaves.len());

    let slave = &slaves[0];
    assert_eq!("26c5dc7e-1450-41d6-8b86-56b9af06fdc2-S0", slave.id);
    assert_eq!("172.17.0.7", slave.hostname);
}

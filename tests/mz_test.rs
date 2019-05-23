use mz16object::object::*;
use mz16object::*;

static FILE: &[u8; 681] = include_bytes!("../resources/helloworld.exe");

#[test]
fn header() {
    let file = MZFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();
    let header = file.header();
    assert_eq!(16 * 3, header.header_data(FILE).len());
    let mut actual = String::new();
    for byte in header.header_data(FILE) {
        actual.push_str(&format!("{:02x} ", byte));
    }
    actual.push_str("\n");
    let expected = include_str!("header_data_expected.txt");
    assert_eq!(expected, actual);
}

#[test]
fn parse() {
    let file = MZFile::parse(FILE);
    assert!(file.is_ok());
}

#[test]
fn machine() {
    let file = MZFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();
    assert_eq!(Machine::X86, file.machine());
}

#[test]
fn segments() {
    let file = MZFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();

    let mut it = file.segments();
    let segment = it.next().unwrap();
}

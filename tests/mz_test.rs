use mz16object::object::{Machine, Object, ObjectSection, ObjectSegment};
use mz16object::{MZFile, MZSegment};

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
    assert_eq!(Machine::Other, file.machine());
}

#[test]
fn segments() {
    let file = MZFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();

    let mut it = file.segments();
    {
        let segment = it.next().unwrap();
        assert_eq!(0x00, segment.address());
        assert_eq!(0x1C, segment.size());
        // TODO add alignment assertion
        {
            let data: &[u8] = &[0xB8, 0x01, 0x00, 0x8E, 0xD8, 0xB4, 0x09, 0x8D, 0x16, 0x0E][..];
            assert_eq!(data, &segment.data()[0..10]);
            assert_eq!(data, segment.data_range(segment.address(), 10).unwrap());
        }
        {
            let data: &[u8] = &[0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x21, 0x24][..];
            let start_size = (segment.data().len() - 10) as usize;
            let end_size = segment.data().len();
            assert_eq!(data, &segment.data()[start_size..end_size]);
        }
        assert_eq!(None, segment.segment_name());
    }
    // Assuming there is only one segment,
    // and that the code and data segments are not different
    {
        let segment = it.next();
        assert!(segment.is_none());
    }
}

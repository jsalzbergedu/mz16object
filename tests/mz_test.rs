use mz16object::object::*;
use mz16object::*;

static FILE: &[u8; 681] = include_bytes!("../resources/helloworld.exe");

#[cfg(test)]
mod segment {
    #[test]
    fn address() {
        panic!();
    }

    #[test]
    fn size() {
        panic!();
    }

    #[test]
    fn align() {
        panic!();
    }

    #[test]
    fn data() {
        panic!();
    }

    #[test]
    fn data_range() {
        panic!();
    }

    #[test]
    fn name() {
        panic!();
    }

    #[test]
    fn next() {
        panic!();
    }
}

#[cfg(test)]
mod section {
    #[test]
    fn index() {
        panic!();
    }

    #[test]
    fn address() {
        panic!();
    }

    #[test]
    fn size() {
        panic!();
    }

    #[test]
    fn align() {
        panic!();
    }

    #[test]
    fn data() {
        panic!();
    }

    #[test]
    fn data_range() {
        panic!();
    }

    #[test]
    fn uncompressed_data() {
        panic!();
    }

    #[test]
    fn name() {
        panic!();
    }

    #[test]
    fn segment_name() {
        panic!();
    }

    #[test]
    fn kind() {
        panic!();
    }

    #[test]
    fn relocations() {
        panic!();
    }

    #[test]
    fn next() {
        panic!();
    }
}

#[cfg(test)]
mod symbol {
    #[test]
    fn next() {
        panic!();
    }
}

#[cfg(test)]
mod relocation {
    #[test]
    fn next() {
        panic!();
    }
}

#[cfg(test)]
mod file {
    use super::*;

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
}

#[cfg(test)]
mod object {
    use super::*;

    #[test]
    fn machine() {
        let file = MZFile::parse(FILE);
        assert!(file.is_ok());
        let file = file.unwrap();
        assert_eq!(Machine::X86, file.machine());
    }

    #[test]
    fn segments() {
        panic!();
    }

    #[test]
    fn section_by_name() {
        panic!();
    }

    #[test]
    fn section_by_index() {
        panic!();
    }

    #[test]
    fn sections() {
        panic!();
    }

    #[test]
    fn symbol_by_index() {
        panic!();
    }

    #[test]
    fn symbols() {
        panic!();
    }

    #[test]
    fn dynamic_symbols() {
        panic!();
    }

    #[test]
    fn symbol_map() {
        panic!();
    }

    #[test]
    fn is_little_endian() {
        panic!();
    }

    #[test]
    fn has_debug_symbols() {
        panic!();
    }

    #[test]
    fn entry() {
        panic!();
    }
}

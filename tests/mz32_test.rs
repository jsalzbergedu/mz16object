use mz16object::object::File as ObjectFile;
use mz16object::object::Machine;
#[allow(unused_imports)]
use mz16object::object::{
    Object, ObjectSection, ObjectSegment, RelocationKind, SectionIndex, SectionKind, Symbol,
    SymbolIndex, SymbolKind,
};

static FILE: &[u8; 28160] = include_bytes!("../resources/helloworld32.exe");

#[test]
fn machine() {
    let file = ObjectFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();
    assert_eq!(Machine::X86, file.machine());
}

/// Check some properties of each segment. See what its address is,
/// what its size is, what its alignment is, what its name is,
/// and how its data begins and ends.
#[test]
fn segments() {
    let file = ObjectFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();

    let mut it = file.segments();

    {
        let segment = it.next().unwrap();
        assert_eq!(0x1000, segment.address());
        assert_eq!(0x589A, segment.size());
        assert_eq!(0x1000, segment.align());
        {
            let data: &[u8] = &[0xCC, 0xEB, 0xFD, 0x90, 0x90, 0x90, 0x90, 0x00, 0x00, 0x00][..];
            assert_eq!(data, &segment.data()[0..10]);
            assert_eq!(data, segment.data_range(segment.address(), 10).unwrap());
        }
        {
            let data: &[u8] = &[0x4C, 0x71, 0x40, 0x00, 0xFF, 0x25, 0x14, 0x71, 0x40, 0x00][..];
            let start_size = (segment.data().len() - 10) as usize;
            let end_size = segment.data().len();
            assert_eq!(data, &segment.data()[start_size..end_size]);
        }
        assert_eq!(Some("AUTO"), segment.name());
    }

    {
        let segment = it.next().unwrap();
        assert_eq!(0x7000, segment.address());
        assert_eq!(0x03B6, segment.size());
        assert_eq!(0x1000, segment.align());
        {
            let data: &[u8] = &[0x3C, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..];
            assert_eq!(data, &segment.data()[0..10]);
        }
        {
            let data: &[u8] = &[0x57, 0x72, 0x69, 0x74, 0x65, 0x46, 0x69, 0x6C, 0x65, 0x00][..];
            let start_size = (segment.data().len() - 10) as usize;
            let end_size = segment.data().len();
            assert_eq!(data, &segment.data()[start_size..end_size]);
        }
        assert_eq!(Some(".idata"), segment.name());
    }

    {
        let segment = it.next().unwrap();
        assert_eq!(0x8000, segment.address());
        assert_eq!(0x1774, segment.size());
        assert_eq!(0x1000, segment.align());
        {
            let data: &[u8] = &[0x01, 0x01, 0x01, 0x00, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20][..];
            assert_eq!(data, &segment.data()[0..10]);
        }
        {
            let data: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..];
            let start_size = (segment.data().len() - 10) as usize;
            let end_size = segment.data().len();
            assert_eq!(data, &segment.data()[start_size..end_size]);
        }
        assert_eq!(Some("DGROUP"), segment.name());
    }

    {
        let segment = it.next().unwrap();
        assert_eq!(0xA000, segment.address());
        assert_eq!(0x0000, segment.size());
        assert_eq!(0x1000, segment.align());
        {
            let data: &[u8] = &[][..];
            assert_eq!(data, segment.data());
        }
        assert_eq!(Some(".reloc"), segment.name());
    }

    {
        let segment = it.next();
        assert!(segment.is_none());
    }
}

#[test]
fn sections() {
    let file = ObjectFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();

    let mut it = file.sections();
    {
        let section = it.next().unwrap();
        let SectionIndex(index) = section.index();
        assert_eq!(0x0000, index);
        assert_eq!(0x1000, section.address());
        assert_eq!(0x589A, section.size());
        assert_eq!(0x1000, section.align());
        {
            let data: &[u8] = &[0xCC, 0xEB, 0xFD, 0x90, 0x90, 0x90, 0x90, 0x00, 0x00, 0x00][..];
            assert_eq!(data, &section.data()[0..10]);
            assert_eq!(data, &section.uncompressed_data()[0..10]);
            assert_eq!(data, section.data_range(section.address(), 10).unwrap());
        }
        {
            let data: &[u8] = &[0x4C, 0x71, 0x40, 0x00, 0xFF, 0x25, 0x14, 0x71, 0x40, 0x00][..];
            let start_size = (section.data().len() - 10) as usize;
            let end_size = section.data().len();
            assert_eq!(data, &section.data()[start_size..end_size]);
        }
        assert_eq!(Some("AUTO"), section.name());
        assert_eq!(None, section.segment_name());
        assert_eq!(SectionKind::Text, section.kind());

        let mut reloc_it = section.relocations();
        assert!(reloc_it.next().is_none());
    }

    {
        let section = it.next().unwrap();
        let SectionIndex(index) = section.index();
        assert_eq!(0x0001, index);
        assert_eq!(0x7000, section.address());
        assert_eq!(0x03B6, section.size());
        assert_eq!(0x1000, section.align());
        {
            let data: &[u8] = &[0x3C, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..];
            assert_eq!(data, &section.data()[0..10]);
        }
        {
            let data: &[u8] = &[0x57, 0x72, 0x69, 0x74, 0x65, 0x46, 0x69, 0x6C, 0x65, 0x00][..];
            let start_size = (section.data().len() - 10) as usize;
            let end_size = section.data().len();
            assert_eq!(data, &section.data()[start_size..end_size]);
        }
        assert_eq!(Some(".idata"), section.name());
        assert_eq!(None, section.segment_name());
        assert_eq!(SectionKind::Data, section.kind());
        let mut reloc_it = section.relocations();
        assert!(reloc_it.next().is_none());
    }

    {
        let section = it.next().unwrap();
        let SectionIndex(index) = section.index();
        assert_eq!(0x0002, index);
        assert_eq!(0x8000, section.address());
        assert_eq!(0x1774, section.size());
        assert_eq!(0x1000, section.align());
        {
            let data: &[u8] = &[0x01, 0x01, 0x01, 0x00, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x20][..];
            assert_eq!(data, &section.data()[0..10]);
        }
        {
            let data: &[u8] = &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00][..];
            let start_size = (section.data().len() - 10) as usize;
            let end_size = section.data().len();
            assert_eq!(data, &section.data()[start_size..end_size]);
        }
        assert_eq!(Some("DGROUP"), section.name());
        assert_eq!(None, section.segment_name());
        assert_eq!(SectionKind::Data, section.kind());
        let mut reloc_it = section.relocations();
        assert!(reloc_it.next().is_none());
    }

    {
        let section = it.next().unwrap();
        let SectionIndex(index) = section.index();
        assert_eq!(0x0003, index);
        assert_eq!(0xA000, section.address());
        assert_eq!(0x0000, section.size());
        assert_eq!(0x1000, section.align());
        {
            let data: &[u8] = &[][..];
            assert_eq!(data, &*section.data());
        }
        assert_eq!(Some(".reloc"), section.name());
        assert_eq!(None, section.segment_name());
        assert_eq!(SectionKind::Data, section.kind());
        let mut reloc_it = section.relocations();
        assert!(reloc_it.next().is_none());
    }

    {
        let section = it.next();
        assert!(section.is_none());
    }
}

#[test]
fn dynamic_symbols() {
    let file = ObjectFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();

    let mut it = file.dynamic_symbols();

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0000, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("CharUpperA"), symbol.name());
        assert_eq!(0x0000, symbol.address());
        assert_eq!(0x0000, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0001, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("CloseHandle"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0002, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("CreateEventA"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0003, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("CreateFileA"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0004, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("ExitProcess"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0005, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("FlushFileBuffers"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0006, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetACP"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0007, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetCPInfo"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0008, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetCommandLineA"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0009, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetCommandLineW"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x000A, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetCurrentThreadId"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x000B, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetFileType"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x000C, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetLastError"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x000D, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetModuleFileNameA"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x000E, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetModuleFileNameW"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x000F, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetModuleHandleA"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0010, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetOEMCP"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0011, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetProcAddress"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0012, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetStdHandle"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0013, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("GetVersion"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0014, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("LoadLibraryA"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0015, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("MultiByteToWideChar"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0016, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("SetConsoleCtrlHandler"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0017, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("SetEnvironmentVariableA"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0018, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("SetFilePointer"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0019, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("SetStdHandle"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x001A, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("SetUnhandledExceptionFilter"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x001B, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("UnhandledExceptionFilter"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x001C, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("VirtualAlloc"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x001D, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("VirtualFree"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x001E, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("VirtualQuery"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x001F, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("WideCharToMultiByte"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        let (SymbolIndex(index), symbol) = it.next().unwrap();
        assert_eq!(0x0020, index);
        assert_eq!(SymbolKind::Unknown, symbol.kind());
        assert_eq!(None, symbol.section_index());
        assert_eq!(true, symbol.is_undefined());
        assert_eq!(true, symbol.is_global());
        assert_eq!(false, symbol.is_local());
        assert_eq!(Some("WriteFile"), symbol.name());
        assert_eq!(0, symbol.address());
        assert_eq!(0, symbol.size());
    }

    {
        assert!(it.next().is_none());
    }
}

#[test]
fn symbols() {
    let file = ObjectFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();

    let mut it = file.symbols();

    {
        let symbol = it.next();
        assert!(symbol.is_none());
    }
}
#[test]
fn section_by_name() {
    let file = ObjectFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();

    assert!(file.section_by_name("AUTO").is_some());
    assert!(file.section_by_name("Nope").is_none());
}

#[test]
fn section_by_index() {
    let file = ObjectFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();
    let section_index = SectionIndex(0);
    let section = file.section_by_index(section_index);
    assert!(section.is_some());
    let section = section.unwrap();
    assert_eq!(Some("AUTO"), section.name());
}

#[test]
fn symbol_by_index() {
    let file = ObjectFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();
    let symbol = file.symbol_by_index(SymbolIndex(0));
    assert!(symbol.is_none()); // odd, but its what the library reports.
}

#[test]
fn symbol_map() {
    let file = ObjectFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();
    let symbol_map = file.symbol_map();
    assert!(symbol_map.get(0).is_none());
    assert_eq!(0, symbol_map.symbols().len());
}

#[test]
fn is_little_endian() {
    let file = ObjectFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();
    assert!(file.is_little_endian());
}

#[test]
fn has_debug_symbols() {
    let file = ObjectFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();
    assert!(!file.has_debug_symbols());
}

#[test]
fn entry() {
    let file = ObjectFile::parse(FILE);
    assert!(file.is_ok());
    let file = file.unwrap();
    assert_eq!(0x10C2, file.entry());
}

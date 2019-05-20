use mz16::header::{Header, HeaderError};
use object::{
    Machine, Object, ObjectSection, ObjectSegment, Relocation, SectionIndex, SectionKind, Symbol,
    SymbolIndex, SymbolMap,
};

use std::borrow::Cow;
use std::iter::Iterator;
use std::slice;

/// An MZ exe file
#[derive(Debug)]
pub struct MZFile<'data> {
    mz: Header,
    data: &'data [u8],
}

/// An iterator over the loadable sections
/// of an mz file
#[derive(Debug)]
pub struct MZSegmentIterator<'data, 'file>
where
    'data: 'file,
{
    file: &'file MZFile<'data>,
    iter: slice::Iter<'file, &'data [u8]>,
}

/// A lodable section of an mz file
#[derive(Debug)]
pub struct MZSegment<'data, 'file>
where
    'data: 'file,
{
    file: &'file MZFile<'data>,
    segment: &'data [u8],
}

impl<'data, 'file> ObjectSegment<'data> for MZSegment<'data, 'file>
where
    'data: 'file,
{
    fn address(&self) -> u64 {
        unimplemented!();
    }

    fn size(&self) -> u64 {
        unimplemented!();
    }

    fn align(&self) -> u64 {
        unimplemented!();
    }

    fn data(&self) -> &'data [u8] {
        unimplemented!();
    }

    fn data_range(&self, address: u64, size: u64) -> Option<&'data [u8]> {
        unimplemented!();
    }

    fn name(&self) -> Option<&str> {
        unimplemented!();
    }
}

impl<'data, 'file> Iterator for MZSegmentIterator<'data, 'file>
where
    'data: 'file,
{
    type Item = MZSegment<'data, 'file>;
    fn next(&mut self) -> Option<MZSegment<'data, 'file>> {
        unimplemented!();
    }
}

/// An iterator over the sections of an MZ file
#[derive(Debug)]
pub struct MZSectionIterator<'data, 'file>
where
    'data: 'file,
{
    file: &'file MZFile<'data>,
    section: &'data [u8],
}

/// A section of an mz file
#[derive(Debug)]
pub struct MZSection<'data, 'file>
where
    'data: 'file,
{
    file: &'file MZFile<'data>,
    index: SectionIndex,
    section: &'data [u8],
}

impl<'data, 'file> Iterator for MZSectionIterator<'data, 'file>
where
    'data: 'file,
{
    type Item = MZSection<'data, 'file>;
    fn next(&mut self) -> Option<MZSection<'data, 'file>> {
        unimplemented!();
    }
}

type Dummy = u8;

/// An iterator over the symbols of an MZFile
#[derive(Debug)]
pub struct MZSymbolIterator<'data, 'file>
where
    'data: 'file,
{
    index: usize,
    exports: &'data Dummy,
    imports: &'file Dummy,
}

impl<'data, 'file> Iterator for MZSymbolIterator<'data, 'file>
where
    'data: 'file,
{
    type Item = (SymbolIndex, Symbol<'data>);
    fn next(&mut self) -> Option<(SymbolIndex, Symbol<'data>)> {
        unimplemented!();
    }
}

/// An iterator over the relocations in an MZ file
#[derive(Debug)]
pub struct MZRelocationIterator;

impl Iterator for MZRelocationIterator {
    type Item = (u64, Relocation);
    fn next(&mut self) -> Option<(u64, Relocation)> {
        unimplemented!();
    }
}

impl<'data, 'file> ObjectSection<'data> for MZSection<'data, 'file>
where
    'data: 'file,
{
    type RelocationIterator = MZRelocationIterator;
    fn index(&self) -> SectionIndex {
        unimplemented!();
    }

    fn address(&self) -> u64 {
        unimplemented!();
    }

    fn size(&self) -> u64 {
        unimplemented!();
    }

    fn align(&self) -> u64 {
        unimplemented!();
    }

    fn data(&self) -> Cow<'data, [u8]> {
        unimplemented!();
    }

    fn data_range(&self, address: u64, size: u64) -> Option<&'data [u8]> {
        unimplemented!();
    }

    fn uncompressed_data(&self) -> Cow<'data, [u8]> {
        unimplemented!();
    }

    fn name(&self) -> Option<&str> {
        unimplemented!();
    }

    fn segment_name(&self) -> Option<&str> {
        unimplemented!();
    }

    fn kind(&self) -> SectionKind {
        unimplemented!();
    }

    fn relocations(&self) -> MZRelocationIterator {
        unimplemented!();
    }
}

impl<'data> MZFile<'data> {
    /// Get the mz header
    #[inline]
    pub fn header<'a>(&'a self) -> &'a Header {
        &self.mz
    }

    /// Parse an mz file from bytes
    pub fn parse(data: &'data [u8]) -> Result<Self, HeaderError> {
        Header::new(data).map(|mz| MZFile { mz: mz, data: data })
    }
}

impl<'data, 'file> Object<'data, 'file> for MZFile<'data>
where
    'data: 'file,
{
    type Segment = MZSegment<'data, 'file>;
    type SegmentIterator = MZSegmentIterator<'data, 'file>;
    type Section = MZSection<'data, 'file>;
    type SectionIterator = MZSectionIterator<'data, 'file>;
    type SymbolIterator = MZSymbolIterator<'data, 'file>;

    fn machine(&self) -> Machine {
        unimplemented!();
    }

    fn segments(&'file self) -> MZSegmentIterator<'data, 'file> {
        unimplemented!();
    }

    fn section_by_name(&'file self, section_name: &str) -> Option<MZSection<'data, 'file>> {
        unimplemented!();
    }

    fn section_by_index(&'file self, index: SectionIndex) -> Option<MZSection<'data, 'file>> {
        unimplemented!();
    }

    fn sections(&'file self) -> MZSectionIterator<'data, 'file> {
        unimplemented!();
    }

    fn symbol_by_index(&self, index: SymbolIndex) -> Option<Symbol<'data>> {
        unimplemented!();
    }

    fn symbols(&'file self) -> MZSymbolIterator<'data, 'file> {
        unimplemented!();
    }

    fn dynamic_symbols(&'file self) -> MZSymbolIterator<'data, 'file> {
        unimplemented!();
    }

    fn symbol_map(&self) -> SymbolMap<'data> {
        unimplemented!();
    }

    #[inline]
    fn is_little_endian(&self) -> bool {
        true
    }

    fn has_debug_symbols(&self) -> bool {
        unimplemented!();
    }

    fn entry(&self) -> u64 {
        unimplemented!();
    }
}

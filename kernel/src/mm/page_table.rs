use bitflags::*;

use super::address::PhyicalPageNumber;

bitflags!{
    pub struct PTEflags: u8{
        const V = 1 << 0;
        const R = 1 << 1;
        const W = 1 << 2;
        const X = 1 << 3;
        const U = 1 << 4;
        const G = 1 << 5;
        const A = 1 << 6;
        const D = 1 << 7;
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PageTableEntry{
    pub bits: usize,
}

impl PageTableEntry {
    pub fn new(ppn:PhyicalPageNumber,flags:PTEflags) -> Self{
        PageTableEntry { bits: ppn.0 << 10 | flags.bits as usize, }
    }
    pub fn empty() -> Self{
        PageTableEntry{
            bits:0,
        }
    }

    pub fn ppn(&self) -> PhyicalPageNumber{
        (self.bits >> 10 & ((1usize << 44)-1)).into()
    }

    pub fn flags(&self) -> PTEflags{
        PTEflags::from_bits(self.bits as u8).unwrap()
    }

    pub fn is_vaild(&self) -> bool{
        (self.flags() & PTEflags::V) != PTEflags::empty()
    }

    pub fn readable(&self) -> bool{
        (self.flags() & PTEflags::R) != PTEflags::empty()
    }

    pub fn writeable(&self) -> bool{
        (self.flags() & PTEflags::W) != PTEflags::empty()
    }

    pub fn executable(&self) -> bool{
        (self.flags() & PTEflags::X) != PTEflags::empty()
    }
}

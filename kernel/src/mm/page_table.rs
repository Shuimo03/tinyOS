use super::{
    address::{PhyicalPageNumber,VirtualPageNumber,PhyicalAddress,VirtualAddress},
    frame_allocator::{FrameTracker, frame_alloc}
};
use alloc::vec::Vec;
use alloc::vec;
use alloc::string::String;
use bitflags::*;


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

    pub fn is_valid(&self) -> bool{
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

// 多级页表实现
pub struct PageTable{
    root_pnn: PhyicalPageNumber,
    frames:Vec<FrameTracker>,
}

impl PageTable {
    pub fn new() -> Self{
        let frame = frame_alloc().unwrap();
        PageTable{
            root_pnn:frame.ppn,
            frames:vec![frame],
        }
    }

    fn find_pte_create(&mut self, vpn: VirtualPageNumber) ->Option<&mut PageTableEntry>{
        let idxs = vpn.indexes();
        let mut ppn = self.root_pnn;
        let mut result : Option<&mut PageTableEntry> = None;
        for i in 0..3{
            let pte = &mut ppn.get_pte_array()[idxs[i]];
            if i == 2{
                result = Some(pte);
                break;
            }
            if !pte.is_valid(){
                let frame = frame_alloc().unwrap();
                *pte = PageTableEntry::new(frame.ppn,PTEflags::V);
                self.frames.push(frame);
            }
            ppn = pte.ppn();
        }
        result
    }

    fn find_pte(&self, vpn:VirtualPageNumber) -> Option<&mut PageTableEntry>{
        let idxs = vpn.indexes();
        let mut ppn = self.root_pnn;
        let mut result: Option<&mut PageTableEntry> = None;
        for i in 0..3{
            let pte = &mut ppn.get_pte_array()[idxs[i]];
            if i == 2{
                result = Some(pte);
                break;
            }
            if !pte.is_valid(){
                return None;
            }
            ppn = pte.ppn();
        }
        result
    }

    pub fn map(&mut self,vpn: VirtualPageNumber,ppn:PhyicalPageNumber,flags:PTEflags){
        let pte = self.find_pte_create(vpn).unwrap();
        assert!(!pte.is_valid(), "vpn {:?} is mapped before mapping",vpn);
        *pte = PageTableEntry::new(ppn, flags | PTEflags::V);
    }

    pub fn unwrap(&mut self, vpn:VirtualPageNumber){
        let pte = self.find_pte(vpn).unwrap();
        assert!(pte.is_valid(), "vpn {:?} is invalid before unmapping", vpn);
        *pte = PageTableEntry::empty();
    }

    pub fn from_token(satp: usize) -> Self{
        Self{
            root_pnn: PhyicalPageNumber::from(satp & ((1usize << 44)-1)),
            frames:Vec::new(),
        }
    }

    pub fn translate(&self, vpn: VirtualPageNumber) -> Option<PageTableEntry>{
            self.find_pte(vpn).map(|pte|{pte.clone()})
    }

}
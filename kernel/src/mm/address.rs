
use core::fmt::{self,Debug,Formatter};
use crate::config::{PAGE_SIZE,PAGE_SIZE_BITS};

use super::page_table::PageTableEntry;

const PHYICAL_ADDR_SV39_WIDTH:usize = 56; // 物理地址宽度
const VIRTUAL_ADDR_SV39_WIDTH: usize = 39; // 虚拟地址宽度
const PPN_SV39_WIDTH:usize = 44; //physical page number
const VPN_SV39_WIDTH: usize = 27; //virtual page number

//物理地址
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhyicalAddress(pub usize);

//虚拟地址
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtualAddress(pub usize);

//物理页号
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhyicalPageNumber(pub usize);

//虚拟页号
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtualPageNumber(pub usize);

impl Debug for VirtualAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        f.write_fmt(format_args!("VA:{:#x}",self.0))
    }
}

impl Debug for VirtualPageNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        f.write_fmt(format_args!("VPN:{:#x}",self.0))
    }
}

impl Debug for PhyicalAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        f.write_fmt(format_args!("PA:{:#x}",self.0))
    }
}

impl Debug for PhyicalPageNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        f.write_fmt(format_args!("PPN:{:#x}",self.0))
    }
}

impl From<usize> for PhyicalAddress {
    fn from(v: usize) -> Self { Self(v & ( (1 << PHYICAL_ADDR_SV39_WIDTH) - 1 )) }
}

impl From<usize> for PhyicalPageNumber {
    fn from(v: usize) -> Self { Self(v & ( (1 << PPN_SV39_WIDTH) - 1 )) }
}

impl From<usize> for VirtualAddress {
    fn from(v: usize) -> Self { Self(v & ( (1 << VIRTUAL_ADDR_SV39_WIDTH) - 1 )) }
}

impl From<usize> for VirtualPageNumber {
    fn from(v: usize) -> Self { Self(v & ( (1 << VPN_SV39_WIDTH) - 1 )) }
}

impl From<PhyicalAddress> for usize {
    fn from(v: PhyicalAddress) -> Self { v.0 }
}

impl From<PhyicalPageNumber> for usize {
    fn from(v: PhyicalPageNumber) -> Self { v.0 }
}

impl From<VirtualAddress> for usize {
    fn from(v: VirtualAddress) -> Self { v.0 }
}

impl From<VirtualPageNumber> for usize {
    fn from(v: VirtualPageNumber) -> Self { v.0 }
}

impl VirtualAddress {
    pub fn floor(&self) -> VirtualPageNumber{VirtualPageNumber(self.0 / PAGE_SIZE)}
    pub fn ceil(&self) -> VirtualPageNumber{VirtualPageNumber((self.0-1+PAGE_SIZE) / PAGE_SIZE)}
    pub fn page_offset(&self) -> usize{self.0 & (PAGE_SIZE - 1)}
    pub fn aligend(&self) -> bool{self.page_offset() == 0}
}

impl From<VirtualAddress> for VirtualPageNumber {
    fn from(v: VirtualAddress) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<VirtualPageNumber> for VirtualAddress {
    fn from(v: VirtualPageNumber) -> Self {
        Self(v.0 << PAGE_SIZE_BITS)
    }
}

// 地址和页号之间可以相互转换

impl PhyicalAddress {
    pub fn floor(&self) -> PhyicalPageNumber{PhyicalPageNumber(self.0 / PAGE_SIZE)}
    pub fn ceil(&self) -> PhyicalPageNumber{PhyicalPageNumber((self.0-1+PAGE_SIZE) / PAGE_SIZE)}
    pub fn page_offset(&self) -> usize{self.0 & (PAGE_SIZE - 1)}
    pub fn aligned(&self) -> bool { self.page_offset() == 0 }
}

impl From<PhyicalAddress> for PhyicalPageNumber {
    fn from(v: PhyicalAddress) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl From<PhyicalPageNumber> for PhyicalAddress {
    fn from(v: PhyicalPageNumber) -> Self {
        Self(v.0 <<  PAGE_SIZE_BITS)
    }
}

impl PhyicalPageNumber {
    // 访问固定页帧
    pub fn get_pte_array(&self)-> &'static mut[PageTableEntry]{
        let pa: PhyicalAddress = self.clone().into();
        unsafe{
            core::slice::from_raw_parts_mut(pa.0 as *mut PageTableEntry, 512)
        }
    }

    pub fn get_bytes_array(&self) -> &'static mut[u8]{
        let pa: PhyicalAddress = self.clone().into();
        unsafe{
            core::slice::from_raw_parts_mut(pa.0 as *mut u8, 4096)
        }
    }
}
//在多级页表中查找一个虚拟地址对应的页表项
impl VirtualPageNumber {
    pub fn indexes(&self) ->[usize;3]{
        let mut vpn = self.0;
        let mut idx = [0usize;3];
        for i in(0..3).rev(){
            idx[i] = vpn & 511;
            vpn >>= 9;
        }
        idx
    }
}
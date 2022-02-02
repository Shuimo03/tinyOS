
use core::fmt::{self,Debug,Formatter};
use crate::config::{PAGE_SIZE,PAGE_SIZE_BITS};

const PhyicalAddr_SV39_WIDTH:usize = 56; // 物理地址宽度
const VirtualAddr_SV39_WIDTH: usize = 39; // 虚拟地址宽度
const PPN_SV39_WIDTH:usize = 44; //physical page number
const VPN_SV39_WIDTH: usize = 27; //virtual page number

//物理地址
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhyicalAddress(pub usize);

//虚拟地址
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtualAddress(pub usize);

//物理页号
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct PhyicalNumber(pub usize);

//虚拟页号
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct VirtualNumber(pub usize);

impl Debug for VirtualAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        f.write_fmt(format_args!("VA:{:#x}",self.0))
    }
}

impl Debug for VirtualNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        f.write_fmt(format_args!("VPN:{:#x}",self.0))
    }
}

impl Debug for PhyicalAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        f.write_fmt(format_args!("PA:{:#x}",self.0))
    }
}

impl Debug for PhyicalNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        f.write_fmt(format_args!("PPN:{:#x}",self.0))
    }
}

impl From<usize> for PhyicalAddress {
    fn from(v: usize) -> Self { Self(v & ( (1 << PhyicalAddr_SV39_WIDTH) - 1 )) }
}
impl From<usize> for PhyicalNumber {
    fn from(v: usize) -> Self { Self(v & ( (1 << PPN_SV39_WIDTH) - 1 )) }
}
impl From<usize> for VirtualAddress {
    fn from(v: usize) -> Self { Self(v & ( (1 << VirtualAddr_SV39_WIDTH) - 1 )) }
}
impl From<usize> for VirtualNumber {
    fn from(v: usize) -> Self { Self(v & ( (1 << VPN_SV39_WIDTH) - 1 )) }
}
impl From<PhyicalAddress> for usize {
    fn from(v: PhyicalAddress) -> Self { v.0 }
}
impl From<PhyicalNumber> for usize {
    fn from(v: PhyicalNumber) -> Self { v.0 }
}
impl From<VirtualAddress> for usize {
    fn from(v: VirtualAddress) -> Self { v.0 }
}
impl From<VirtualNumber> for usize {
    fn from(v: VirtualNumber) -> Self { v.0 }
}

impl VirtualAddress {
    pub fn page_offset(&self) -> usize{
        self.0 & (PAGE_SIZE - 1)
    }
}

impl From<VirtualAddress> for VirtualNumber {
    fn from(v: VirtualAddress) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}

impl PhyicalAddress {
    pub fn page_offset(&self) -> usize{
        self.0 & (PAGE_SIZE - 1)
    }
}

impl From<PhyicalAddress> for PhyicalNumber {
    fn from(v: PhyicalAddress) -> Self {
        assert_eq!(v.page_offset(), 0);
        v.floor()
    }
}
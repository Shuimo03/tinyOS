//use super::{PhyicalAddress,PhyicalPageNumber};
use super::{address::PhyicalPageNumber,address::PhyicalAddress};
use alloc::vec::Vec;
use crate::sync::UPSafeCell;
use crate::config::KERNEL_END;
use lazy_static::*;
use core::fmt::{self, Debug, Formatter};

pub struct FrameTracker{
    pub ppn: PhyicalPageNumber,
}

impl FrameTracker {
    pub fn new(ppn:PhyicalPageNumber)->Self{
        let bytes_array = ppn.get_bytes_array();
        for i in bytes_array{
            *i = 0;
        }
        Self{ppn}
    }
}

impl Debug for FrameTracker {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result{
        f.write_fmt(format_args!("FrameTracker:PPN={:#x}", self.ppn.0))
    }
}

impl Drop for FrameTracker{
    fn drop(&mut self){
        frame_dealloc(self.ppn);
    }
}

// 物理页帧管理器
trait FrameAllocator {
    fn new() -> Self;
    fn alloc(&mut self)-> Option<PhyicalPageNumber>;
    fn dealloc(&mut  self, ppn: PhyicalPageNumber);
}

pub struct StackFrameAllocator{
    current: usize, // 空闲内存的起始物理页号
    end: usize, //空闲内存的结束页号
    recycled: Vec<usize>,
}

impl  StackFrameAllocator {
    pub fn init(&mut self, l:PhyicalPageNumber,r:PhyicalPageNumber){
        self.current = l.0;
        self.end =r.0;
        println!("last {} Physical Frames.", self.end - self.current);
    }
}


impl FrameAllocator for StackFrameAllocator {

    fn new() -> Self{
        Self{
            current:0,
            end:0,
            recycled:Vec::new(),
        }
  }

    /**
   * 分配之前检查栈recycled中有没有之前回收的物理页号(PhyicalPageNumber)，如果有的话之间弹出栈顶并返回,
   * 没有的话,就从之前没有分配的物理页号区间[currrent,end)上进行分配:
   *  1.分配左端点current,同时将管理器内部维护的current加一表示current 已经被分配。
   *  2.在即将返回的时候，使用into方法将usizez转换为物理页号。
   */

fn alloc(&mut self)-> Option<PhyicalPageNumber> {
    if let Some(ppn) = self.recycled.pop(){
        Some(ppn.into())
    }else{
        // 这里表示没有空间可以分配了
        if self.current == self.end{
            None
        }else{
            self.current += 1;
            Some((self.current-1).into())
        }
    }
}


/**
 * 回收页面的两个条件:
 * 1. 该页面之前一定被分配出去过,所以它的物理页号一定小于current
 * 2. 该页面没有处于正在回收状态,既它的物理页号不在栈中能够找到.
 */
fn dealloc(&mut  self, ppn: PhyicalPageNumber) {
    let ppn = ppn.0;
    if ppn > self.current || self.recycled.iter().find(|&v|{*v==ppn}).is_some(){
        panic!("Frame ppn={:#x} has not been allocated!", ppn);
    }
    self.recycled.push(ppn);
}
}

type FrameAllocatorImpl = StackFrameAllocator;
lazy_static!{
    pub static ref FRAME_ALLOCATOR:UPSafeCell<FrameAllocatorImpl> = unsafe{
        UPSafeCell::new(FrameAllocatorImpl::new())
    };
}

pub fn init_frame_allocator() {
    extern "C" {
        fn ekernel();
    }
    FRAME_ALLOCATOR
        .exclusive_access()
        .init(PhyicalAddress::from(ekernel as usize).ceil(), PhyicalAddress::from(KERNEL_END).floor());
}

pub fn frame_alloc() -> Option<FrameTracker>{
    FRAME_ALLOCATOR
    .exclusive_access()
    .alloc()
    .map(|ppn| FrameTracker::new(ppn))
}

pub fn frame_dealloc(ppn: PhyicalPageNumber){
    FRAME_ALLOCATOR
    .exclusive_access()
    .dealloc(ppn);
}

#[allow(unused)]
pub fn frame_allocator_test(){
    let mut v:Vec<FrameTracker> = Vec::new();
    for i in 0..5{
        let frame = frame_alloc().unwrap();
        println!("{:?}", frame);
        v.push(frame);
    }
    v.clear();
    for i in 0..5{
        let frame = frame_alloc().unwrap();
        println!("{:?}",frame);
        v.push(frame);
    }
    drop(v);
    println!("frame_allocator_test passed!");
}

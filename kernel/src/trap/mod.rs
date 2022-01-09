mod handler;
mod context;
mod timer;
pub use context::TrapContext;
pub fn init(){
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
}
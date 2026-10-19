use std::sync::{LazyLock, Mutex};

pub struct Memory {
    pub reg_pc: u16,
    pub reg_a: u16,
    pub reg_b: u16,
    pub reg_c: u16,

    pub memory: [u16; 65536],
    pub stack: Vec<u16>,
}

pub static MEM: LazyLock<Mutex<Memory>> = LazyLock::new(|| {
    Mutex::new(Memory {
        reg_pc: 0,
        reg_a: 0,
        reg_b: 0,
        reg_c: 0,
        memory: [0 as u16; 65536],
        stack: vec![],
    })
});

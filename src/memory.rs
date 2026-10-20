use std::sync::{LazyLock, Mutex};

#[derive(Clone)]
pub struct Memory {
    pub reg_pc: u16,
    pub reg_a: u16,
    pub reg_b: u16,
    pub reg_c: u16,

    pub memory: [u16; 65536],
    pub stack: Vec<u16>,
}

impl Memory {
    /// Update a the register specified by the arg str: &str
    /// to num: u16
    /// It's not recommended to update PC this way, use the jmp instruction instead.
    pub fn update_reg_str(&mut self, str: &str, num: u16) {
        match str {
            "a" => {
                self.reg_a = num;
            }
            "b" => {
                self.reg_b = num;
            }
            "c" => {
                self.reg_c = num;
            }
            "pc" => {
                self.reg_pc = num;
            }
            _ => {
                println!("invalid register given! pc is {}", self.reg_pc);
                std::process::exit(1);
            }
        }
    }

    pub fn get_reg_str(self, str: &str) -> u16 {
        match str {
            "a" => {
                return self.reg_a.clone();
            }
            "b" => {
                return self.reg_b.clone();
            }
            "c" => {
                return self.reg_c.clone();
            }
            "pc" => {
                return self.reg_pc.clone();
            }
            _ => {
                println!("invalid register given! pc is {}", self.reg_pc);
                std::process::exit(1);
            }
        }
    }
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

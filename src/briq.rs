use std;
use std::fmt::Display;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

// 16 bytes sequence
pub struct Briq {
    l: [u8; 8],
    h: [u8; 8],
}

impl Briq {
    pub fn new() -> Briq {
        Briq { l: [0; 8], h: [0; 8] }
    }

    fn hex_l(&self) -> String {
        let mut s = "0x".to_string();
        for i in self.l.iter() {
            s.push_str(format!("{:x}", i).as_str());
        }
        s.to_string()
    }

    fn hex_h(&self) -> String {
        let mut s = "0x".to_string();
        for i in self.h.iter() {
            s.push_str(format!("{:x}", i).as_str());
        }
        s.to_string()
    }

    fn set_l(&mut self, n: u64) {
        (&mut(self.l)[..]).write_u64::<BigEndian>(n).unwrap();
    }

    fn get_l(&self) -> u64 {
        (&(self.l)[..]).read_u64::<BigEndian>().unwrap()
    }

    fn set_h(&mut self, n: u64) {
        (&mut(self.h)[..]).write_u64::<BigEndian>(n).unwrap();
    }

    fn get_h(&self) -> u64 {
        (&(self.h)[..]).read_u64::<BigEndian>().unwrap()
    }
}

impl std::fmt::Display for Briq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", "po", "pa")
    }
}

impl std::fmt::Debug for Briq {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.hex_l(), self.hex_h())
    }
}

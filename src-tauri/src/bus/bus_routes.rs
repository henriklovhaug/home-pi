use std::sync::OnceLock;

pub static BUS: OnceLock<Bus> = OnceLock::new();

use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BusTime {
    hour: u8,
    minute: u8,
}

#[derive(Debug, Serialize)]
pub struct Bus {
    pub hour: HashMap<u8, Vec<u8>>,
}

impl Bus {
    pub fn init() -> Bus {
        let mut m = HashMap::new();
        m.insert(5, vec![49]);
        m.insert(6, vec![9, 29, 46]);
        m.insert(7, vec![1, 11, 21, 31, 43, 53]);
        m.insert(8, vec![3, 13, 23, 33, 44, 55]);
        m.insert(9, vec![4, 13, 23, 33, 48, 58]);
        m.insert(10, vec![11, 23, 33, 42, 52]);
        m.insert(11, vec![2, 12, 22, 32, 42, 52]);
        m.insert(12, vec![2, 12, 22, 32, 42, 57]);
        m.insert(13, vec![12, 22, 32, 42, 52]);
        m.insert(14, vec![2, 12, 22, 32, 48, 58]);
        m.insert(15, vec![8, 17, 27, 37, 47, 57]);
        m.insert(16, vec![7, 17, 27, 37, 53]);
        m.insert(17, vec![3, 13, 23, 33, 41, 51]);
        m.insert(18, vec![1, 16, 26, 36, 46]);
        m.insert(19, vec![1, 11, 26, 36, 46, 56]);
        m.insert(20, vec![16, 26]);
        Bus { hour: m }
    }

    pub fn get(&self, hour: u8) -> (u8, Vec<u8>) {
        if hour < 5 {
            return (5, self.hour.get(&5).unwrap().clone());
        } else if hour > 20 {
            return (20, self.hour.get(&20).unwrap().clone());
        } else {
            return (hour, self.hour.get(&hour).unwrap().clone());
        }
    }

    pub fn get_next_n(&self, hour: u8, minute: u8, n: u8) -> Vec<BusTime> {
        let (hour, minutes) = self.get(hour);
        let mut result = Vec::new();
        for m in minutes {
            if m > minute {
                result.push(BusTime { hour, minute: m });
            }
        }
        let mut i = 1;
        while result.len() < n as usize {
            let (hour, minutes) = self.get(hour + i);
            for m in minutes {
                result.push(BusTime { hour, minute: m });
            }
            i += 1;
        }
        result.truncate(n as usize);
        result
    }
}

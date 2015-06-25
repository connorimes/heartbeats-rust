use libc::{uint64_t, int64_t, c_void, c_char, c_double};
use std::ffi::CString;
use std::ptr;
use energymon_sys::energymon::EnergyMon;

#[link(name = "hbt-acc-pow")]
extern {
    fn heartbeat_acc_pow_init(parent: *mut c_void,
                              window_size: uint64_t,
                              buffer_depth: uint64_t,
                              log_name: *const c_char,
                              num_energy_impls: uint64_t,
                              energy_impls: *mut EnergyMon) -> *mut c_void;

    fn heartbeat_acc(hb: *mut c_void,
                     user_tag: uint64_t,
                     work: uint64_t,
                     accuracy: c_double,
                     hb_prev: *mut c_void) -> int64_t;

    fn heartbeat_finish(hb: *mut c_void);
}

pub struct Heartbeat {
    pub hb: *mut c_void,
}

impl Heartbeat {
    pub fn new(parent: Option<&mut Heartbeat>,
               window_size: u64,
               buffer_depth: u64, 
               log_name: &str,
               energy_impl: &mut EnergyMon) -> Result<Heartbeat, String> {
        let parent = match parent {
            Some(p) => p.hb,
            None => ptr::null_mut(),
        };
        let heart = unsafe {
            heartbeat_acc_pow_init(parent, window_size, buffer_depth,
                                   CString::new(log_name).unwrap().as_ptr(),
                                   1, energy_impl)
        };
        if heart.is_null() {
            return Err("Failed to initialize heartbeat".to_string());
        }
        Ok(Heartbeat { hb: heart, })
    }

    pub fn heartbeat(&mut self,
                     tag: u64,
                     work: u64,
                     accuracy: f64,
                     hb_prev: Option<&Heartbeat>) -> i64 {
        let hb_prev = match hb_prev {
            Some(p) => p.hb,
            None => ptr::null_mut(),
        };
        unsafe {
            heartbeat_acc(self.hb, tag, work, accuracy, hb_prev)
        }
    }
}

impl Drop for Heartbeat {
    fn drop(&mut self) {
        unsafe {
            heartbeat_finish(self.hb);
        }
    }
}


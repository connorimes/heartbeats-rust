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

    fn hb_get_user_tag(hb: *const c_void) -> uint64_t;

    fn hb_get_window_rate(hb: *const c_void) -> c_double;

    fn hb_get_window_power(hb: *const c_void) -> c_double;
}

/// A `Heartbeat` is used for tracking performance/accuracy/power of recurring jobs.
pub struct Heartbeat {
    /// The underlying C struct `heartbeat_t`.
    pub hb: *mut c_void,
}

impl Heartbeat {
    /// Allocate and initialize a new `Heartbeat` with its underlying C struct.
    pub fn new(parent: Option<&mut Heartbeat>,
               window_size: u64,
               buffer_depth: u64, 
               log_name: Option<&CString>,
               energy_impl: Option<&mut EnergyMon>) -> Result<Heartbeat, &'static str> {
        let parent = match parent {
            Some(p) => p.hb,
            None => ptr::null_mut(),
        };
        let log_ptr = match log_name {
          Some(n) => n.as_ptr(),
          None => ptr::null(),
        };
        let num_energy_impls: u64;
        let mut em: *mut EnergyMon;
        if energy_impl.is_some() {
            num_energy_impls = 1;
            em = energy_impl.unwrap();
        } else {
            num_energy_impls = 0;
            em = ptr::null_mut();
        }
        let heart = unsafe {
            heartbeat_acc_pow_init(parent, window_size, buffer_depth, log_ptr,
                                   num_energy_impls, em)
        };
        if heart.is_null() {
            return Err("Failed to initialize heartbeat");
        }
        Ok(Heartbeat { hb: heart, })
    }

    /// Issue a heartbeat.
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

    /// Utility function to get the most recent user-specified tag
    pub fn get_tag(&mut self) -> u64 {
        unsafe {
            hb_get_user_tag(self.hb)
        }
    }

    /// Utility function to get the current window performance.
    pub fn get_window_perf(&mut self) -> f64 {
        unsafe {
            hb_get_window_rate(self.hb)
        }
    }

    /// Utility function to get the current window power.
    pub fn get_window_pwr(&mut self) -> f64 {
        unsafe {
            hb_get_window_power(self.hb)
        }
    }
}

impl Drop for Heartbeat {
    /// Cleans up and deallocates the underlying C struct.
    fn drop(&mut self) {
        unsafe {
            heartbeat_finish(self.hb);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Heartbeat;

    #[test]
    fn test_no_energymon() {
        let mut hb = Heartbeat::new(None, 20, 20, None, None).unwrap();
        hb.heartbeat(0, 1, 0.0, None);
    }
}

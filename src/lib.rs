extern crate libc;

pub mod heartbeat;

use libc::{uint64_t, int64_t, c_void, c_char, c_double, c_longlong};

pub type HeartbeatReadEnergyFn = extern fn(*mut c_void) -> c_longlong;

#[link(name = "hbt-acc-pow")]
extern {
    pub fn heartbeat_acc_pow_init(parent: *mut c_void,
	                              window_size: uint64_t,
	                              buffer_depth: uint64_t,
	                              log_name: *const c_char,
	                              read_energy_fn: Option<HeartbeatReadEnergyFn>,
	                              ref_arg: *mut c_void) -> *mut c_void;

    pub fn heartbeat_acc(hb: *mut c_void,
	                     user_tag: uint64_t,
	                     work: uint64_t,
	                     accuracy: c_double,
	                     hb_prev: *mut c_void) -> int64_t;

    pub fn heartbeat_finish(hb: *mut c_void);

    pub fn hb_get_user_tag(hb: *const c_void) -> uint64_t;

    pub fn hb_get_window_rate(hb: *const c_void) -> c_double;

    pub fn hb_get_window_power(hb: *const c_void) -> c_double;
}

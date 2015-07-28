extern crate libc;

pub mod heartbeat;

use libc::{uint64_t, int64_t, c_void, c_char, c_double, c_longlong};

#[repr(C)]
pub type hb_get_energy_func = extern fn(*mut c_void) -> c_longlong;

#[repr(C)]
pub struct heartbeat_time_data {
  pub last_timestamp: int64_t,
  pub total_time: int64_t,
  pub window_time: int64_t,
}

#[repr(C)]
pub struct _heartbeat_work_data {
  pub total_work: uint64_t,
  pub window_work: uint64_t,
}

#[repr(C)]
pub struct _heartbeat_accuracy_data {
  pub total_accuracy: c_double,
  pub window_accuracy: c_double,
}

#[repr(C)]
pub struct _heartbeat_energy_data {
  pub last_energy: c_double,
  pub total_energy: c_double,
  pub window_energy: c_double,
}

#[repr(C)]
pub struct heartbeat_record_t {
  pub id: uint64_t,
  pub shared_id: uint64_t,
  pub user_tag: uint64_t,
  pub timestamp: uint64_t,

  pub work: uint64_t,
  pub latency: int64_t,
  pub global_perf: c_double,
  pub window_perf: c_double,
  pub instant_perf: c_double,

  pub accuracy: c_double,
  pub global_acc: c_double,
  pub window_acc: c_double,
  pub instant_acc: c_double,

  pub energy: c_double,
  pub global_pwr: c_double,
  pub window_pwr: c_double,
  pub instant_pwr: c_double,
}

#[repr(C)]
pub struct _heartbeat_shared_data {
  pub valid: c_char,
  pub counter: uint64_t,
  // pub mutex: pthread_mutex_t;

  // data
  pub td: heartbeat_time_data,
}

#[repr(C)]
pub struct _heartbeat_local_data {
  pub valid: c_char,
  pub counter: uint64_t,
  pub ef: *const hb_get_energy_func,
  pub ref_arg: *mut c_void,

  // data
  pub td: heartbeat_time_data,		
  pub wd: _heartbeat_work_data,
  pub ad: _heartbeat_accuracy_data,
  pub ed: _heartbeat_energy_data,

  // logging
  pub text_file: *mut c_void,
  pub log: *mut heartbeat_record_t,
  pub buffer_depth: uint64_t,
  pub buffer_index: uint64_t,
  pub read_index: uint64_t,
}

#[repr(C)]
pub struct heartbeat_t {
  pub parent: *mut heartbeat_t,
  pub window_size: uint64_t,
  pub sd: *mut _heartbeat_shared_data,
  pub ld: _heartbeat_local_data,
}

extern "C" {
    pub fn heartbeat_acc_pow_init(parent: *mut heartbeat_t,
	                              window_size: uint64_t,
	                              buffer_depth: uint64_t,
	                              log_name: *const c_char,
	                              read_energy_fn: Option<hb_get_energy_func>,
	                              ref_arg: *mut c_void) -> *mut heartbeat_t;

    pub fn heartbeat_acc(hb: *mut heartbeat_t,
	                     user_tag: uint64_t,
	                     work: uint64_t,
	                     accuracy: c_double,
	                     hb_prev: *mut heartbeat_t) -> int64_t;

    pub fn heartbeat_finish(hb: *mut heartbeat_t);

    pub fn hb_get_user_tag(hb: *const heartbeat_t) -> uint64_t;

    pub fn hb_get_window_rate(hb: *const heartbeat_t) -> c_double;

    pub fn hb_get_window_power(hb: *const heartbeat_t) -> c_double;
}

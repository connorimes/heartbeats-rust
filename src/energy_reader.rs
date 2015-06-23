use libc::{self, c_void};

//#[cfg_attr(hb_energy_impl_msr, link(name = "hb-energy-msr"))]
extern {
    fn hb_energy_impl_alloc() -> *mut c_void;
}

pub struct EnergyReader {
    pub er: *mut c_void,
}

impl EnergyReader {
    pub fn new() -> Result<EnergyReader, String> {
        let reader = unsafe {
            hb_energy_impl_alloc()
        };
        if reader.is_null() {
            return Err("Failed to allocate energy reader".to_string());
        }
        Ok(EnergyReader { er: reader, })
    }
}

impl Drop for EnergyReader {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.er);
        }
    }
}


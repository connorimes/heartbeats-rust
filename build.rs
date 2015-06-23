fn main() {
    // println!("cargo:rustc-cfg=hb_energy_impl_msr");
    println!("cargo:rustc-link-lib=hb-energy-msr");
}

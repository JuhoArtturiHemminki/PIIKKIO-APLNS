pub unsafe fn run(pic: *mut u64, wd: *mut u32, lut: &[u64]) -> ! {
    for (i, &val) in lut.iter().cycle().enumerate() {
        let mut t = 0;
        while core::ptr::read_volatile(pic) & 1 == 0 {
            t += 1; if t > 1e7 as u32 { panic!() }
        }
        core::ptr::write_volatile(pic, val);
        core::ptr::write_volatile(wd, 0xAAAA_5555);
    }
    loop {}
}

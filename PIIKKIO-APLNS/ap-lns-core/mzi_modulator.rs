pub struct MZIModulator {
    address: *mut u64,
    phi: f64,
}

impl MZIModulator {
    pub fn new(addr: usize) -> Self {
        Self {
            address: addr as *mut u64,
            phi: 1.61803398875,
        }
    }

    pub unsafe fn set_phase_inversion(&self, wave_amplitude: f64) {
        let anti_phase = (wave_amplitude * self.phi).to_bits();
        std::ptr::write_volatile(self.address, anti_phase);
    }
}


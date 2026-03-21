#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{compiler_fence, Ordering};

pub struct VAxionDriver {
    pic_reg: *mut u64,
    watchdog: *mut u32,
    phi_constant: f64,
}

impl VAxionDriver {
    pub const unsafe fn new(pic_addr: usize, wd_addr: usize) -> Self {
        Self {
            pic_reg: pic_addr as *mut u64,
            watchdog: wd_addr as *mut u32,
            phi_constant: 1.61803398875,
        }
    }

    #[inline(always)]
    fn reset_watchdog(&self) {
        unsafe { write_volatile(self.watchdog, 0xAAAA_5555) };
    }

    #[inline(always)]
    fn wait_for_ready(&self) -> Result<(), ()> {
        let mut timeout = 0u32;
        while unsafe { read_volatile(self.pic_reg) } & 1 == 0 {
            timeout += 1;
            if timeout > 10_000_000 {
                return Err(());
            }
            compiler_fence(Ordering::SeqCst);
        }
        Ok(())
    }

    #[inline(always)]
    pub fn calculate_brillouin_shift(&self, freq: f64, vel: f64, n: f64) -> f64 {
        let c = 299_792_458.0;
        (2.0 * n * vel * freq * 0.70710678118) / c
    }

    #[inline(always)]
    pub fn landauer_energy_limit(&self, temp: f64, bits: f64) -> f64 {
        let k_b = 1.380649e-23;
        let ln2 = 0.69314718056;
        k_b * temp * ln2 * bits
    }

    pub unsafe fn run_optimized(&self, lut: &[u64], temp_k: f64) -> ! {
        let mut drift_accumulator: f64 = 0.0;
        
        for &val in lut.iter().cycle() {
            if self.wait_for_ready().is_err() {
                panic!("PIC_TIMEOUT");
            }

            let correction = (drift_accumulator * self.phi_constant).sin();
            let final_val = val ^ correction.to_bits();

            write_volatile(self.pic_reg, final_val);
            self.reset_watchdog();

            drift_accumulator += 0.0000000012;
            if drift_accumulator > 6.28318530718 {
                drift_accumulator -= 6.28318530718;
            }
        }
        loop { compiler_fence(Ordering::SeqCst); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let driver = VAxionDriver::new(0xFF001000, 0xFE000000);
    let static_lut: [u64; 4] = [0x1, 0x2, 0x4, 0x8];
    driver.run_optimized(&static_lut, 300.0);
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop { compiler_fence(Ordering::SeqCst); }
}

#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{compiler_fence, Ordering};

const PHI_Q16: u64 = 106039;
const WATCHDOG_RELOAD: u32 = 0xAAAA_5555;

pub struct VAxionSystem {
    base_addr: usize,
    drift: u64,
}

impl VAxionSystem {
    pub const fn new(base: usize) -> Self {
        Self {
            base_addr: base,
            drift: 0,
        }
    }

    #[inline(always)]
    pub fn update_optical_phase(&mut self) {
        let correction = (self.drift.wrapping_mul(PHI_Q16)) >> 16;
        unsafe {
            write_volatile((self.base_addr + 0x3000) as *mut u64, correction);
        }
        self.drift = self.drift.wrapping_add(1);
    }

    #[inline(always)]
    pub unsafe fn transfer_frame(&self, data: &[u8]) -> Result<(), ()> {
        let status_reg = (self.base_addr + 0x1008) as *const u32;
        let dma_buffer = (self.base_addr + 0x4000) as *mut u8;
        let watchdog = (self.base_addr + 0x2000) as *mut u32;

        let mut timeout = 0u32;
        while (read_volatile(status_reg) & 0x1) == 0 {
            timeout += 1;
            if timeout > 1_000_000 {
                return Err(());
            }
            compiler_fence(Ordering::SeqCst);
        }

        core::ptr::copy_nonoverlapping(data.as_ptr(), dma_buffer, data.len());
        write_volatile(watchdog, WATCHDOG_RELOAD);
        
        Ok(())
    }

    #[inline(always)]
    pub fn calculate_brillouin_fixed(&self, freq_mhz: u64, vel_ms: u64) -> u64 {
        let n_q16 = 144179; 
        let c = 299_792_458;
        (2 * n_q16 * vel_ms * freq_mhz) / c
    }
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let mut system = VAxionSystem::new(0xFE000000);
    let static_data = [0x55; 1024];

    loop {
        system.update_optical_phase();
        let _ = system.transfer_frame(&static_data);
        compiler_fence(Ordering::SeqCst);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        compiler_fence(Ordering::SeqCst);
    }
}

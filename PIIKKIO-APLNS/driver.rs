#![no_std]
#![no_main]

use core::ptr::{read_volatile, write_volatile};
use core::sync::atomic::{compiler_fence, Ordering};

const PHI_Q16: u64 = 106039;
const WATCHDOG_RELOAD: u32 = 0xAAAA_5555;
const BASE_ADDR: usize = 0xFE000000;
const DMA_BUFFER_SIZE: usize = 1024;

#[link_section = ".data"]
static mut DMA_FRAME_BUFFER: [u8; DMA_BUFFER_SIZE] = [0; DMA_BUFFER_SIZE];

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
            let _ = read_volatile((self.base_addr + 0x3000) as *const u64);
        }
        self.drift = self.drift.wrapping_add(1);
    }

    #[inline(always)]
    pub unsafe fn transfer_frame(&self, data: &[u8]) -> Result<(), ()> {
        if data.len() > DMA_BUFFER_SIZE {
            return Err(());
        }

        let status_reg = (self.base_addr + 0x1008) as *const u32;
        let dma_buffer = &mut DMA_FRAME_BUFFER as *mut [u8; DMA_BUFFER_SIZE] as *mut u8;
        let watchdog = (self.base_addr + 0x2000) as *mut u32;

        let mut timeout = 0u32;
        while (read_volatile(status_reg) & 0x1) == 0 {
            timeout = timeout.saturating_add(1);
            if timeout > 1_000_000 {
                return Err(());
            }
            compiler_fence(Ordering::SeqCst);
        }

        core::ptr::copy_nonoverlapping(data.as_ptr(), dma_buffer, data.len());
        
        compiler_fence(Ordering::SeqCst);
        write_volatile(watchdog, WATCHDOG_RELOAD);
        
        Ok(())
    }

    #[inline(always)]
    pub fn calculate_brillouin_fixed(&self, freq_mhz: u64, vel_ms: u64) -> u64 {
        let n_q16 = 144179u128; 
        let c = 299_792_458u128;
        ((2 * n_q16 * vel_ms as u128 * freq_mhz as u128) / c) as u64
    }
}

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let mut system = VAxionSystem::new(BASE_ADDR);
    let static_data = [0x55; DMA_BUFFER_SIZE];

    loop {
        system.update_optical_phase();
        if let Err(_) = system.transfer_frame(&static_data) {
            let reset_reg = (BASE_ADDR + 0x1000) as *mut u32;
            write_volatile(reset_reg, 0x1);
        }
        compiler_fence(Ordering::SeqCst);
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        let safe_state_reg = (BASE_ADDR + 0x5000) as *mut u32;
        write_volatile(safe_state_reg, 0xDEAD);
    }
    loop {
        compiler_fence(Ordering::SeqCst);
    }
}

use std::ptr;

pub struct DirectDMALink {
    buffer: *mut u8,
    size: usize,
}

impl DirectDMALink {
    pub unsafe fn map_device(phys_addr: usize, len: usize) -> Self {
        Self {
            buffer: phys_addr as *mut u8,
            size: len,
        }
    }

    pub unsafe fn push_frame(&self, data: &[u8]) {
        ptr::copy_nonoverlapping(data.as_ptr(), self.buffer, 
data.len().min(self.size));
    }
}


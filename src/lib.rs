//!
//! # lscpu
//!
//! Implementation of [lscpu](https://www.man7.org/linux/man-pages/man1/lscpu.1.html) in rust
//!
//! ## Run a std example
//!
//! ```
//! cargo run --example std
//! ```
//!
//! This code can be runned also in a `no-std` environment.
//!
//! ## Cpu Data:
//!
//! ```rust
//! pub struct Cpu {
//!     pub architecture: &'static str,
//!     pub cpu_op_modes: &'static str,
//!     pub address_sizes: String,
//!     pub byte_order: &'static str,
//!     pub cpu_count: u32,
//!     pub on_line_cpu: u32,
//!     pub vendor_id: String,
//!     pub model_name: String,
//!     pub cpu_family: u32,
//!     pub cpu_model: u32,
//!     pub is_hybrid: &'static str,
//!     pub threads_per_core: u32,
//!     pub cores_per_socket: u32,
//!     pub sockets: u32,
//!     pub stepping: u32,
//!    pub boost_enabled: &'static str,
//! }
//! ```
//!

#![no_std]

extern crate alloc;
use paste::paste;

use alloc::string::{String, ToString};
use core::{arch::asm, clone::Clone, default::Default, fmt::Write, write};

pub struct Cpu {
    pub architecture: &'static str,
    pub cpu_op_modes: &'static str,
    pub address_sizes: String,
    pub byte_order: &'static str,
    pub cpu_count: u32,
    pub on_line_cpu: u32,
    pub vendor_id: String,
    pub model_name: String,
    pub cpu_family: u32,
    pub cpu_model: u32,
    pub is_hybrid: &'static str,
    pub threads_per_core: u32,
    pub cores_per_socket: u32,
    pub sockets: u32,
    pub stepping: u32,
    pub boost_enabled: &'static str,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            architecture: get_cpu_architecture(),
            cpu_op_modes: get_cpu_op_modes(),
            address_sizes: get_address_sizes(),
            byte_order: get_byte_order(),
            cpu_count: get_cpu_count(),
            on_line_cpu: get_on_line_cpu(),
            vendor_id: get_vendor_id(),
            model_name: get_model_name(),
            cpu_family: get_cpu_family(),
            cpu_model: get_cpu_model(),
            threads_per_core: get_threads_per_core(),
            cores_per_socket: get_cores_per_socket(),
            sockets: get_sockets(),
            stepping: get_stepping(),
            boost_enabled: get_boost_enabled(),
            is_hybrid: get_hybrid_flag(),
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for Cpu {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "Architecture:             {}\n\
             CPU op-mode(s):           {}\n\
             Address sizes:            {}\n\
             Byte Order:               {}\n\
             CPU(s):                   {}\n\
             On-line CPU(s) list:      0,{}\n\
             Vendor ID:                {}\n\
             Model name:               {}\n\
             CPU family:               {}\n\
             Model:                    {}\n\
             Is hybrid:                {}\n\
             Thread(s) per core:       {}\n\
             Core(s) per socket:       {}\n\
             Socket(s):                {}\n\
             Stepping:                 {}\n\
             Frequency boost:          {}\n
            ",
            self.architecture,
            self.cpu_op_modes,
            self.address_sizes,
            self.byte_order,
            self.cpu_count,
            self.on_line_cpu,
            self.vendor_id,
            self.model_name,
            self.cpu_family,
            self.cpu_model,
            self.is_hybrid,
            self.threads_per_core,
            self.cores_per_socket,
            self.sockets,
            self.stepping,
            self.boost_enabled
        )
    }
}

macro_rules! generate_getters {
    ($struct_name:ident, $( $field:ident : $field_type:ty ),* ) => {
        impl $struct_name {
            $(
                paste! {
                    pub fn [<get_ $field>](&self) -> $field_type {
                        self.$field.clone()
                    }
                }
            )*
        }
    };
}

generate_getters!(Cpu,
    architecture: &'static str,
    cpu_op_modes: &'static str,
    address_sizes: String,
    byte_order: &'static str,
    cpu_count: u32,
    on_line_cpu: u32,
    vendor_id: String,
    model_name: String,
    cpu_family: u32,
    cpu_model: u32,
    is_hybrid: &'static str,
    threads_per_core: u32,
    cores_per_socket: u32,
    sockets: u32,
    stepping: u32,
    boost_enabled: &'static str
);

fn get_cpu_architecture() -> &'static str {
    let mut cpuid_info: [u32; 4] = [0; 4];
    unsafe {
        asm!(
            "cpuid",
            inout("eax") 0x80000000u32 as i32 => cpuid_info[0],
            lateout("ecx") cpuid_info[2],
            lateout("edx") cpuid_info[3],
        );
    }
    if cpuid_info[0] < 0x80000001 {
        return "x86";
    }
    unsafe {
        asm!(
            "cpuid",
            inout("eax") 0x80000001u32 as i32 => cpuid_info[0],
            lateout("ecx") cpuid_info[2],
            lateout("edx") cpuid_info[3],
        );
    }
    if cpuid_info[3] & (1 << 29) != 0 {
        "x86_64"
    } else {
        "x86"
    }
}

fn get_cpu_op_modes() -> &'static str {
    let mut cpuid_info: [u32; 4] = [0; 4];

    unsafe {
        asm!(
            "cpuid",
            inout("eax") 0x80000000u32 as i32 => cpuid_info[0],
            lateout("ecx") cpuid_info[2],
            lateout("edx") cpuid_info[3],
        );
    }

    if cpuid_info[0] < 0x80000001 {
        return "32-bit";
    }

    unsafe {
        asm!(
            "cpuid",
            inout("eax") 0x80000001u32 as i32 => cpuid_info[0],
            lateout("ecx") cpuid_info[2],
            lateout("edx") cpuid_info[3],
        );
    }

    if cpuid_info[3] & (1 << 29) != 0 {
        "32-bit, 64-bit"
    } else {
        "32-bit"
    }
}

fn get_byte_order() -> &'static str {
    let value: u16 = 0x0001;
    let bytes = value.to_ne_bytes();
    if bytes[0] == 0x01 {
        "Little Endian"
    } else {
        "Big Endian"
    }
}

fn get_address_sizes() -> String {
    let mut cpuid_info: [u32; 4] = [0; 4];
    unsafe {
        asm!(
            "cpuid",
            inout("eax") 0x80000008u32 as i32 => cpuid_info[0],
            lateout("ecx") cpuid_info[2],
            lateout("edx") cpuid_info[3],
        );
    }
    let physical_size = cpuid_info[0] & 0xFF;
    let virtual_size = (cpuid_info[0] >> 8) & 0xFF;
    let mut result = String::new();
    write!(
        &mut result,
        "{} bits physical, {} bits virtual",
        physical_size, virtual_size
    )
    .unwrap_or(());
    result
}

fn get_cpu_count() -> u32 {
    let mut _eax: u32;
    let mut _ebx: u32;
    let mut _ecx: u32;
    let mut _edx: u32;

    unsafe {
        _eax = 0x0B;
        _ecx = 0;
        asm!(
            "cpuid",
            inout("eax") _eax,
            lateout("ecx") _ecx,
            lateout("edx") _edx,
        );
    }
    unsafe {
        asm!(
            "mov eax, ebx",
            out("eax") _ebx,
            options(nostack, nomem, preserves_flags),
        );
    }

    if _eax != 0 {
        return _ebx;
    }

    unsafe {
        _eax = 0x04;
        _ecx = 0;
        asm!(
            "cpuid",
            inout("eax") _eax,
            lateout("ecx") _ecx,
            lateout("edx") _edx,
        );
    }
    unsafe {
        asm!(
            "mov eax, ebx",
            out("eax") _ebx,
            options(nostack, nomem, preserves_flags),
        );
    }

    if _eax != 0 {
        return ((_eax >> 26) & 0x3F) + 1;
    }

    unsafe {
        _eax = 0x01;
        asm!(
            "cpuid",
            inout("eax") _eax,
            lateout("ecx") _ecx,
            lateout("edx") _edx,
        );
    }
    unsafe {
        asm!(
            "mov eax, ebx",
            out("eax") _ebx,
            options(nostack, nomem, preserves_flags),
        );
    }

    if (_edx & (1 << 28)) != 0 {
        return (_ebx >> 16) & 0xFF;
    }

    1
}

fn get_on_line_cpu() -> u32 {
    let eax: u32 = 1;
    let mut _ebx: u32 = 0;
    let mut _ecx: u32 = 0;
    let mut _edx: u32 = 0;

    unsafe {
        asm!(
            "cpuid",
            inout("eax") eax => _,
            out("ecx") _ecx,
            out("edx") _edx
        );
    }

    eax
}

pub fn get_vendor_id() -> String {
    let mut _eax: u32 = 0;
    let mut _ecx: u32 = 0;
    let mut _edx: u32 = 0;

    unsafe {
        asm!(
            "cpuid",
            in("eax") 0,
            lateout("eax") _eax,
            lateout("ecx") _ecx,
            lateout("edx") _edx,
            options(nostack, nomem, preserves_flags),
        );

        asm!(
            "mov eax, ebx",
            out("eax") _eax,
            options(nostack, nomem, preserves_flags),
        );
    }

    let mut vendor_id = [0u8; 12];
    vendor_id[0..4].copy_from_slice(&_eax.to_le_bytes());
    vendor_id[4..8].copy_from_slice(&_edx.to_le_bytes());
    vendor_id[8..12].copy_from_slice(&_ecx.to_le_bytes());

    String::from_utf8(vendor_id.to_vec()).unwrap_or("Unknown".to_string())
}

pub fn get_model_name() -> String {
    let mut model_name = [0u8; 48];
    let mut cpuid_info: [u32; 4] = [0; 4];

    unsafe {
        asm!(
            "cpuid",
            inout("eax") 0x80000002u32 as i32 => cpuid_info[0],
            lateout("ecx") cpuid_info[2],
            lateout("edx") cpuid_info[3],
            options(nostack, nomem, preserves_flags),
        );
    }

    unsafe {
        asm!(
            "mov eax, ebx",
            out("eax") cpuid_info[1],
            options(nostack, nomem, preserves_flags),
        );
    }

    model_name[0..4].copy_from_slice(&cpuid_info[0].to_le_bytes());
    model_name[4..8].copy_from_slice(&cpuid_info[1].to_le_bytes());
    model_name[8..12].copy_from_slice(&cpuid_info[2].to_le_bytes());
    model_name[12..16].copy_from_slice(&cpuid_info[3].to_le_bytes());

    unsafe {
        asm!(
            "cpuid",
            inout("eax") 0x80000003u32 as i32 => cpuid_info[0],
            lateout("ecx") cpuid_info[2],
            lateout("edx") cpuid_info[3],
            options(nostack, nomem, preserves_flags),
        );
    }

    unsafe {
        asm!(
            "mov eax, ebx",
            out("eax") cpuid_info[1],
            options(nostack, nomem, preserves_flags),
        );
    }

    model_name[16..20].copy_from_slice(&cpuid_info[0].to_le_bytes());
    model_name[20..24].copy_from_slice(&cpuid_info[1].to_le_bytes());
    model_name[24..28].copy_from_slice(&cpuid_info[2].to_le_bytes());
    model_name[28..32].copy_from_slice(&cpuid_info[3].to_le_bytes());

    unsafe {
        asm!(
            "cpuid",
            inout("eax") 0x80000004u32 as i32 => cpuid_info[0],
            lateout("ecx") cpuid_info[2],
            lateout("edx") cpuid_info[3],
            options(nostack, nomem, preserves_flags),
        );
    }
    unsafe {
        asm!(
            "mov eax, ebx",
            out("eax") cpuid_info[1],
            options(nostack, nomem, preserves_flags),
        );
    }
    model_name[32..36].copy_from_slice(&cpuid_info[0].to_le_bytes());
    model_name[36..40].copy_from_slice(&cpuid_info[1].to_le_bytes());
    model_name[40..44].copy_from_slice(&cpuid_info[2].to_le_bytes());
    model_name[44..48].copy_from_slice(&cpuid_info[3].to_le_bytes());

    String::from_utf8(model_name.to_vec()).unwrap_or("Unknown".to_string())
}

fn get_cpu_family() -> u32 {
    let mut eax: u32;
    let mut _ebx: u32;
    let mut _ecx: u32;
    let mut _edx: u32;

    unsafe {
        asm!(
            "cpuid",
            inout("eax") 0x01 => eax,
            lateout("ecx") _ecx,
            lateout("edx") _edx,
        );
    }

    unsafe {
        asm!(
            "mov eax, ebx",
            out("eax") _ebx,
            options(nostack, nomem, preserves_flags),
        );
    }

    let base_family = (eax >> 8) & 0xF;
    let extended_family = (eax >> 20) & 0xFF;

    if base_family == 0xF {
        base_family + extended_family
    } else {
        base_family
    }
}

fn get_cpu_model() -> u32 {
    let mut eax: u32;
    let mut _ebx: u32;
    let mut _ecx: u32;
    let mut _edx: u32;

    unsafe {
        asm!(
            "cpuid",
            inout("eax") 0x01 => eax,
            lateout("ecx") _ecx,
            lateout("edx") _edx,
        );
    }

    unsafe {
        asm!(
            "mov eax, ebx",
            out("eax") _ebx,
            options(nostack, nomem, preserves_flags),
        );
    }

    let family = (eax >> 8) & 0xF;
    let base_model = (eax >> 4) & 0xF;
    let extended_model = (eax >> 16) & 0xF;

    if family == 0x6 || family == 0xF {
        (extended_model << 4) | base_model
    } else {
        base_model
    }
}

fn get_threads_per_core() -> u32 {
    let mut _eax: u32;
    let mut ebx: u32;
    let mut _ecx: u32;
    let mut _edx: u32;

    unsafe {
        _eax = 0x0B;
        _ecx = 0;
        asm!(
            "cpuid",
            inout("eax") _eax,
            inout("ecx") _ecx,
            lateout("edx") _edx,
        );
    }

    unsafe {
        asm!(
            "mov eax, ebx",
            out("eax") ebx,
            options(nostack, nomem, preserves_flags),
        );
    }

    if _eax != 0 {
        let logical_processors = ebx;

        unsafe {
            _ecx = 1;
            asm!(
                "cpuid",
                inout("eax") _eax,
                inout("ecx") _ecx,
                lateout("edx") _edx,
            );
        }
        unsafe {
            asm!(
                "mov eax, ebx",
                out("eax") ebx,
                options(nostack, nomem, preserves_flags),
            );
        }

        let cores_per_package = ebx;
        if cores_per_package > 0 {
            return logical_processors / cores_per_package;
        }
    }

    unsafe {
        _eax = 0x01;
        asm!(
            "cpuid",
            inout("eax") _eax,
            lateout("ecx") _ecx,
            lateout("edx") _edx,
        );
    }

    unsafe {
        asm!(
            "mov eax, ebx",
            out("eax") ebx,
            options(nostack, nomem, preserves_flags),
        );
    }

    let logical_processors = (ebx >> 16) & 0xFF;
    let hyper_threading = (_edx & (1 << 28)) != 0;

    if hyper_threading {
        let core_count = get_cpu_count();
        if core_count > 0 {
            return logical_processors + 1 / core_count;
        }
    }

    1
}

fn get_cores_per_socket() -> u32 {
    let mut _eax: u32;
    let mut ebx: u32;
    let mut _ecx: u32;
    let mut _edx: u32;

    _eax = 0x0B;
    _ecx = 1;

    unsafe {
        asm!(
            "cpuid",
            inout("eax") _eax => _eax,
            inout("ecx") _ecx => _ecx,
            lateout("edx") _edx,
        );
    }

    unsafe {
        asm!(
            "mov eax, ebx",
            out("eax") ebx,
            options(nostack, nomem, preserves_flags),
        );
    }

    fix_result(ebx)
}

fn get_sockets() -> u32 {
    let mut _eax: u32;
    let mut ebx: u32;
    let mut _ecx: u32;
    let mut _edx: u32;

    _eax = 0x0B;
    _ecx = 0;

    unsafe {
        asm!(
            "cpuid",
            inout("eax") _eax => _eax,
            inout("ecx") _ecx => _ecx,
            lateout("edx") _edx,
        );
    }

    unsafe {
        asm!(
            "mov eax, ebx",
            out("eax") ebx,
            options(nostack, nomem, preserves_flags),
        );
    }

    fix_result(ebx)
}

fn fix_result(n: u32) -> u32 {
    let mut c = 1;
    let mut n = n;

    while (n & c) != 0 {
        n ^= c;
        c <<= 1;
    }

    n ^ c
}

fn get_stepping() -> u32 {
    let mut eax: u32;
    let mut _ebx: u32;
    let mut _ecx: u32;
    let mut _edx: u32;

    unsafe {
        asm!(
            "cpuid",
            inout("eax") 1 => eax,
            out("ecx") _ecx,
            out("edx") _edx,
        );

        asm!(
            "mov eax, ebx",
            out("eax") _ebx,
            options(nostack, nomem, preserves_flags),
        );

        eax & 0xF
    }
}

fn get_boost_enabled() -> &'static str {
    let mut _eax: u32;
    let mut ebx: u32;

    unsafe {
        asm!(
            "cpuid",
            inout("eax") 0x80000007u32 as i32 => _eax,
        );

        asm!(
            "mov eax, ebx",
            out("eax") ebx,
            options(nostack, nomem, preserves_flags),
        );

        match (ebx as u64 & (1 << 38)) != 0 {
            true => "disabled",
            false => "enabled",
        }
    }
}

fn get_hybrid_flag() -> &'static str {
    unsafe {
        let edx: u32;

        asm!(
            "cpuid",
            inout("eax") 0x07 => _,
            inout("ecx") 0 => _,
            out("edx") edx,
        );

        match (edx & (1 << 15)) != 0 {
            true => "hybryd",
            false => "no",
        }
    }
}

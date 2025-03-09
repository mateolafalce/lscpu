use core::{arch::asm, fmt::Write};

pub fn lscpu() -> String {
    let mut output = String::new();
    output.push_str(&format!(
        "Architecture:             {}\n",
        get_cpu_architecture()
    ));
    output.push_str(&format!(
        "  CPU op-mode(s):         {}\n",
        get_cpu_op_modes()
    ));
    output.push_str(&format!(
        "  Address sizes:          {}\n",
        get_address_sizes()
    ));
    output.push_str(&format!("  Byte Order:             {}\n", get_byte_order()));
    output.push_str(&format!("CPU(s):                   {}\n", get_cpu_count()));
    output.push_str(&format!(
        "  On-line CPU(s) list:    0,{}\n",
        get_on_line_cpu()
    ));
    output.push_str(&format!("Vendor ID:                {}\n", get_vendor_id()));
    output.push_str(&format!("  Model name:             {}\n", get_model_name()));
    output.push_str(&format!("    CPU family:           {}\n", get_cpu_family()));
    output.push_str(&format!("    Model:                {}\n", get_cpu_model()));
    output.push_str(&format!(
        "    Thread(s) per core:   {}\n",
        get_threads_per_core()
    ));
    output.push_str(&format!(
        "    Core(s) per socket:   {}\n",
        get_cores_per_socket()
    ));
    output.push_str(&format!("    Socket(s):            {}\n", get_sockets()));
    output.push_str(&format!("    Stepping:             {}\n", get_stepping()));
    output.push_str(&format!(
        "    Frequency boost:      {}\n",
        get_boost_enabled()
    ));
    output
}

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

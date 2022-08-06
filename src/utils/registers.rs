use std::collections::HashMap;

lazy_static! {
    pub static ref MAPPED_REGISTERS: HashMap<u16, &'static str> = {
        let mut m = HashMap::new();

        // PPU
        m.insert(0x2000, "PPUCTRL");
        m.insert(0x2001, "PPUMASK");
        m.insert(0x2002, "PPUSTATUS");
        m.insert(0x2003, "OAMADDR");
        m.insert(0x2004, "OAMDATA");
        m.insert(0x2005, "PPUSCROLL");
        m.insert(0x2006, "PPUADDR");
        m.insert(0x2007, "PPUDATA");
        m.insert(0x4014, "OAMDMA");

        // 2A03

        m.insert(0x4000, "SQ1_VOL");
        m.insert(0x4001, "SQ1_SWEEP");
        m.insert(0x4002, "SQ1_LO");
        m.insert(0x4003, "SQ1_HI");
        m.insert(0x4004, "SQ2_VOL");
        m.insert(0x4005, "SQ2_SWEEP");
        m.insert(0x4006, "SQ2_LO");
        m.insert(0x4007, "SQ2_HI");
        m.insert(0x4008, "TRI_LINEAR");
        m.insert(0x400A, "TRI_LO");
        m.insert(0x400B, "TRI_HI");
        m.insert(0x400C, "NOISE_VOL");
        m.insert(0x400E, "NOISE_LO");
        m.insert(0x400F, "NOISE_HI");
        m.insert(0x4010, "DMC_FREQ");
        m.insert(0x4011, "DMC_RAW");
        m.insert(0x4012, "DMC_START");
        m.insert(0x4013, "DMC_LEN");
        m.insert(0x4014, "OAMDMA");
        m.insert(0x4015, "SND_CHN");
        m.insert(0x4016, "JOY1");
        m.insert(0x4017, "JOY2");

        m
    };
}

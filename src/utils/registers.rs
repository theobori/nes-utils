pub fn get_mapped_register(address: u16) -> Option<String> {
    let name = match address {
        // PPU
        0x2000 => "PPUCTRL",
        0x2001 => "PPUMASK",
        0x2002 => "PPUSTATUS",
        0x2003 => "OAMADDR",
        0x2004 => "OAMDATA",
        0x2005 => "PPUSCROLL",
        0x2006 => "PPUADDR",
        0x2007 => "PPUDATA",
        0x4014 => "OAMDMA",

        // 2A03
        0x4000 => "SQ1_VOL",
        0x4001 => "SQ1_SWEEP",
        0x4002 => "SQ1_LO",
        0x4003 => "SQ1_HI",
        0x4004 => "SQ2_VOL",
        0x4005 => "SQ2_SWEEP",
        0x4006 => "SQ2_LO",
        0x4007 => "SQ2_HI",
        0x4008 => "TRI_LINEAR",
        0x400A => "TRI_LO",
        0x400B => "TRI_HI",
        0x400C => "NOISE_VOL",
        0x400E => "NOISE_LO",
        0x400F => "NOISE_HI",
        0x4010 => "DMC_FREQ",
        0x4011 => "DMC_RAW",
        0x4012 => "DMC_START",
        0x4013 => "DMC_LEN",
        0x4015 => "SND_CHN",
        0x4016 => "JOY1",
        0x4017 => "JOY2",
        _ => return None
    };

    let ret = String::from(name);

    Some(ret)
}

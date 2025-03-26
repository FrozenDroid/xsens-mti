use crate::wire::WireError;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Gnss2DFix {
    pub lat: f32,
    pub lon: f32,

    // in meters
    pub horizontal_accuracy: f32,
    pub vertical_accuracy: f32,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct Gnss3DFix {
    pub fix2d: Gnss2DFix,
    pub alt: f32,
    pub alt_msl: f32,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum GnssFix {
    NoFix,
    DeadReckoningOnly,
    Fix2D(Gnss2DFix),
    Fix3D(Gnss3DFix),
    GnssWithDeadReckoning(Gnss3DFix),
    TimeOnlyFix,
}

/// Contains the latitude and longitude in degrees of the GNSS/INS position
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct GnssPvtData {
    pub itow: u32,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
    pub nano: i32,
    pub valid: u8,
    // in nanoseconds
    pub time_accuracy: u32,
    pub satellites_used: u8,
    pub fix: GnssFix,
}

impl GnssPvtData {
    pub fn from_be_slice(bytes: &[u8]) -> Result<Self, WireError> {
        let fix_type = u8::from_be(bytes[20]);

        let lon = i32::from_be_bytes(bytes[24..28].try_into().unwrap()) as f32 * 1e-7;
        let lat = i32::from_be_bytes(bytes[28..32].try_into().unwrap()) as f32 * 1e-7;

        let h_acc = u32::from_be_bytes(bytes[40..44].try_into().unwrap()) as f32;
        let v_acc = u32::from_be_bytes(bytes[44..48].try_into().unwrap()) as f32;

        let fix2d = Gnss2DFix {
            lat,
            lon,
            horizontal_accuracy: h_acc / 1000.0,
            vertical_accuracy: v_acc / 1000.0,
        };
        let fix3d = Gnss3DFix {
            fix2d,
            alt: i32::from_be_bytes(bytes[32..36].try_into().unwrap()) as f32 / 1000.0,
            alt_msl: i32::from_be_bytes(bytes[36..40].try_into().unwrap()) as f32 / 1000.0,
        };

        Ok(GnssPvtData {
            itow: u32::from_be_bytes(bytes[0..4].try_into().unwrap()),
            year: u16::from_be_bytes(bytes[4..6].try_into().unwrap()),
            month: u8::from_be(bytes[6]),
            day: u8::from_be(bytes[7]),
            hour: u8::from_be(bytes[8]),
            min: u8::from_be(bytes[9]),
            sec: u8::from_be(bytes[10]),
            nano: i32::from_be_bytes(bytes[16..20].try_into().unwrap()),
            valid: u8::from_be(bytes[11]),
            time_accuracy: u32::from_be_bytes(bytes[12..16].try_into().unwrap()),
            satellites_used: u8::from_be(bytes[22]),
            fix: match fix_type {
                0 => GnssFix::NoFix,
                1 => GnssFix::DeadReckoningOnly,
                2 => GnssFix::Fix2D(fix2d),
                3 => GnssFix::Fix3D(fix3d),
                4 => GnssFix::GnssWithDeadReckoning(fix3d),
                _ => panic!(),
            },
        })
    }
}

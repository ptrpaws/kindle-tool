use binrw::{BinRead, BinReaderExt, BinResult, Endian};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Cursor, Read, Seek, SeekFrom, Write};
use strum::{Display as StrumDisplay, FromRepr};

// a LUT would be ~7x slower (benchmarked on aarch64)
#[inline(always)]
fn deobfuscate_byte(byte: u8) -> u8 {
  ((byte >> 4) | (byte << 4)) ^ 0xA7
}

pub fn deobfuscate_in_place(data: &mut [u8]) {
  for byte in data.iter_mut() {
    *byte = deobfuscate_byte(*byte);
  }
}

#[inline(always)]
fn obfuscate_byte(byte: u8) -> u8 {
  ((byte >> 4) | (byte << 4)) ^ 0x7A
}

pub fn obfuscate_in_place(data: &mut [u8]) {
  for byte in data.iter_mut() {
    *byte = obfuscate_byte(*byte);
  }
}

mod parsers {
  use super::{deobfuscate_in_place, BinResult};

  #[binrw::parser(reader)]
  pub fn parse_deobfuscated_md5() -> BinResult<String> {
    let mut buf = [0u8; 32];
    reader.read_exact(&mut buf)?;
    deobfuscate_in_place(&mut buf);
    Ok(String::from_utf8_lossy(&buf).to_string())
  }
}

// device table as of abff364 kindletool
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BinRead, FromRepr, StrumDisplay, Default)]
#[br(repr = u16)]
pub enum Device {
  #[strum(to_string = "Kindle 1")]
  Kindle1 = 0x01,
  #[strum(to_string = "Kindle 2 US")]
  Kindle2US = 0x02,
  #[strum(to_string = "Kindle 2 International")]
  Kindle2International = 0x03,
  #[strum(to_string = "Kindle DX US")]
  KindleDXUS = 0x04,
  #[strum(to_string = "Kindle DX International")]
  KindleDXInternational = 0x05,
  #[strum(to_string = "Unknown Kindle (0x07)")]
  ValidKindleUnknown_0x07 = 0x07,
  #[strum(to_string = "Kindle 3 WiFi+3G")]
  Kindle3WiFi3G = 0x06,
  #[strum(to_string = "Kindle 3 WiFi")]
  Kindle3WiFi = 0x08,
  #[strum(to_string = "Kindle DX Graphite")]
  KindleDXGraphite = 0x09,
  #[strum(to_string = "Kindle 3 WiFi+3G Europe")]
  Kindle3WiFi3GEurope = 0x0A,
  #[strum(to_string = "Unknown Kindle (0x0B)")]
  ValidKindleUnknown_0x0B = 0x0B,
  #[strum(to_string = "Unknown Kindle (0x0C)")]
  ValidKindleUnknown_0x0C = 0x0C,
  #[strum(to_string = "Unknown Kindle (0x0D)")]
  ValidKindleUnknown_0x0D = 0x0D,
  #[strum(to_string = "Silver Kindle 4 Non-Touch (2011)")]
  Kindle4NonTouch = 0x0E,
  #[strum(to_string = "Kindle 5 Touch WiFi+3G")]
  Kindle5TouchWiFi3G = 0x0F,
  #[strum(to_string = "Kindle 5 Touch WiFi+3G Europe")]
  Kindle5TouchWiFi3GEurope = 0x10,
  #[strum(to_string = "Kindle 5 Touch WiFi")]
  Kindle5TouchWiFi = 0x11,
  #[strum(to_string = "Kindle 5 Touch (Unknown Variant)")]
  Kindle5TouchUnknown = 0x12,
  #[strum(to_string = "Kindle PaperWhite WiFi+3G")]
  KindlePaperWhiteWiFi3G = 0x1B,
  #[strum(to_string = "Kindle PaperWhite WiFi+3G Canada")]
  KindlePaperWhiteWiFi3GCanada = 0x1C,
  #[strum(to_string = "Kindle PaperWhite WiFi+3G Europe")]
  KindlePaperWhiteWiFi3GEurope = 0x1D,
  #[strum(to_string = "Kindle PaperWhite WiFi+3G Japan")]
  KindlePaperWhiteWiFi3GJapan = 0x1F,
  #[strum(to_string = "Kindle PaperWhite WiFi+3G Brazil")]
  KindlePaperWhiteWiFi3GBrazil = 0x20,
  #[strum(to_string = "Black Kindle 4 Non-Touch (2012)")]
  Kindle4NonTouchBlack = 0x23,
  #[strum(to_string = "Kindle PaperWhite WiFi")]
  KindlePaperWhiteWiFi = 0x24,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) WiFi Japan")]
  KindlePaperWhite2WiFiJapan = 0x5A,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) WiFi")]
  KindlePaperWhite2WiFi = 0xD4,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) WiFi+3G")]
  KindlePaperWhite2WiFi3G = 0xD5,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) WiFi+3G Canada")]
  KindlePaperWhite2WiFi3GCanada = 0xD6,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) WiFi+3G Europe")]
  KindlePaperWhite2WiFi3GEurope = 0xD7,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) WiFi+3G Russia")]
  KindlePaperWhite2WiFi3GRussia = 0xD8,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) WiFi+3G Japan")]
  KindlePaperWhite2WiFi3GJapan = 0xF2,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) WiFi (4GB) International")]
  KindlePaperWhite2WiFi4GBInternational = 0x17,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) WiFi+3G (4GB) Canada")]
  KindlePaperWhite2WiFi3G4GBCanada = 0x5F,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) WiFi+3G (4GB) Europe")]
  KindlePaperWhite2WiFi3G4GBEurope = 0x60,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) WiFi+3G (4GB) Brazil")]
  KindlePaperWhite2WiFi3G4GBBrazil = 0x61,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) WiFi+3G (4GB)")]
  KindlePaperWhite2WiFi3G4GB = 0x62,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) (Unknown Variant 0xF4)")]
  KindlePaperWhite2Unknown_0xF4 = 0xF4,
  #[strum(to_string = "Kindle PaperWhite 2 (2013) (Unknown Variant 0xF9)")]
  KindlePaperWhite2Unknown_0xF9 = 0xF9,
  #[strum(to_string = "Kindle Voyage WiFi")]
  KindleVoyageWiFi = 0x13,
  #[strum(to_string = "Kindle Voyage WiFi+3G")]
  KindleVoyageWiFi3G = 0x54,
  #[strum(to_string = "Kindle Voyage WiFi+3G Japan")]
  KindleVoyageWiFi3GJapan = 0x2A,
  #[strum(to_string = "Kindle Voyage WiFi+3G (Variant 0x4F)")]
  KindleVoyageWiFi3G_0x4F = 0x4F,
  #[strum(to_string = "Kindle Voyage WiFi+3G Mexico")]
  KindleVoyageWiFi3GMexico = 0x52,
  #[strum(to_string = "Kindle Voyage WiFi+3G Europe")]
  KindleVoyageWiFi3GEurope = 0x53,
  #[strum(to_string = "Kindle Basic (2014)")]
  KindleBasic = 0xC6,
  #[strum(to_string = "Unknown Kindle (0x99)")]
  ValidKindleUnknown_0x99 = 0x99,
  #[strum(to_string = "Kindle Basic (2014) Australia")]
  KindleBasicKiwi = 0xDD,
  #[strum(to_string = "Unknown Kindle (0x16)")]
  ValidKindleUnknown_0x16 = 0x16,
  #[strum(to_string = "Unknown Kindle (0x21)")]
  ValidKindleUnknown_0x21 = 0x21,
  #[strum(to_string = "Kindle PaperWhite 3 (2015) WiFi")]
  KindlePaperWhite3WiFi = 0x201,
  #[strum(to_string = "Kindle PaperWhite 3 (2015) WiFi+3G")]
  KindlePaperWhite3WiFi3G = 0x202,
  #[strum(to_string = "Kindle PaperWhite 3 (2015) WiFi+3G Mexico")]
  KindlePaperWhite3WiFi3GMexico = 0x204,
  #[strum(to_string = "Kindle PaperWhite 3 (2015) WiFi+3G Europe")]
  KindlePaperWhite3WiFi3GEurope = 0x205,
  #[strum(to_string = "Kindle PaperWhite 3 (2015) WiFi+3G Canada")]
  KindlePaperWhite3WiFi3GCanada = 0x206,
  #[strum(to_string = "Kindle PaperWhite 3 (2015) WiFi+3G Japan")]
  KindlePaperWhite3WiFi3GJapan = 0x207,
  #[strum(to_string = "White Kindle PaperWhite 3 (2016) WiFi")]
  KindlePaperWhite3WhiteWiFi = 0x26B,
  #[strum(to_string = "White Kindle PaperWhite 3 (2016) WiFi+3G Japan")]
  KindlePaperWhite3WhiteWiFi3GJapan = 0x26C,
  #[strum(to_string = "White Kindle PaperWhite 3 (Unknown Variant 0KD)")]
  KindlePW3WhiteUnknown_0KD = 0x26D,
  #[strum(to_string = "White Kindle PaperWhite 3 (2016) WiFi+3G International")]
  KindlePaperWhite3WhiteWiFi3GInternational = 0x26E,
  #[strum(to_string = "White Kindle PaperWhite 3 (2016) WiFi+3G International (Bis)")]
  KindlePaperWhite3WhiteWiFi3GInternationalBis = 0x26F,
  #[strum(to_string = "White Kindle PaperWhite 3 (Unknown Variant 0KG)")]
  KindlePW3WhiteUnknown_0KG = 0x270,
  #[strum(to_string = "Kindle PaperWhite 3 (2016) WiFi (32GB) Japan")]
  KindlePaperWhite3BlackWiFi32GBJapan = 0x293,
  #[strum(to_string = "White Kindle PaperWhite 3 (2016) WiFi (32GB) Japan")]
  KindlePaperWhite3WhiteWiFi32GBJapan = 0x294,
  #[strum(to_string = "Kindle PaperWhite 3 (2016) (Unknown Variant TTT)")]
  KindlePW3Unknown_TTT = 0x6F7B,
  #[strum(to_string = "Kindle Oasis WiFi")]
  KindleOasisWiFi = 0x20C,
  #[strum(to_string = "Kindle Oasis WiFi+3G")]
  KindleOasisWiFi3G = 0x20D,
  #[strum(to_string = "Kindle Oasis WiFi+3G International")]
  KindleOasisWiFi3GInternational = 0x219,
  #[strum(to_string = "Kindle Oasis (Unknown Variant 0GS)")]
  KindleOasisUnknown_0GS = 0x21A,
  #[strum(to_string = "Kindle Oasis WiFi+3G China")]
  KindleOasisWiFi3GChina = 0x21B,
  #[strum(to_string = "Kindle Oasis WiFi+3G Europe")]
  KindleOasisWiFi3GEurope = 0x21C,
  #[strum(to_string = "Kindle Basic 2 (2016) (Unknown Variant 0DU)")]
  KindleBasic2Unknown_0DU = 0x1BC,
  #[strum(to_string = "Kindle Basic 2 (2016)")]
  KindleBasic2 = 0x269,
  #[strum(to_string = "White Kindle Basic 2 (2016)")]
  KindleBasic2White = 0x26A,
  #[strum(to_string = "Kindle Oasis 2 (2017) (Unknown Variant 0LM)")]
  KindleOasis2Unknown_0LM = 0x295,
  #[strum(to_string = "Kindle Oasis 2 (2017) (Unknown Variant 0LN)")]
  KindleOasis2Unknown_0LN = 0x296,
  #[strum(to_string = "Kindle Oasis 2 (2017) (Unknown Variant 0LP)")]
  KindleOasis2Unknown_0LP = 0x297,
  #[strum(to_string = "Kindle Oasis 2 (2017) (Unknown Variant 0LQ)")]
  KindleOasis2Unknown_0LQ = 0x298,
  #[strum(to_string = "Champagne Kindle Oasis 2 (2017) WiFi (32GB)")]
  KindleOasis2WiFi32GBChampagne = 0x2E1,
  #[strum(to_string = "Kindle Oasis 2 (2017) (Unknown Variant 0P2)")]
  KindleOasis2Unknown_0P2 = 0x2E2,
  #[strum(to_string = "Kindle Oasis 2 (2017) WiFi+3G (32GB) (Variant 0P6)")]
  KindleOasis2Unknown_0P6 = 0x2E6,
  #[strum(to_string = "Kindle Oasis 2 (2017) (Unknown Variant 0P7)")]
  KindleOasis2Unknown_0P7 = 0x2E7,
  #[strum(to_string = "Kindle Oasis 2 (2017) WiFi (8GB)")]
  KindleOasis2WiFi8GB = 0x2E8,
  #[strum(to_string = "Kindle Oasis 2 (2017) WiFi+3G (32GB)")]
  KindleOasis2WiFi3G32GB = 0x341,
  #[strum(to_string = "Kindle Oasis 2 (2017) WiFi+3G (32GB) Europe")]
  KindleOasis2WiFi3G32GBEurope = 0x342,
  #[strum(to_string = "Kindle Oasis 2 (2017) (Unknown Variant 0S3)")]
  KindleOasis2Unknown_0S3 = 0x343,
  #[strum(to_string = "Kindle Oasis 2 (2017) (Unknown Variant 0S4)")]
  KindleOasis2Unknown_0S4 = 0x344,
  #[strum(to_string = "Kindle Oasis 2 (2017) (Unknown Variant 0S7)")]
  KindleOasis2Unknown_0S7 = 0x347,
  #[strum(to_string = "Kindle Oasis 2 (2017) WiFi (32GB)")]
  KindleOasis2WiFi32GB = 0x34A,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) WiFi (8GB)")]
  KindlePaperWhite4WiFi8GB = 0x2F7,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) WiFi+4G (32GB)")]
  KindlePaperWhite4WiFi4G32GB = 0x361,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) WiFi+4G (32GB) Europe")]
  KindlePaperWhite4WiFi4G32GBEurope = 0x362,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) WiFi+4G (32GB) Japan")]
  KindlePaperWhite4WiFi4G32GBJapan = 0x363,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) (Unknown Variant 0T4)")]
  KindlePaperWhite4Unknown_0T4 = 0x364,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) (Unknown Variant 0T5)")]
  KindlePaperWhite4Unknown_0T5 = 0x365,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) WiFi (32GB)")]
  KindlePaperWhite4WiFi32GB = 0x366,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) (Unknown Variant 0T7)")]
  KindlePaperWhite4Unknown_0T7 = 0x367,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) (Unknown Variant 0TJ)")]
  KindlePaperWhite4Unknown_0TJ = 0x372,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) (Unknown Variant 0TK)")]
  KindlePaperWhite4Unknown_0TK = 0x373,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) (Unknown Variant 0TL)")]
  KindlePaperWhite4Unknown_0TL = 0x374,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) (Unknown Variant 0TM)")]
  KindlePaperWhite4Unknown_0TM = 0x375,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) (Unknown Variant 0TN)")]
  KindlePaperWhite4Unknown_0TN = 0x376,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) WiFi (8GB) India")]
  KindlePaperWhite4WiFi8GBIndia = 0x402,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) WiFi (32GB) India")]
  KindlePaperWhite4WiFi32GBIndia = 0x403,
  #[strum(to_string = "Twilight Blue Kindle PaperWhite 4 (2018) WiFi (32GB)")]
  KindlePaperWhite4WiFi32GBBlue = 0x4D8,
  #[strum(to_string = "Plum Kindle PaperWhite 4 (2018) WiFi (32GB)")]
  KindlePaperWhite4WiFi32GBPlum = 0x4D9,
  #[strum(to_string = "Sage Kindle PaperWhite 4 (2018) WiFi (32GB)")]
  KindlePaperWhite4WiFi32GBSage = 0x4DA,
  #[strum(to_string = "Twilight Blue Kindle PaperWhite 4 (2018) WiFi (8GB)")]
  KindlePaperWhite4WiFi8GBBlue = 0x4DB,
  #[strum(to_string = "Plum Kindle PaperWhite 4 (2018) WiFi (8GB)")]
  KindlePaperWhite4WiFi8GBPlum = 0x4DC,
  #[strum(to_string = "Sage Kindle PaperWhite 4 (2018) WiFi (8GB)")]
  KindlePaperWhite4WiFi8GBSage = 0x4DD,
  #[strum(to_string = "Kindle PaperWhite 4 (2018) (Unknown Variant 0PL)")]
  KindlePW4Unknown_0PL = 0x2F4,
  #[strum(to_string = "Kindle Basic 3 (2019)")]
  KindleBasic3 = 0x414,
  #[strum(to_string = "White Kindle Basic 3 (2019) (8GB)")]
  KindleBasic3White8GB = 0x3CF,
  #[strum(to_string = "Kindle Basic 3 (2019) (Unknown Variant 0WG)")]
  KindleBasic3Unknown_0WG = 0x3D0,
  #[strum(to_string = "White Kindle Basic 3 (2019)")]
  KindleBasic3White = 0x3D1,
  #[strum(to_string = "Kindle Basic 3 (2019) (Unknown Variant 0WJ)")]
  KindleBasic3Unknown_0WJ = 0x3D2,
  #[strum(to_string = "Kindle Basic 3 (2019) Kids Edition")]
  KindleBasic3KidsEdition = 0x3AB,
  #[strum(to_string = "Champagne Kindle Oasis 3 (2019) WiFi (32GB)")]
  KindleOasis3WiFi32GBChampagne = 0x434,
  #[strum(to_string = "Kindle Oasis 3 (2019) WiFi+4G (32GB) Japan")]
  KindleOasis3WiFi4G32GBJapan = 0x3D8,
  #[strum(to_string = "Kindle Oasis 3 (2019) WiFi+4G (32GB) India")]
  KindleOasis3WiFi4G32GBIndia = 0x3D7,
  #[strum(to_string = "Kindle Oasis 3 (2019) WiFi+4G (32GB)")]
  KindleOasis3WiFi4G32GB = 0x3D6,
  #[strum(to_string = "Kindle Oasis 3 (2019) WiFi (32GB)")]
  KindleOasis3WiFi32GB = 0x3D5,
  #[strum(to_string = "Kindle Oasis 3 (2019) WiFi (8GB)")]
  KindleOasis3WiFi8GB = 0x3D4,
  #[strum(to_string = "Kindle PaperWhite 5 Signature Edition (2021)")]
  KindlePaperWhite5SignatureEdition = 0x690,
  #[strum(to_string = "Kindle PaperWhite 5 (2011) (Unknown Variant 1Q0)")]
  KindlePaperWhite5Unknown_1Q0 = 0x700,
  #[strum(to_string = "Kindle PaperWhite 5 (2021)")]
  KindlePaperWhite5 = 0x6FF,
  #[strum(to_string = "Kindle PaperWhite 5 (2021) (Unknown Variant 1VD)")]
  KindlePaperWhite5Unknown_1VD = 0x7AD,
  #[strum(to_string = "Kindle PaperWhite 5 Signature Edition (2021) (Variant 219)")]
  KindlePaperWhite5SE_219 = 0x829,
  #[strum(to_string = "Kindle PaperWhite 5 (2021) (Variant 21A)")]
  KindlePaperWhite5_21A = 0x82A,
  #[strum(to_string = "Kindle PaperWhite 5 Signature Edition (2021) (Variant 2BH)")]
  KindlePaperWhite5SE_2BH = 0x971,
  #[strum(to_string = "Kindle PaperWhite 5 (2021) (Unknown Variant 2BJ)")]
  KindlePaperWhite5Unknown_2BJ = 0x972,
  #[strum(to_string = "Kindle PaperWhite 5 (2021) (Variant 2DK)")]
  KindlePaperWhite5_2DK = 0x9B3,
  #[strum(to_string = "Kindle Basic 4 (2022) (Unknown Variant 22D)")]
  KindleBasic4Unknown_22D = 0x84D,
  #[strum(to_string = "Kindle Basic 4 (2022) (Unknown Variant 25T)")]
  KindleBasic4Unknown_25T = 0x8BB,
  #[strum(to_string = "Kindle Basic 4 (2022) (Unknown Variant 23A)")]
  KindleBasic4Unknown_23A = 0x86A,
  #[strum(to_string = "Kindle Basic 4 (2022) (Variant 2AQ)")]
  KindleBasic4_2AQ = 0x958,
  #[strum(to_string = "Kindle Basic 4 (2022) (Variant 2AP)")]
  KindleBasic4_2AP = 0x957,
  #[strum(to_string = "Kindle Basic 4 (2022) (Unknown Variant 1XH)")]
  KindleBasic4Unknown_1XH = 0x7F1,
  #[strum(to_string = "Kindle Basic 4 (2022) (Unknown Variant 22C)")]
  KindleBasic4Unknown_22C = 0x84C,
  #[strum(to_string = "Kindle Scribe (Unknown Variant 27J)")]
  KindleScribeUnknown_27J = 0x8F2,
  #[strum(to_string = "Kindle Scribe (Unknown Variant 2BL)")]
  KindleScribeUnknown_2BL = 0x974,
  #[strum(to_string = "Kindle Scribe (Unknown Variant 263)")]
  KindleScribeUnknown_263 = 0x8C3,
  #[strum(to_string = "Kindle Scribe (16GB) (Variant 227)")]
  KindleScribe16GB_227 = 0x847,
  #[strum(to_string = "Kindle Scribe (Unknown Variant 2BM)")]
  KindleScribeUnknown_2BM = 0x975,
  #[strum(to_string = "Kindle Scribe (Variant 23L)")]
  KindleScribe_23L = 0x874,
  #[strum(to_string = "Kindle Scribe (64GB) (Variant 23M)")]
  KindleScribe64GB_23M = 0x875,
  #[strum(to_string = "Kindle Scribe (Unknown Variant 270)")]
  KindleScribeUnknown_270 = 0x8E0,
  #[strum(to_string = "Kindle Basic 5 (2024) (Unknown Variant 3L5)")]
  KindleBasic5Unknown_3L5 = 0xE85,
  #[strum(to_string = "Kindle Basic 5 (2024) (Unknown Variant 3L6)")]
  KindleBasic5Unknown_3L6 = 0xE86,
  #[strum(to_string = "Kindle Basic 5 (2024) (Unknown Variant 3L4)")]
  KindleBasic5Unknown_3L4 = 0xE84,
  #[strum(to_string = "Kindle Basic 5 (2024) (Unknown Variant 3L3)")]
  KindleBasic5Unknown_3L3 = 0xE83,
  #[strum(to_string = "Kindle Basic 5 (2024) (Unknown Variant A89)")]
  KindleBasic5Unknown_A89 = 0x2909,
  #[strum(to_string = "Kindle Basic 5 (2024) (Unknown Variant 3L2)")]
  KindleBasic5Unknown_3L2 = 0xE82,
  #[strum(to_string = "Kindle Basic 5 (2024) (Unknown Variant 3KM)")]
  KindleBasic5Unknown_3KM = 0xE75,
  #[strum(to_string = "Kindle PaperWhite 6 (2024) (Unknown Variant 349)")]
  KindlePaperWhite6Unknown_349 = 0xC89,
  #[strum(to_string = "Kindle PaperWhite 6 (2024) (Unknown Variant 346)")]
  KindlePaperWhite6Unknown_346 = 0xC86,
  #[strum(to_string = "Kindle PaperWhite 6 (2024) (Unknown Variant 33X)")]
  KindlePaperWhite6Unknown_33X = 0xC7F,
  #[strum(to_string = "Kindle PaperWhite 6 (2024) (Unknown Variant 33W)")]
  KindlePaperWhite6Unknown_33W = 0xC7E,
  #[strum(to_string = "Kindle PaperWhite 6 (2024) (Unknown Variant 3HA)")]
  KindlePaperWhite6Unknown_3HA = 0xE2A,
  #[strum(to_string = "Kindle PaperWhite 6 (2024) (Unknown Variant 3H5)")]
  KindlePaperWhite6Unknown_3H5 = 0xE25,
  #[strum(to_string = "Kindle PaperWhite 6 (2024) (Unknown Variant 3H3)")]
  KindlePaperWhite6Unknown_3H3 = 0xE23,
  #[strum(to_string = "Kindle PaperWhite 6 (2024) (Unknown Variant 3H8)")]
  KindlePaperWhite6Unknown_3H8 = 0xE28,
  #[strum(to_string = "Kindle PaperWhite 6 (2024) (Unknown Variant 3J5)")]
  KindlePaperWhite6Unknown_3J5 = 0xE45,
  #[strum(to_string = "Kindle PaperWhite 6 (2024) (Unknown Variant 3JS)")]
  KindlePaperWhite6Unknown_3JS = 0xE5A,
  #[strum(to_string = "Kindle Scribe 2 (2024) (Unknown Variant 3V0)")]
  KindleScribe2Unknown_3V0 = 0xFA0,
  #[strum(to_string = "Kindle Scribe 2 (2024) (Unknown Variant 3V1)")]
  KindleScribe2Unknown_3V1 = 0xFA1,
  #[strum(to_string = "Kindle Scribe 2 (2024) (Unknown Variant 3X5)")]
  KindleScribe2Unknown_3X5 = 0xFE5,
  #[strum(to_string = "Kindle Scribe 2 (2024) (Unknown Variant 3UV)")]
  KindleScribe2Unknown_3UV = 0xF9D,
  #[strum(to_string = "Kindle Scribe 2 (2024) (Unknown Variant 3X4)")]
  KindleScribe2Unknown_3X4 = 0xFE4,
  #[strum(to_string = "Kindle Scribe 2 (2024) (Unknown Variant 3X3)")]
  KindleScribe2Unknown_3X3 = 0xFE3,
  #[strum(to_string = "Kindle Scribe 2 (2024) (Unknown Variant 41E)")]
  KindleScribe2Unknown_41E = 0x102E,
  #[strum(to_string = "Kindle Scribe 2 (2024) (Unknown Variant 41D)")]
  KindleScribe2Unknown_41D = 0x102D,
  #[strum(to_string = "Kindle ColorSoft (2024) (Unknown Variant 3H9)")]
  KindleColorSoftUnknown_3H9 = 0xE29,
  #[strum(to_string = "Kindle ColorSoft (2024) (Unknown Variant 3H4)")]
  KindleColorSoftUnknown_3H4 = 0xE24,
  #[strum(to_string = "Kindle ColorSoft (2024) (Unknown Variant 3HB)")]
  KindleColorSoftUnknown_3HB = 0xE2B,
  #[strum(to_string = "Kindle ColorSoft (2024) (Unknown Variant 3H6)")]
  KindleColorSoftUnknown_3H6 = 0xE26,
  #[strum(to_string = "Kindle ColorSoft (2024) (Unknown Variant 3H2)")]
  KindleColorSoftUnknown_3H2 = 0xE22,
  #[strum(to_string = "Kindle ColorSoft (2024) (Unknown Variant 34X)")]
  KindleColorSoftUnknown_34X = 0xC9F,
  #[strum(to_string = "Kindle ColorSoft (2024) (Unknown Variant 3H7)")]
  KindleColorSoftUnknown_3H7 = 0xE27,
  #[strum(to_string = "Kindle ColorSoft (2024) (Unknown Variant 3JT)")]
  KindleColorSoftUnknown_3JT = 0xE5B,
  #[strum(to_string = "Kindle ColorSoft (2024) (Unknown Variant 3J6)")]
  KindleColorSoftUnknown_3J6 = 0xE46,
  #[strum(to_string = "Kindle ColorSoft (2024) (Unknown Variant 456)")]
  KindleColorSoftUnknown_456 = 0x10A6,
  #[strum(to_string = "Kindle ColorSoft (2024) (Unknown Variant 455)")]
  KindleColorSoftUnknown_455 = 0x10A5,
  #[strum(to_string = "Kindle ColorSoft (2024) (Unknown Variant 4EP)")]
  KindleColorSoftUnknown_4EP = 0x11D7,
  #[default]
  Unknown = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, BinRead, FromRepr, StrumDisplay, Default)]
#[br(repr = u32)]
pub enum Platform {
  #[strum(to_string = "Unspecified")]
  PlatUnspecified = 0x00,
  #[strum(to_string = "Mario (Deprecated)")]
  MarioDeprecated = 0x01,
  #[strum(to_string = "Luigi")]
  Luigi = 0x02,
  #[strum(to_string = "Banjo")]
  Banjo = 0x03,
  #[strum(to_string = "Yoshi")]
  Yoshi = 0x04,
  #[strum(to_string = "Yoshime (Prototype)")]
  YoshimeProto = 0x05,
  #[strum(to_string = "Yoshime (Yoshime3)")]
  Yoshime = 0x06,
  #[strum(to_string = "Wario")]
  Wario = 0x07,
  #[strum(to_string = "Duet")]
  Duet = 0x08,
  #[strum(to_string = "Heisenberg")]
  Heisenberg = 0x09,
  #[strum(to_string = "Zelda")]
  Zelda = 0x0A,
  #[strum(to_string = "Rex")]
  Rex = 0x0B,
  #[strum(to_string = "Bellatrix")]
  Bellatrix = 0x0C,
  #[strum(to_string = "Bellatrix3")]
  Bellatrix3 = 0x0D,
  #[strum(to_string = "Bellatrix4")]
  Bellatrix4 = 0x0E,
  #[default]
  Unknown,
}

#[derive(Debug, BinRead)]
pub struct OtaV1 {
  #[br(parse_with = parsers::parse_deobfuscated_md5)]
  pub md5_hash: String,
  pub source_rev: u32,
  pub target_rev: u32,
  pub device_code: u16,
  pub optional: u8,
  pub padding: u8,
}

impl Display for OtaV1 {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let device = Device::from_repr(self.device_code as usize).unwrap_or_default();
    writeln!(f, "{:<14} {}", "Bundle Type:", "OTA V1")?;
    writeln!(f, "{:<14} {}", "MD5 Hash:", self.md5_hash)?;
    writeln!(f, "{:<14} {}", "Minimum OTA:", self.source_rev)?;
    writeln!(f, "{:<14} {}", "Target OTA:", self.target_rev)?;
    writeln!(f, "{:<14} {} (0x{:04X})", "Device:", device, self.device_code)?;
    writeln!(f, "{:<14} {}", "Optional:", self.optional)?;
    write!(f, "{:<14} {} (0x{:02X})", "Padding Byte:", self.padding, self.padding)
  }
}

#[derive(Debug)]
pub struct MetaString(pub String);

impl BinRead for MetaString {
  type Args<'a> = ();
  fn read_options<R: Read + Seek>(reader: &mut R, _endian: Endian, _args: Self::Args<'_>) -> BinResult<Self> {
    let len: u16 = reader.read_be()?;
    let mut buf = vec![0; len as usize];
    reader.read_exact(&mut buf)?;
    deobfuscate_in_place(&mut buf);
    Ok(MetaString(String::from_utf8_lossy(&buf).to_string()))
  }
}

impl Display for MetaString {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug, BinRead)]
pub struct OtaV2 {
  pub source_rev: u64,
  pub target_rev: u64,
  pub num_devices: u16,
  #[br(count = num_devices)]
  pub device_codes: Vec<u16>,
  pub critical: u8,
  pub padding: u8,
  #[br(parse_with = parsers::parse_deobfuscated_md5)]
  pub md5_hash: String,
  pub num_metadata: u16,
  #[br(count = num_metadata)]
  pub metadata: Vec<MetaString>,
}

impl Display for OtaV2 {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    writeln!(f, "{:<14} {}", "Bundle Type:", "OTA V2")?;
    writeln!(f, "{:<14} {}", "Minimum OTA:", self.source_rev)?;
    writeln!(f, "{:<14} {}", "Target OTA:", self.target_rev)?;
    writeln!(f, "{:<14} {}", "Critical:", self.critical)?;
    writeln!(f, "{:<14} {} (0x{:02X})", "Padding Byte:", self.padding, self.padding)?;
    writeln!(f, "{:<14} {}", "MD5 Hash:", self.md5_hash)?;
    writeln!(f, "{:<14} {}", "Device Count:", self.device_codes.len())?;
    for &code in &self.device_codes {
      let device = Device::from_repr(code as usize).unwrap_or_default();
      writeln!(f, "  - {} (0x{:04X})", device, code)?;
    }
    write!(f, "{:<14} {}", "Metadata Count:", self.metadata.len())?;
    for meta in &self.metadata {
      write!(f, "\n  - {}", meta)?;
    }
    Ok(())
  }
}

#[derive(Debug, BinRead)]
#[br(little)]
struct RecoveryV1Header {
  #[br(seek_before = SeekFrom::Start(4))]
  target_ota_rev2: u64,

  #[br(seek_before = SeekFrom::Start(12), parse_with = parsers::parse_deobfuscated_md5)]
  md5_hash: String,

  magic1: u32,
  magic2: u32,
  minor: u32,

  #[br(seek_before = SeekFrom::Start(56))]
  device_or_platform_code: u32,

  header_rev: u32,
  board_code_rev2: u32,
}

#[derive(Debug)]
pub struct RecoveryV1 {
  pub md5_hash: String,
  pub magic1: u32,
  pub magic2: u32,
  pub minor: u32,
  pub header_rev: u32,
  pub device_info: RecoveryDevice,
  pub target_ota: Option<u64>,
}

#[derive(Debug)]
pub enum RecoveryDevice {
  Device(Device, u16),
  Platform { platform: Platform, board: u32 },
}

impl Display for RecoveryDevice {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    match self {
      RecoveryDevice::Device(device, code) => {
        writeln!(f, "{:<14} {} (0x{:04X})", "Device:", device, code)
      }
      RecoveryDevice::Platform { platform, board } => {
        writeln!(f, "{:<14} {}", "Platform:", platform)?;
        write!(f, "{:<14} Unknown (0x{:02X})", "Board:", board)
      }
    }
  }
}

impl BinRead for RecoveryV1 {
  type Args<'a> = ();
  fn read_options<R: Read + Seek>(reader: &mut R, _endian: Endian, _args: Self::Args<'_>) -> BinResult<Self> {
    let mut header_data = vec![0; 131068];
    reader.read_exact(&mut header_data)?;
    let mut cursor = Cursor::new(&header_data);
    let header: RecoveryV1Header = cursor.read_le()?;

    let device_info = if header.header_rev == 2 {
      RecoveryDevice::Platform {
        platform: Platform::from_repr(header.device_or_platform_code as usize)
          .unwrap_or_default(),
        board: header.board_code_rev2,
      }
    } else {
      let device_code = header.device_or_platform_code as u16;
      RecoveryDevice::Device(
        Device::from_repr(device_code as usize).unwrap_or_default(),
        device_code,
      )
    };

    let target_ota = if header.header_rev == 2 {
      Some(header.target_ota_rev2)
    } else {
      None
    };

    Ok(Self {
      md5_hash: header.md5_hash,
      magic1: header.magic1,
      magic2: header.magic2,
      minor: header.minor,
      header_rev: header.header_rev,
      device_info,
      target_ota,
    })
  }
}

impl Display for RecoveryV1 {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    writeln!(f, "{:<14} {}", "Bundle Type:", "Recovery V1")?;
    writeln!(f, "{:<14} {}", "MD5 Hash:", self.md5_hash)?;
    writeln!(f, "{:<14} {}", "Magic 1:", self.magic1)?;
    writeln!(f, "{:<14} {}", "Magic 2:", self.magic2)?;
    writeln!(f, "{:<14} {}", "Minor:", self.minor)?;
    writeln!(f, "{:<14} {}", "Header Rev:", self.header_rev)?;
    if let Some(ota) = self.target_ota {
      writeln!(f, "{:<14} {}", "Target OTA:", ota)?;
    }
    write!(f, "{}", self.device_info)
  }
}

#[derive(Debug, BinRead)]
#[br(little)]
struct RecoveryV2Contents {
  #[br(pad_before = 4)]
  target_ota: u64,
  #[br(parse_with = parsers::parse_deobfuscated_md5)]
  md5_hash: String,
  magic1: u32,
  magic2: u32,
  minor: u32,
  platform_code: u32,
  header_rev: u32,
  board: u32,
  #[br(pad_before = 7)]
  #[allow(dead_code)]
  num_devices: u8,
  #[br(count = num_devices)]
  device_codes: Vec<u16>,
}

#[derive(Debug)]
pub struct RecoveryV2 {
  pub target_ota: u64,
  pub md5_hash: String,
  pub magic1: u32,
  pub magic2: u32,
  pub minor: u32,
  pub platform_code: u32,
  pub header_rev: u32,
  pub board: u32,
  pub device_codes: Vec<u16>,
}

impl BinRead for RecoveryV2 {
  type Args<'a> = ();

  fn read_options<R: Read + Seek>(reader: &mut R, _endian: Endian, _args: Self::Args<'_>) -> BinResult<Self> {
    let mut header_data = vec![0; 131068];
    reader.read_exact(&mut header_data)?;
    let mut cursor = Cursor::new(&header_data);

    let contents: RecoveryV2Contents = cursor.read_le()?;

    Ok(Self {
      target_ota: contents.target_ota,
      md5_hash: contents.md5_hash,
      magic1: contents.magic1,
      magic2: contents.magic2,
      minor: contents.minor,
      platform_code: contents.platform_code,
      header_rev: contents.header_rev,
      board: contents.board,
      device_codes: contents.device_codes,
    })
  }
}

impl Display for RecoveryV2 {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    writeln!(f, "{:<14} {}", "Bundle Type:", "Recovery V2")?;
    writeln!(f, "{:<14} {}", "Target OTA:", self.target_ota)?;
    writeln!(f, "{:<14} {}", "MD5 Hash:", self.md5_hash)?;
    writeln!(f, "{:<14} {}", "Magic 1:", self.magic1)?;
    writeln!(f, "{:<14} {}", "Magic 2:", self.magic2)?;
    writeln!(f, "{:<14} {}", "Minor:", self.minor)?;
    let platform = Platform::from_repr(self.platform_code as usize).unwrap_or_default();
    writeln!(f, "{:<14} {} (0x{:02X})", "Platform:", platform, self.platform_code)?;
    writeln!(f, "{:<14} {}", "Header Rev:", self.header_rev)?;
    writeln!(f, "{:<14} Unknown (0x{:02X})", "Board:", self.board)?;
    write!(f, "{:<14} {}", "Device Count:", self.device_codes.len())?;
    for &code in &self.device_codes {
      let device = Device::from_repr(code as usize).unwrap_or_default();
      write!(f, "\n - {} (0x{:04X})", device, code)?;
    }
    Ok(())
  }
}

#[derive(Debug)]
pub struct SignatureEnvelope {
  pub cert_num: u32,
  pub signature: Vec<u8>,
  pub wrapped_bundle: Box<UpdateBundle>,
}

impl BinRead for SignatureEnvelope {
  type Args<'a> = ();
  fn read_options<R: Read + Seek>(reader: &mut R, endian: Endian, _args: Self::Args<'_>) -> BinResult<Self> {
    let cert_num: u32 = reader.read_le()?;
    reader.seek(SeekFrom::Current(56))?;
    let sig_size = if cert_num == 2 { 256 } else { 128 };
    let mut signature = vec![0; sig_size];
    reader.read_exact(&mut signature)?;

    let wrapped_bundle = Box::new(UpdateBundle::read_options(reader, endian, ())?);

    Ok(Self {
      cert_num,
      signature,
      wrapped_bundle,
    })
  }
}

impl Display for SignatureEnvelope {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    let cert_name = match self.cert_num {
      0x00 => "pubdevkey01.pem (Developer)",
      0x01 => "pubprodkey01.pem (Official 1K)",
      0x02 => "pubprodkey02.pem (Official 2K)",
      _ => "Unknown",
    };
    writeln!(f, "{:<14} {}", "Bundle Type:", "Signature Envelope")?;
    writeln!(f, "{:<14} {}", "Cert Number:", self.cert_num)?;
    writeln!(f, "{:<14} {}", "Cert File:", cert_name)?;

    writeln!(f, "\n--- Wrapped Bundle ---")?;
    write!(f, "{}", self.wrapped_bundle)
  }
}

#[derive(Debug, BinRead)]
pub enum UpdateBundle {
  #[br(magic = b"SP01")]
  Signed(Box<SignatureEnvelope>),
  #[br(magic = b"FC02")]
  OtaV1Fc02(OtaV1),
  #[br(magic = b"FD03")]
  OtaV1Fd03(OtaV1),
  #[br(magic = b"FC04")]
  OtaV2Fc04(OtaV2),
  #[br(magic = b"FD04")]
  OtaV2Fd04(OtaV2),
  #[br(magic = b"FL01")]
  OtaV2Fl01(OtaV2),
  #[br(magic = b"FB01")]
  RecoveryV1Fb01(RecoveryV1),
  #[br(magic = b"FB02")]
  RecoveryV1Fb02(RecoveryV1),
  #[br(magic = b"FB03")]
  RecoveryV2Fb03(RecoveryV2),
}

impl UpdateBundle {
  pub fn magic_str(&self) -> &'static str {
    match self {
      UpdateBundle::Signed(_) => "SP01",
      UpdateBundle::OtaV1Fc02(_) => "FC02",
      UpdateBundle::OtaV1Fd03(_) => "FD03",
      UpdateBundle::OtaV2Fc04(_) => "FC04",
      UpdateBundle::OtaV2Fd04(_) => "FD04",
      UpdateBundle::OtaV2Fl01(_) => "FL01",
      UpdateBundle::RecoveryV1Fb01(_) => "FB01",
      UpdateBundle::RecoveryV1Fb02(_) => "FB02",
      UpdateBundle::RecoveryV2Fb03(_) => "FB03",
    }
  }

  pub fn description(&self) -> &'static str {
    match self.magic_str() {
      "FB01" | "FB02" => "(Fullbin)",
      "FB03" => "(Fullbin [OTA?, fwo?])",
      "FC02" | "FC04" => "(OTA [ota])",
      "FD03" | "FD04" => "(Versionless [vls])",
      "FL01" => "(Language [lang])",
      "SP01" => "(Signing Envelope)",
      _ => "Unknown",
    }
  }
}

impl Display for UpdateBundle {
  fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
    writeln!(f, "{:<14} {} {}", "Bundle Magic:", self.magic_str(), self.description())?;
    match self {
      UpdateBundle::Signed(p) => write!(f, "{}", p),
      UpdateBundle::OtaV1Fc02(p) | UpdateBundle::OtaV1Fd03(p) => write!(f, "{}", p),
      UpdateBundle::OtaV2Fc04(p) | UpdateBundle::OtaV2Fd04(p) | UpdateBundle::OtaV2Fl01(p) => write!(f, "{}", p),
      UpdateBundle::RecoveryV1Fb01(p) | UpdateBundle::RecoveryV1Fb02(p) => write!(f, "{}", p),
      UpdateBundle::RecoveryV2Fb03(p) => write!(f, "{}", p),
    }
  }
}

pub fn dump_payload<R: Read + Seek, W: Write>(reader: &mut R, writer: &mut W) -> Result<(), Box<dyn std::error::Error>> {
  const BUFFER_SIZE: usize = 8192;

  let _bundle: UpdateBundle = reader.read_le()?;

  let mut buffer = [0; BUFFER_SIZE];
  loop {
    let bytes_read = reader.read(&mut buffer)?;
    if bytes_read == 0 {
      break;
    }

    let chunk = &mut buffer[..bytes_read];
    deobfuscate_in_place(chunk);
    writer.write_all(chunk)?;
  }

  Ok(())
}
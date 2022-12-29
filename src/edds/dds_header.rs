use std::io::{BufRead, Seek};

use deku::DekuEnumExt;
use deku::{
    bitvec::{BitSlice, BitVec, Msb0},
    prelude::*,
};
use deku::{DekuContainerWrite, DekuRead, DekuUpdate, DekuWrite};
use enumflags2::{bitflags, BitFlags};
use four_cc::FourCC;

use crate::core::errors::EddsError;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little", magic = b"DDS ")]
struct DekuTest {
    #[deku(bits = "4")]
    field_a: u8,
    #[deku(bits = "4")]
    field_b: u8,
    field_c: u16,
}

#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DdsPixelformatFlags {
    DDPF_ALPHAPIXELS = 1,
    DDPF_ALPHA = 2,
    DDPF_FOURCC = 4,
    DDPF_RGB = 64,
    DDPF_YUV = 512,
    DDPF_LUMINANCE = 131072,
}

#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DdsCapsFlags {
    DDSCAPS_COMPLEX = 0x8,
    DDSCAPS_MIPMAP = 0x400000,
    DDSCAPS_TEXTURE = 0x1000,
}

#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DdsCaps2Flags {
    DDSCAPS2_CUBEMAP = 0x200,
    DDSCAPS2_CUBEMAP_POSITIVEX = 0x400,
    DDSCAPS2_CUBEMAP_NEGATIVEX = 0x800,
    DDSCAPS2_CUBEMAP_POSITIVEY = 0x1000,
    DDSCAPS2_CUBEMAP_NEGATIVEY = 0x2000,
    DDSCAPS2_CUBEMAP_POSITIVEZ = 0x4000,
    DDSCAPS2_CUBEMAP_NEGATIVEZ = 0x8000,
    DDSCAPS2_VOLUME = 0x200000,
}

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct DdsPixelFormat {
    pub size: u32,
    #[deku(
        reader = "DdsPixelFormat::read_flags(deku::rest)",
        writer = "DdsPixelFormat::write_flags(deku::output, &self.flags)"
    )]
    pub flags: BitFlags<DdsPixelformatFlags>,
    pub four_cc: FourCCEnum,
    pub rgb_bit_count: u32,
    pub r_bit_mask: u32,
    pub g_bit_mask: u32,
    pub b_bit_mask: u32,
    pub a_bit_mask: u32,
}

impl DdsPixelFormat {
    fn read_flags(
        rest: &BitSlice<u8, Msb0>,
    ) -> Result<(&BitSlice<u8, Msb0>, BitFlags<DdsPixelformatFlags>), DekuError> {
        let (rest, value) = u32::read(rest, ())?;
        Ok((rest, BitFlags::from_bits(value).unwrap()))
    }

    fn write_flags(
        output: &mut BitVec<u8, Msb0>,
        flags: &BitFlags<DdsPixelformatFlags>,
    ) -> Result<(), DekuError> {
        let value: u32 = flags.bits();
        value.write(output, ())
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DdsHeaderFlags {
    DDSD_CAPS = 1,
    DDSD_HEIGHT = 2,
    DDSD_WIDTH = 4,
    DDSD_PITCH = 8,
    DDSD_PIXELFORMAT = 4096,
    DDSD_MIPMAPCOUNT = 131072,
    DDSD_LINEARSIZE = 524288,
    DDSD_DEPTH = 8388608,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(type = "u32")]
#[allow(non_camel_case_types)]
pub enum D3D10_Resource_Dimension {
    D3D10_RESOURCE_DIMENSION_UNKNOWN = 0,
    D3D10_RESOURCE_DIMENSION_BUFFER = 1,
    D3D10_RESOURCE_DIMENSION_TEXTURE1D = 2,
    D3D10_RESOURCE_DIMENSION_TEXTURE2D = 3,
    D3D10_RESOURCE_DIMENSION_TEXTURE3D = 4,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(type = "u32")]
#[allow(non_camel_case_types)]
pub enum DxgiFormat {
    DXGI_FORMAT_UNKNOWN = 0,
    DXGI_FORMAT_R32G32B32A32_TYPELESS = 1,
    DXGI_FORMAT_R32G32B32A32_FLOAT = 2,
    DXGI_FORMAT_R32G32B32A32_UINT = 3,
    DXGI_FORMAT_R32G32B32A32_SINT = 4,
    DXGI_FORMAT_R32G32B32_TYPELESS = 5,
    DXGI_FORMAT_R32G32B32_FLOAT = 6,
    DXGI_FORMAT_R32G32B32_UINT = 7,
    DXGI_FORMAT_R32G32B32_SINT = 8,
    DXGI_FORMAT_R16G16B16A16_TYPELESS = 9,
    DXGI_FORMAT_R16G16B16A16_FLOAT = 10,
    DXGI_FORMAT_R16G16B16A16_UNORM = 11,
    DXGI_FORMAT_R16G16B16A16_UINT = 12,
    DXGI_FORMAT_R16G16B16A16_SNORM = 13,
    DXGI_FORMAT_R16G16B16A16_SINT = 14,
    DXGI_FORMAT_R32G32_TYPELESS = 15,
    DXGI_FORMAT_R32G32_FLOAT = 16,
    DXGI_FORMAT_R32G32_UINT = 17,
    DXGI_FORMAT_R32G32_SINT = 18,
    DXGI_FORMAT_R32G8X24_TYPELESS = 19,
    DXGI_FORMAT_D32_FLOAT_S8X24_UINT = 20,
    DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS = 21,
    DXGI_FORMAT_X32_TYPELESS_G8X24_UINT = 22,
    DXGI_FORMAT_R10G10B10A2_TYPELESS = 23,
    DXGI_FORMAT_R10G10B10A2_UNORM = 24,
    DXGI_FORMAT_R10G10B10A2_UINT = 25,
    DXGI_FORMAT_R11G11B10_FLOAT = 26,
    DXGI_FORMAT_R8G8B8A8_TYPELESS = 27,
    DXGI_FORMAT_R8G8B8A8_UNORM = 28,
    DXGI_FORMAT_R8G8B8A8_UNORM_SRGB = 29,
    DXGI_FORMAT_R8G8B8A8_UINT = 30,
    DXGI_FORMAT_R8G8B8A8_SNORM = 31,
    DXGI_FORMAT_R8G8B8A8_SINT = 32,
    DXGI_FORMAT_R16G16_TYPELESS = 33,
    DXGI_FORMAT_R16G16_FLOAT = 34,
    DXGI_FORMAT_R16G16_UNORM = 35,
    DXGI_FORMAT_R16G16_UINT = 36,
    DXGI_FORMAT_R16G16_SNORM = 37,
    DXGI_FORMAT_R16G16_SINT = 38,
    DXGI_FORMAT_R32_TYPELESS = 39,
    DXGI_FORMAT_D32_FLOAT = 40,
    DXGI_FORMAT_R32_FLOAT = 41,
    DXGI_FORMAT_R32_UINT = 42,
    DXGI_FORMAT_R32_SINT = 43,
    DXGI_FORMAT_R24G8_TYPELESS = 44,
    DXGI_FORMAT_D24_UNORM_S8_UINT = 45,
    DXGI_FORMAT_R24_UNORM_X8_TYPELESS = 46,
    DXGI_FORMAT_X24_TYPELESS_G8_UINT = 47,
    DXGI_FORMAT_R8G8_TYPELESS = 48,
    DXGI_FORMAT_R8G8_UNORM = 49,
    DXGI_FORMAT_R8G8_UINT = 50,
    DXGI_FORMAT_R8G8_SNORM = 51,
    DXGI_FORMAT_R8G8_SINT = 52,
    DXGI_FORMAT_R16_TYPELESS = 53,
    DXGI_FORMAT_R16_FLOAT = 54,
    DXGI_FORMAT_D16_UNORM = 55,
    DXGI_FORMAT_R16_UNORM = 56,
    DXGI_FORMAT_R16_UINT = 57,
    DXGI_FORMAT_R16_SNORM = 58,
    DXGI_FORMAT_R16_SINT = 59,
    DXGI_FORMAT_R8_TYPELESS = 60,
    DXGI_FORMAT_R8_UNORM = 61,
    DXGI_FORMAT_R8_UINT = 62,
    DXGI_FORMAT_R8_SNORM = 63,
    DXGI_FORMAT_R8_SINT = 64,
    DXGI_FORMAT_A8_UNORM = 65,
    DXGI_FORMAT_R1_UNORM = 66,
    DXGI_FORMAT_R9G9B9E5_SHAREDEXP = 67,
    DXGI_FORMAT_R8G8_B8G8_UNORM = 68,
    DXGI_FORMAT_G8R8_G8B8_UNORM = 69,
    DXGI_FORMAT_BC1_TYPELESS = 70,
    DXGI_FORMAT_BC1_UNORM = 71,
    DXGI_FORMAT_BC1_UNORM_SRGB = 72,
    DXGI_FORMAT_BC2_TYPELESS = 73,
    DXGI_FORMAT_BC2_UNORM = 74,
    DXGI_FORMAT_BC2_UNORM_SRGB = 75,
    DXGI_FORMAT_BC3_TYPELESS = 76,
    DXGI_FORMAT_BC3_UNORM = 77,
    DXGI_FORMAT_BC3_UNORM_SRGB = 78,
    DXGI_FORMAT_BC4_TYPELESS = 79,
    DXGI_FORMAT_BC4_UNORM = 80,
    DXGI_FORMAT_BC4_SNORM = 81,
    DXGI_FORMAT_BC5_TYPELESS = 82,
    DXGI_FORMAT_BC5_UNORM = 83,
    DXGI_FORMAT_BC5_SNORM = 84,
    DXGI_FORMAT_B5G6R5_UNORM = 85,
    DXGI_FORMAT_B5G5R5A1_UNORM = 86,
    DXGI_FORMAT_B8G8R8A8_UNORM = 87,
    DXGI_FORMAT_B8G8R8X8_UNORM = 88,
    DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM = 89,
    DXGI_FORMAT_B8G8R8A8_TYPELESS = 90,
    DXGI_FORMAT_B8G8R8A8_UNORM_SRGB = 91,
    DXGI_FORMAT_B8G8R8X8_TYPELESS = 92,
    DXGI_FORMAT_B8G8R8X8_UNORM_SRGB = 93,
    DXGI_FORMAT_BC6H_TYPELESS = 94,
    DXGI_FORMAT_BC6H_UF16 = 95,
    DXGI_FORMAT_BC6H_SF16 = 96,
    DXGI_FORMAT_BC7_TYPELESS = 97,
    DXGI_FORMAT_BC7_UNORM = 98,
    DXGI_FORMAT_BC7_UNORM_SRGB = 99,
    DXGI_FORMAT_AYUV = 100,
    DXGI_FORMAT_Y410 = 101,
    DXGI_FORMAT_Y416 = 102,
    DXGI_FORMAT_NV12 = 103,
    DXGI_FORMAT_P010 = 104,
    DXGI_FORMAT_P016 = 105,
    DXGI_FORMAT_420_OPAQUE = 106,
    DXGI_FORMAT_YUY2 = 107,
    DXGI_FORMAT_Y210 = 108,
    DXGI_FORMAT_Y216 = 109,
    DXGI_FORMAT_NV11 = 110,
    DXGI_FORMAT_AI44 = 111,
    DXGI_FORMAT_IA44 = 112,
    DXGI_FORMAT_P8 = 113,
    DXGI_FORMAT_A8P8 = 114,
    DXGI_FORMAT_B4G4R4A4_UNORM = 115,
    DXGI_FORMAT_P208 = 130,
    DXGI_FORMAT_V208 = 131,
    DXGI_FORMAT_V408 = 132,
    DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE,
    DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE,
    DXGI_FORMAT_FORCE_UINT = 0xffffffff,
}

/// https://msdn.microsoft.com/en-us/library/bb943991.aspx
#[derive(Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum DdsPixelFormatEnum {
    D3DFMT_A1R5G5B5,
    D3DFMT_A2B10G10R10,
    D3DFMT_A2R10G10B10,
    D3DFMT_A4L4,
    D3DFMT_A4R4G4B4,
    D3DFMT_A8,
    D3DFMT_A8B8G8R8,
    D3DFMT_A8L8,
    D3DFMT_A8R3G3B2,
    D3DFMT_A8R8G8B8,
    D3DFMT_G16R16,
    D3DFMT_L16,
    D3DFMT_L8,
    D3FMT_R5G6B5,
    D3DFMT_R8G8B8,
    Unknown,
    D3DFMT_X1R5G5B5,
    D3DFMT_X4R4G4B4,
    D3DFMT_X8B8G8R8,
    D3DFMT_X8R8G8B8,
}

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
// #[deku(endian = "little")]
#[deku(magic = b"DDS ")]
pub struct DdsHeader {
    #[deku(assert_eq = "124")]
    pub size: u32,
    #[deku(
        reader = "DdsHeader::read_flags(deku::rest)",
        writer = "DdsHeader::write_flags(deku::output, &self.flags)"
    )]
    pub flags: BitFlags<DdsHeaderFlags>,
    pub height: u32,
    pub width: u32,
    pub pitch_or_linear_size: u32,
    pub depth: u32,
    pub mip_map_count: u32,
    #[deku(count = "11")]
    pub reserved: Vec<u32>,
    pub ddspf: DdsPixelFormat,
    #[deku(
        reader = "DdsHeader::read_caps(deku::rest)",
        writer = "DdsHeader::write_caps(deku::output, &self.caps)"
    )]
    pub caps: BitFlags<DdsCapsFlags>,
    #[deku(
        reader = "DdsHeader::read_caps2(deku::rest)",
        writer = "DdsHeader::write_caps2(deku::output, &self.caps2)"
    )]
    pub caps2: BitFlags<DdsCaps2Flags>,
    pub caps3: u32,
    pub caps4: u32,
    pub reserved2: u32,
    // #[deku(
    //     cond = "ddspf.flags.contains(DdsPixelformatFlags::DDPF_FOURCC) && ddspf.four_cc == 808540228"
    // )]
    #[deku(skip)]
    pub dx10_header: Option<DdsHeaderDX10>,
}

impl DdsHeader {
    pub fn from_stream<R>(reader: &mut R) -> Result<Self, EddsError>
    where
        R: Seek + BufRead,
    {
        let mut buf = [0; 128];
        let read = reader.read(&mut buf)?;
        assert_eq!(read, 128);
        let (_, mut header) = DdsHeader::from_bytes((&buf, 0))?;

        if header
            .ddspf
            .flags
            .contains(DdsPixelformatFlags::DDPF_FOURCC)
            && header.ddspf.four_cc == FourCCEnum::DX10
        {
            let mut buf = [0; 20];
            let read = reader.read(&mut buf)?;
            assert_eq!(read, 20);
            let (_, dx10_header) = DdsHeaderDX10::from_bytes((&buf, 0))?;

            header.dx10_header = Some(dx10_header);
        }

        Ok(header)
    }

    pub fn get_pixel_format(&self) -> DdsPixelFormatEnum {
        match (
            self.ddspf.rgb_bit_count,
            self.ddspf.r_bit_mask,
            self.ddspf.g_bit_mask,
            self.ddspf.b_bit_mask,
            self.ddspf.a_bit_mask,
        ) {
            (16, 0x7C00, 0x3E0, 0x1F, 0x8000) => DdsPixelFormatEnum::D3DFMT_A1R5G5B5,
            (32, 0x3FF, 0xFFC00, 0x3FF00000, 0xC0000000) => DdsPixelFormatEnum::D3DFMT_A2B10G10R10,
            (32, 0x3FF00000, 0xFFC00, 0x3FF, 0xC0000000) => DdsPixelFormatEnum::D3DFMT_A2R10G10B10,
            (8, 0xF, 0x0, 0x0, 0xF0) => DdsPixelFormatEnum::D3DFMT_A4L4,
            (16, 0xF00, 0xF0, 0xF, 0xF000) => DdsPixelFormatEnum::D3DFMT_A4R4G4B4,
            (8, 0x0, 0x0, 0x0, 0xFF) => DdsPixelFormatEnum::D3DFMT_A8,
            (32, 0xFF, 0xFF00, 0xFF0000, 0xFF000000) => DdsPixelFormatEnum::D3DFMT_A8B8G8R8,
            (16, 0xFF, 0x0, 0x0, 0xFF00) => DdsPixelFormatEnum::D3DFMT_A8L8,
            (16, 0xE0, 0x1C, 0x3, 0xFF00) => DdsPixelFormatEnum::D3DFMT_A8R3G3B2,
            (32, 0xFF0000, 0xFF00, 0xFF, 0xFF000000) => DdsPixelFormatEnum::D3DFMT_A8R8G8B8,
            (32, 0xFFFF, 0xFFFF0000, 0x0, 0x0) => DdsPixelFormatEnum::D3DFMT_G16R16,
            (16, 0xFFFF, 0x0, 0x0, 0x0) => DdsPixelFormatEnum::D3DFMT_L16,
            (8, 0xFF, 0x0, 0x0, 0x0) => DdsPixelFormatEnum::D3DFMT_L8,
            (16, 0xF800, 0x7E0, 0x1F, 0x0) => DdsPixelFormatEnum::D3FMT_R5G6B5,
            (24, 0xFF0000, 0xFF00, 0xFF, 0x0) => DdsPixelFormatEnum::D3DFMT_R8G8B8,
            (16, 0x7C00, 0x3E0, 0x1F, 0x0) => DdsPixelFormatEnum::D3DFMT_X1R5G5B5,
            (16, 0xF00, 0xF0, 0xF, 0x0) => DdsPixelFormatEnum::D3DFMT_X4R4G4B4,
            (32, 0xFF, 0xFF00, 0xFF0000, 0x0) => DdsPixelFormatEnum::D3DFMT_X8B8G8R8,
            (32, 0xFF0000, 0xFF00, 0xFF, 0x0) => DdsPixelFormatEnum::D3DFMT_X8R8G8B8,
            (_, _, _, _, _) => DdsPixelFormatEnum::Unknown,
        }
    }

    fn read_flags(
        rest: &BitSlice<u8, Msb0>,
    ) -> Result<(&BitSlice<u8, Msb0>, BitFlags<DdsHeaderFlags>), DekuError> {
        let (rest, value) = u32::read(rest, ())?;
        Ok((rest, BitFlags::from_bits(value).unwrap()))
    }

    /// Parse from String to u8 and write
    fn write_flags(
        output: &mut BitVec<u8, Msb0>,
        flags: &BitFlags<DdsHeaderFlags>,
    ) -> Result<(), DekuError> {
        let value: u32 = flags.bits();
        value.write(output, ())
    }

    fn read_caps(
        rest: &BitSlice<u8, Msb0>,
    ) -> Result<(&BitSlice<u8, Msb0>, BitFlags<DdsCapsFlags>), DekuError> {
        let (rest, value) = u32::read(rest, ())?;
        Ok((rest, BitFlags::from_bits(value).unwrap()))
    }

    fn write_caps(
        output: &mut BitVec<u8, Msb0>,
        flags: &BitFlags<DdsCapsFlags>,
    ) -> Result<(), DekuError> {
        let value: u32 = flags.bits();
        value.write(output, ())
    }

    fn read_caps2(
        rest: &BitSlice<u8, Msb0>,
    ) -> Result<(&BitSlice<u8, Msb0>, BitFlags<DdsCaps2Flags>), DekuError> {
        let (rest, value) = u32::read(rest, ())?;
        Ok((rest, BitFlags::from_bits(value).unwrap()))
    }

    fn write_caps2(
        output: &mut BitVec<u8, Msb0>,
        flags: &BitFlags<DdsCaps2Flags>,
    ) -> Result<(), DekuError> {
        let value: u32 = flags.bits();
        value.write(output, ())
    }
}

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(type = "u32", bytes = "4")]
pub enum FourCCEnum {
    None = 0,
    DXT1 = four_cc_to_u32(FourCC(*b"DXT1")),
    DXT2 = four_cc_to_u32(FourCC(*b"DXT2")),
    DXT3 = four_cc_to_u32(FourCC(*b"DXT3")),
    DXT4 = four_cc_to_u32(FourCC(*b"DXT4")),
    DXT5 = four_cc_to_u32(FourCC(*b"DXT5")),
    DX10 = four_cc_to_u32(FourCC(*b"DX10")),
    ATT1 = four_cc_to_u32(FourCC(*b"ATT1")),
    ATT2 = four_cc_to_u32(FourCC(*b"ATT2")),
    BC4U = four_cc_to_u32(FourCC(*b"BC4U")),
    BC4S = four_cc_to_u32(FourCC(*b"BC4S")),
    BC5U = four_cc_to_u32(FourCC(*b"BC5U")),
    BC5S = four_cc_to_u32(FourCC(*b"BC5S")),
    RGBG = four_cc_to_u32(FourCC(*b"RGBG")),
    GRGB = four_cc_to_u32(FourCC(*b"GRGB")),
}

const fn four_cc_to_u32(four_cc: FourCC) -> u32 {
    (four_cc.0[0] as u32)
        + ((four_cc.0[1] as u32) << 8)
        + ((four_cc.0[2] as u32) << 16)
        + ((four_cc.0[3] as u32) << 24)
}

#[derive(Debug, PartialEq, Eq, DekuRead, DekuWrite)]
pub struct DdsHeaderDX10 {
    pub dxgi_format: DxgiFormat,
    pub resource_dimension: D3D10_Resource_Dimension,
    pub misc_flag: u32,
    pub array_size: u32,
    pub misc_flags2: u32,
}

impl Default for DdsHeaderDX10 {
    fn default() -> Self {
        Self {
            dxgi_format: DxgiFormat::DXGI_FORMAT_UNKNOWN,
            resource_dimension: D3D10_Resource_Dimension::D3D10_RESOURCE_DIMENSION_UNKNOWN,
            misc_flag: 0,
            array_size: 0,
            misc_flags2: 0,
        }
    }
}

use std::io::{BufRead, Seek};

use crate::core::{errors::EddsError, read::ReadExtTrait};

use super::{
    dds_header::{DdsHeader, DxgiFormat},
    DdsPixelFormatEnum, FourCCEnum,
};

use lzzzz::lz4;

#[derive(Debug, Clone)]
pub struct Edds {
    pub mipmaps: Vec<Mipmap>,
}

#[derive(Debug, Clone)]
pub enum MipmapType {
    COPY,
    LZ4,
}

#[derive(Debug, Clone)]
pub struct Mipmap {
    pub width: usize,
    pub height: usize,
    pub data_type: MipmapType,
    pub compressed_data_size: u32,
    pub data: Vec<u8>,
}

impl Edds {
    pub fn from<I>(input: &mut I) -> Result<Edds, EddsError>
    where
        I: Seek + BufRead,
    {
        let header = DdsHeader::from_stream(input)?;
        let mut mipmaps = Vec::new();

        for i in (1..(header.mip_map_count + 1)).rev() {
            let data_type = input.read_string_lossy(4)?;
            let compressed_data_size = input.read_u32()?;
            mipmaps.push(Mipmap {
                width: Edds::get_dim_for_index(header.width, i),
                height: Edds::get_dim_for_index(header.height, i),
                data_type: match data_type.as_str() {
                    "COPY" => MipmapType::COPY,
                    "LZ4 " => MipmapType::LZ4,
                    unk => return Err(EddsError::UnknownImageDataType(format!("{:?}", unk))),
                },
                data: Vec::new(),
                compressed_data_size,
            });
        }

        for mipmap in mipmaps.iter_mut() {
            match mipmap.data_type {
                MipmapType::COPY => {
                    let mut buf = vec![0; mipmap.compressed_data_size as usize];
                    input.read_exact(&mut buf).unwrap();
                    mipmap.data = Edds::decode_data(&buf, mipmap.width, mipmap.height, &header)?;
                }
                MipmapType::LZ4 => {
                    let mut lz4_stream = lz4::Decompressor::new().unwrap();

                    let uncompressed_data_size = input.read_u32().unwrap() as usize;

                    let mut data_read = 4;
                    let mut complete_buffer = Vec::with_capacity(uncompressed_data_size as usize);

                    loop {
                        let compress_block_size = input.read_u24().unwrap() as usize;
                        data_read += 3;

                        let is_last_block = input.read_u8().unwrap() as u32 != 0;
                        data_read += 1;

                        let mut buf = vec![0; compress_block_size];
                        input.read_exact(&mut buf).unwrap();

                        data_read += compress_block_size;

                        let mut block_size = 65536;
                        if is_last_block {
                            block_size = uncompressed_data_size - complete_buffer.len();
                        }

                        let decomp = lz4_stream.next(&buf, block_size as usize).unwrap();
                        complete_buffer.append(&mut decomp.to_owned());

                        if is_last_block {
                            assert_eq!(data_read, mipmap.compressed_data_size as usize);
                            break;
                        }
                    }

                    mipmap.data =
                        Edds::decode_data(&complete_buffer, mipmap.width, mipmap.height, &header)?;
                }
            };
        }

        Ok(Edds { mipmaps })
    }

    fn get_dim_for_index(max_dim: u32, index: u32) -> usize {
        std::cmp::max(max_dim / 2_u32.pow(index - 1), 1) as usize
    }

    fn decode_data(
        src: &[u8],
        width: usize,
        height: usize,
        header: &DdsHeader,
    ) -> Result<Vec<u8>, EddsError> {
        match &header.dx10_header {
            Some(dx10_header) => decode_dx10_data(dx10_header, src, width, height),
            None => decode_four_cc_data(header, src, width, height),
        }
    }
}

fn decode_four_cc_data(
    header: &DdsHeader,
    src: &[u8],
    width: usize,
    height: usize,
) -> Result<Vec<u8>, EddsError> {
    match &header.ddspf.four_cc {
        FourCCEnum::None => decode_pixel_format_data(header, src),
        FourCCEnum::DXT5 => {
            let bc5 = texpresso::Format::Bc5;
            let mut output = vec![0; width * height * 4];
            bc5.decompress(src, width, height, &mut output);
            Ok(output)
        }
        ni_four_cc => Err(EddsError::UnknownImageDataFormat(format!(
            "{:?}",
            ni_four_cc
        ))),
    }
}

fn decode_pixel_format_data(header: &DdsHeader, src: &[u8]) -> Result<Vec<u8>, EddsError> {
    match header.get_pixel_format() {
        DdsPixelFormatEnum::D3DFMT_X8R8G8B8 | DdsPixelFormatEnum::D3DFMT_A8R8G8B8 => {
            let mut src = src.to_vec();
            for i in (0..src.len()).step_by(4) {
                let r = src[i];
                let b = src[i + 2];

                src[i] = b;
                src[i + 2] = r;
            }
            Ok(src)
        }
        unk => Err(EddsError::UnknownImageDataFormat(format!("{:?}", unk))),
    }
}

fn decode_dx10_data(
    dx10_header: &super::DdsHeaderDX10,
    src: &[u8],
    width: usize,
    height: usize,
) -> Result<Vec<u8>, EddsError> {
    match dx10_header.dxgi_format {
        DxgiFormat::DXGI_FORMAT_BC4_UNORM => Ok(bcndecode::decode(
            src,
            width,
            height,
            bcndecode::BcnEncoding::Bc4,
            bcndecode::BcnDecoderFormat::LUM,
        )?),
        DxgiFormat::DXGI_FORMAT_B8G8R8X8_UNORM_SRGB => {
            let mut src = src.to_vec();
            for i in (0..src.len()).step_by(4) {
                let r = src[i];
                let b = src[i + 2];

                src[i] = b;
                src[i + 2] = r;
            }
            Ok(src)
        }
        DxgiFormat::DXGI_FORMAT_BC7_UNORM_SRGB => {
            let mut dst = vec![0_u8; width * height * 4];
            bcdec_rust::bcdec_bc7_unorm_safer(src, width, height, &mut dst);
            Ok(dst)
        }
        _ => Err(EddsError::UnknownImageDataFormat(format!(
            "{:?}",
            dx10_header.dxgi_format
        ))),
    }
}

use std::{fs::File, io::BufReader};

use eff::edds::Edds;
use serial_test::serial;

const INPUT_PATH_PREFIX: &str = "./tests/test-data/edds_in/";
#[allow(dead_code)]
const OUTPUT_PATH_PREFIX: &str = "./tests/test-data/edds_out/";

fn export_mipmaps(edds: &Edds, filename: &str, color_type: image::ColorType) {
    for (i, mipmap) in edds.mipmaps.iter().enumerate() {
        image::save_buffer(
            format!("{}{}.out.{}.png", OUTPUT_PATH_PREFIX, filename, i),
            &mipmap.data,
            mipmap.width as u32,
            mipmap.height as u32,
            color_type,
        )
        .unwrap();
    }
}

#[test]
#[serial]
fn edds_bc4_test() {
    let file = File::open(format!("{}prop_bc4.edds", INPUT_PATH_PREFIX)).unwrap();
    let edds = Edds::from(&mut BufReader::new(file)).unwrap();

    export_mipmaps(&edds, "prop_bc4", image::ColorType::L8);
}

#[test]
#[serial]
fn edds_bc7_test() {
    let file = File::open(format!("{}car_bc7.edds", INPUT_PATH_PREFIX)).unwrap();
    let edds = Edds::from(&mut BufReader::new(file)).unwrap();

    export_mipmaps(&edds, "car_bc7", image::ColorType::Rgba8);
}

#[test]
#[serial]
fn edds_rgba_test() {
    let file = File::open(format!("{}uaz_rgba.edds", INPUT_PATH_PREFIX)).unwrap();
    let edds = Edds::from(&mut BufReader::new(file)).unwrap();

    export_mipmaps(&edds, "uaz_rgba", image::ColorType::Rgba8);
}

#[test]
#[serial]
fn edds_non_dx10_header_test() {
    let file = File::open(format!("{}optic.edds", INPUT_PATH_PREFIX)).unwrap();
    let edds = Edds::from(&mut BufReader::new(file)).unwrap();

    export_mipmaps(&edds, "optic", image::ColorType::Rgba8);
}

#[test]
#[serial]
fn eden_1337_layer_test() {
    let file = File::open(format!("{}Eden_1337_layer.edds", INPUT_PATH_PREFIX)).unwrap();
    let edds = Edds::from(&mut BufReader::new(file)).unwrap();

    export_mipmaps(&edds, "Eden_1337_layer_test", image::ColorType::Rgba8);
}

#[test]
#[serial]
fn eden_1337_normal_test() {
    let file = File::open(format!("{}Eden_1337_normal.edds", INPUT_PATH_PREFIX)).unwrap();
    let edds = Edds::from(&mut BufReader::new(file)).unwrap();

    export_mipmaps(&edds, "Eden_1337_normal_test", image::ColorType::Rgba8);
}

#[test]
#[serial]
fn eden_1337_supertexture_test() {
    let file = File::open(format!("{}Eden_1337_supertexture.edds", INPUT_PATH_PREFIX)).unwrap();
    let edds = Edds::from(&mut BufReader::new(file)).unwrap();

    export_mipmaps(
        &edds,
        "Eden_1337_supertexture_test",
        image::ColorType::Rgba8,
    );
}

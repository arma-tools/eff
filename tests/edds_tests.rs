use std::{fs::File, io::BufReader};

use eff::edds::Edds;

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
fn edds_bc4_test() {
    let file = File::open(format!("{}prop_bc4.edds", INPUT_PATH_PREFIX)).unwrap();
    let edds = Edds::from(&mut BufReader::new(file)).unwrap();

    export_mipmaps(&edds, "prop_bc4", image::ColorType::L8);
}

#[test]
fn edds_bc7_test() {
    let file = File::open(format!("{}car_bc7.edds", INPUT_PATH_PREFIX)).unwrap();
    let edds = Edds::from(&mut BufReader::new(file)).unwrap();

    export_mipmaps(&edds, "car_bc7", image::ColorType::Rgba8);
}

#[test]
fn edds_rgba_test() {
    let file = File::open(format!("{}uaz_rgba.edds", INPUT_PATH_PREFIX)).unwrap();
    let edds = Edds::from(&mut BufReader::new(file)).unwrap();

    export_mipmaps(&edds, "uaz_rgba", image::ColorType::Rgba8);
}

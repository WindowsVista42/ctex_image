use ctex::par_encode_all;
use std::io::Write;

fn main() {
    let mut encodes = par_encode_all("input/*.png").unwrap();

    encodes.iter_mut().for_each(|(img, name)| {
        let file = std::fs::File::create(&*format!("output/{}.ctex", name)).unwrap();
        let mut encoder = lz4::EncoderBuilder::new().level(9).build(file).unwrap();
        encoder.write_all(img.as_slice()).unwrap();
        encoder.finish().1.unwrap();
    });
}

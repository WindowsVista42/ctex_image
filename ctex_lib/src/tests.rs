#[cfg(test)]
mod tests {
    use crate::decode::{avx2_decode, decode_path, decode_raw, sse2_decode};
    use crate::encode::{encode_path, encode_raw};
    use crate::flag::Flags;
    use crate::{flag, CtexImage};
    use std::io::Write;

    const INPUT_LUT: [u32; 3] = [5, 4, 6];
    const INPUT_CTEX: [u8; 128] = [
        0, 1, 1, 1, //
        0, 1, 1, 1, //
        1, 1, 1, 1, //
        1, 1, 2, 1, //
        1, 1, 1, 1, //
        1, 1, 1, 1, //
        1, 1, 1, 1, //
        1, 1, 1, 1, //
        //------------
        0, 1, 1, 1, //
        0, 1, 1, 1, //
        1, 1, 1, 1, //
        1, 1, 2, 1, //
        1, 1, 1, 1, //
        1, 1, 1, 1, //
        1, 1, 1, 1, //
        1, 1, 1, 1, //
        //============
        0, 1, 1, 1, //
        0, 1, 1, 1, //
        1, 1, 1, 1, //
        1, 1, 2, 1, //
        1, 1, 1, 1, //
        1, 1, 1, 1, //
        1, 1, 1, 1, //
        1, 1, 1, 1, //
        //------------
        0, 1, 1, 1, //
        0, 1, 1, 1, //
        1, 1, 1, 1, //
        1, 1, 2, 1, //
        1, 1, 1, 1, //
        1, 1, 1, 1, //
        1, 1, 1, 1, //
        1, 1, 1, 1, //
    ];

    const OUTPUT_CTEX: [u32; 128] = [
        5, 5, 4, 4, 4, 4, 4, 4, 5, 5, 4, 4, 4, 4, 4, 4, //
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, //
        4, 4, 4, 6, 4, 4, 4, 4, 4, 4, 4, 6, 4, 4, 4, 4, //
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, //
        //================================================
        5, 5, 4, 4, 4, 4, 4, 4, 5, 5, 4, 4, 4, 4, 4, 4, //
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, //
        4, 4, 4, 6, 4, 4, 4, 4, 4, 4, 4, 6, 4, 4, 4, 4, //
        4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, //
    ];

    #[test]
    fn test_avx2_decode() {
        let out = avx2_decode(&INPUT_LUT, &INPUT_CTEX);

        assert_eq!(out.len(), INPUT_CTEX.len());
        assert_eq!(out.as_slice(), &OUTPUT_CTEX);
    }

    #[test]
    fn test_sse2_decode() {
        let out = sse2_decode(&INPUT_LUT, &INPUT_CTEX);

        assert_eq!(out.len(), INPUT_CTEX.len());
        assert_eq!(out.as_slice(), &OUTPUT_CTEX);
    }

    #[test]
    fn test_runtime_detection_decode() {
        let out = decode_raw(&INPUT_LUT, &INPUT_CTEX, Flags::default_no_compression());

        assert_eq!(out.len(), INPUT_CTEX.len());
        assert_eq!(out.as_slice(), &OUTPUT_CTEX);
    }

    #[test]
    fn test_encode() {
        let out = encode_raw(&OUTPUT_CTEX, Flags::default_no_compression());

        assert_eq!(out.0.len(), INPUT_LUT.len());
        assert_eq!(out.1.len(), INPUT_CTEX.len());
        assert_eq!(out.0.as_slice(), &INPUT_LUT);
        assert_eq!(out.1.as_slice(), &INPUT_CTEX);
    }

    #[test]
    fn test_round_trip() {
        let (lut, offsets, flags) = encode_raw(&OUTPUT_CTEX, Flags::default());
        let out = decode_raw(&*lut, &*offsets, Flags::default_no_compression());

        assert_eq!(out.len(), OUTPUT_CTEX.len());
        assert_eq!(out, &OUTPUT_CTEX);
    }

    #[test]
    fn test_encode_path() {
        let CtexImage {
            flags,
            lut,
            offsets,
        } = encode_path("test/test.png", Flags::default_no_compression());

        let mut file = std::fs::File::create("test/test.ctex").unwrap();
        unsafe {
            file.write_all(flags.as_u64().as_ne_bytes());
            file.write_all(lut.as_slice().align_to::<u8>().1);
            file.write_all(offsets.as_slice());
        }

        let img = decode_path("test/test.ctex");

        let png = image::open("test/test.png").unwrap()
            .to_rgba8()
            .pixels()
            .map(|p| unsafe { p.0.align_to::<u32>().1[0] })
            .collect::<Vec<u32>>();

        assert_eq!(img, png);
    }
}
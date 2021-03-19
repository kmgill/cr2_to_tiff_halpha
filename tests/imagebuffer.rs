use cr2_to_tiff_halpha::imagebuffer::ImageBuffer;

#[test]
fn load_cr2() {
    let image = ImageBuffer::from_cr2("testing/IMG_0107.CR2").unwrap();
    assert_eq!(image.width, 4770);
    assert_eq!(image.height, 3176);
}

#[test]
fn load_cr2_extract_red() {
    let image = ImageBuffer::from_cr2("testing/IMG_0107.CR2").unwrap();
    let red = image.red().unwrap();
    assert_eq!(red.width, 2385);
    assert_eq!(red.height, 1588);
}

#[test]
fn load_cr2_check_min_max_no_override() {
    let image = ImageBuffer::from_cr2("testing/IMG_0107.CR2").unwrap();
    let red = image.red().unwrap();
    let minmax = red.get_min_max(-1.0).unwrap();
    assert_eq!(minmax.min, 935.0);
    assert_eq!(minmax.max, 1223.0);
}

#[test]
fn load_cr2_check_min_max_with_override() {
    let image = ImageBuffer::from_cr2("testing/IMG_0107.CR2").unwrap();
    let red = image.red().unwrap();
    let minmax = red.get_min_max(400.0).unwrap();
    assert_eq!(minmax.min, 400.0);
    assert_eq!(minmax.max, 1223.0);
}

#[test]
fn load_cr2_scalar() {
    let image = ImageBuffer::from_cr2("testing/IMG_0107.CR2").unwrap();
    let red = image.red().unwrap();
    let scaled = red.scale(2.0).unwrap();
    let minmax = scaled.get_min_max(-1.0).unwrap();
    assert_eq!(minmax.min, 935.0 * 2.0);
    assert_eq!(minmax.max, 1223.0 * 2.0);
}

#[test]
fn load_cr2_divide_into() {
    let image = ImageBuffer::from_cr2("testing/IMG_0107.CR2").unwrap();
    let red = image.red().unwrap();
    let scaled = red.divide_into(2.0).unwrap();
    let minmax = scaled.get_min_max(-1.0).unwrap();
    assert_eq!(minmax.min, 0.0016353229);
    assert_eq!(minmax.max, 0.0021390375);
}
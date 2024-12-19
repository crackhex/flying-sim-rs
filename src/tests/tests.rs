#[cfg(test)]
mod tests {
    use crate::includes::trig_table::coss;
    use crate::utils::m64_handling::M64File;
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;
    use std::time::Instant;

    #[test]
    fn test_trig() {
        let time = Instant::now();
        let mut array: [f32; u16::MAX as usize + 1] = [0.0; 65536];
        let _x = (i16::MIN..i16::MAX).into_iter().for_each(|i| {
            array[i.cast_unsigned() as usize] = coss(i);
        });
        //println!("{:?}, {:?}", &array, &time.elapsed())
    }

    #[test]
    fn test_m64() {
        let test_m64 = Path::new("src\\tests\\test_r.m64");
        let m64 = M64File::read_file(test_m64).unwrap();
        let test_w = Path::new("src\\tests\\test_w.m64");
        let _x = m64.write_file(test_w).unwrap();
        let mut buf1: Vec<u8> = Vec::new();
        let mut buf2: Vec<u8> = Vec::new();
        File::open(test_m64)
            .unwrap()
            .read_to_end(&mut buf1)
            .unwrap();
        File::open(test_w).unwrap().read_to_end(&mut buf2).unwrap();
        assert_eq!(buf1, buf2);
        println!("\n aa{:?}", m64.inputs[0]);
    }
}

use crate::includes::trig_table::coss;
use std::time::Instant;

#[test]
fn test_trig() {
    let time = Instant::now();
    let mut array: [f32; u16::MAX as usize + 1] = [0.0; 65536];
    let _x = (i16::MIN..i16::MAX).into_iter().for_each(|i| {
        array[i.cast_unsigned() as usize] = coss(i);
    });
    println!("{:?}, {:?}", &array, &time.elapsed())
}

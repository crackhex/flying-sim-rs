use std::time::Instant;
use crate::includes::trig_table::coss;

#[test]
fn test_trig() {
    let time = Instant::now();
    let mut array: [f32; u16::MAX as usize + 1] = [0.0; 65536];
    let x = (u16::MIN..u16::MAX).into_iter().for_each(
        |i| {
            array[i as usize] = coss(i);

        }
    );
    println!("{:?}, {:?}", &array, &time.elapsed())
}



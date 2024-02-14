//use std::{thread, time, time::Instant};
use std::time::Instant;

fn r(mul: i32, shift: i32, x: &mut i32, y: &mut i32) -> i32 {
    let temp = *x;
    *x -= mul * *y >> shift;
    *y += mul * temp >> shift;
    let temp = 3145728 - *x * *x - *y * *y >> 11;
    *x = *x * temp >> 10;
    *y = *y * temp >> 10;
    temp
}

fn main() {
    let mut b = vec![32u8; 1760];
    let mut z = vec![127i8; 1760];
    let (mut sa, mut ca, mut sb, mut cb) = (1024i32, 0i32, 1024i32, 0i32);

    print!("\x1B[2J");

    let mut last_instant = Instant::now();
    let mut frame_count = 0;

    loop {
        for i in b.iter_mut() {
            *i = 32; // Clearing the text buffer
        }
        for i in z.iter_mut() {
            *i = 127; // Clearing the z buffer
        }

        let (mut sj, mut cj) = (0i32, 1024i32);
        for _j in 0..90 {
            let (mut si, mut ci) = (0i32, 1024i32);
            for _i in 0..324 {
                let r1 = 1;
                let r2 = 2048;
                let k2 = 5120 * 1024;
                let x0 = r1 * cj + r2;
                let x1 = ci * x0 >> 10;
                let x2 = ca * sj >> 10;
                let x3 = si * x0 >> 10;
                let x4 = r1 * x2 - (sa * x3 >> 10);
                let x5 = sa * sj >> 10;
                let x6 = k2 + r1 * 1024 * x5 + ca * x3;
                let x7 = cj * si >> 10;
                let x = 40 + 30 * (cb * x1 - sb * x4) / x6;
                let y = 12 + 15 * (cb * x4 + sb * x1) / x6;
                let n = (-ca * x7 - cb * ((-sa * x7 >> 10) + x2) - ci * (cj * sb >> 10) >> 10) - x5
                    >> 7;
                let o = x + 80 * y;
                let zz = ((x6 - k2) >> 15) as i8;
                if 22 > y && y > 0 && x > 0 && 80 > x && zz < z[o as usize] {
                    z[o as usize] = zz;
                    b[o as usize] = b".,-~:;=!*#$@"[n.max(0) as usize];
                }
                r(5, 8, &mut ci, &mut si);
            }
            r(9, 7, &mut cj, &mut sj);
        }
        print!("\x1b[H");
        for k in 0..1760 {
            if k % 80 != 0 {
                print!("{}", b[k] as char);
            } else {
                println!("");
            }
        }
        r(5, 7, &mut ca, &mut sa);
        r(5, 8, &mut cb, &mut sb);

        //thread::sleep(time::Duration::from_millis(15));
        //print!("\x1b[23A");

        frame_count += 1;

        if last_instant.elapsed().as_secs_f32() >= 1.0 {
            println!("FPS: {}", frame_count);
            frame_count = 0;
            last_instant = Instant::now();
        }
    }
}

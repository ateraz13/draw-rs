// Practically useless

#[allow(dead_code)]
pub fn u32_sqrt( v : u32 ) -> u32
{
    let mut rem = 0usize;
    let mut root = 0usize;
    let mut a = v as usize;

    for _ in 0 .. 16 {
        root <<= 1;
        rem <<= 2;
        rem += a >> 30;
        a <<= 2;

        if root < rem{
            root += 1;
            rem -= root;
            root += 1;
        }
    }
    root as u32 >> 1
}


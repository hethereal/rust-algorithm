
/// Karatsuba Multiplication
pub fn karatsuba(x: &mut usize, y: &mut usize) -> usize {
    //转成字符串，方便进行切分
    let a = &mut x.to_string();
    let b = &mut y.to_string();

    //拆分被乘数
    let p1: &usize = &a[0..a.len() / 2].parse().unwrap();
    let p2: &usize = &a[a.len() / 2..a.len()].parse().unwrap();

    //拆分乘数
    let p3: &usize = &b[0..b.len() / 2].parse().unwrap();
    let p4: &usize = &b[b.len() / 2..b.len()].parse().unwrap();

    println!("{},{},{},{}", p1, p2, p3, p4);

    let res = p1 * p3 * 10usize.pow(a.len() as u32) +
        (((p1 + p2) * (p3 + p4) - (p1 * p3) - (p2 * p4)) * 10usize.pow((a.len() / 2) as u32))
        + (p2 * p4);
    res
}
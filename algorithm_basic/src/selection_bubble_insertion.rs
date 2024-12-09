//! 选择、冒泡、插入排序

/// 选择排序
fn selection_sort<T: Ord>(v: &mut Vec<T>) {
    if v.len() < 2 { return; }

    for i in 0..v.len() - 1 {
        let mut min = i;
        for j in i +1..v.len() {
            if v[j] < v[min] {
                min = j;
            }
        }
        v.swap(i, min);
    }
}


/// 冒泡排序
fn bubble_sort<T: Ord>(v: &mut Vec<T>) {
    if v.len() < 2 { return; }

    for end in (1..v.len()).rev() {
        for i in 0..end {
            if v[i] > v[i + 1] {
                v.swap(i, i + 1);
            }
        }
    }
}

///插入排序
fn insertion_sort<T: Ord>(v: &mut Vec<T>) {
    if v.len() < 2 { return; }
    for i in 1..v.len() {
        for j in (0..=i-1).rev() {
            if v[j] <= v[j+1] { continue; }
            else { v.swap(j, j+1); }
        }
    }
}




#[cfg(test)]
mod test {
    use rand::{thread_rng, Rng};
    use crate::selection_bubble_insertion::{bubble_sort, insertion_sort, selection_sort};

    #[test]
    fn test01() {
        let mut rng = thread_rng();
        let mut v = Vec::new();
        for i in 0..10 {
            v.push(rng.gen_range(0..=100));
        }
        println!("{:?}", v);
        //selection_sort(&mut v);
        //bubble_sort(&mut v);
        insertion_sort(&mut v);
        println!("{:?}", v);
    }

    #[test]
    fn test02() {
        let mut rng = thread_rng();
        let mut v = Vec::new();
        for i in 0..10 {
            v.push(rng.gen_range(0..=100));
        }
        println!("{:?}", v);
        selection_sort(&mut v);
        bubble_sort(&mut v);
        println!("{:?}", v);
    }
}
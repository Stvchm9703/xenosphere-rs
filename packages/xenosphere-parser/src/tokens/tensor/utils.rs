pub fn merge_shape<T>(a_shape: &Vec<T>, b_shape: &Vec<T>) -> Vec<T>
where
    T: Clone + PartialEq + Default + PartialOrd,
{
    if a_shape.len() == 0 {
        return b_shape.clone();
    }
    if b_shape.len() == 0 {
        return a_shape.clone();
    }
    let mut base_iter = a_shape.clone();
    let mut check_iter = b_shape.clone();
    if a_shape.len() > b_shape.len() {
        let mut cloned = b_shape.clone();
        cloned.resize(a_shape.len(), T::default());
        check_iter = cloned;
    } else if b_shape.len() > a_shape.len() {
        let mut cloned = a_shape.clone();
        cloned.resize(b_shape.len(), T::default());
        base_iter = cloned;
    }
    let zip_iter = base_iter.iter().zip(check_iter.iter());
    zip_iter
        .map(|(x, y)| if x < y { y.clone() } else { x.clone() })
        .collect::<Vec<_>>()
    // base_iter.map(|x| *x).collect::<Vec<i32>>()
}

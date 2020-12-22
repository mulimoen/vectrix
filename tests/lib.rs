use vectrs::Vector;

#[test]
fn vector_from_array_or_tuple() {
    assert_eq!(Vector::from([0]), Vector::from((0,)));
    assert_eq!(Vector::from([0, 1]), Vector::from((0, 1)));
    assert_eq!(Vector::from([0, 1, 2]), Vector::from((0, 1, 2)));
    assert_eq!(Vector::from([0, 1, 2, 3]), Vector::from((0, 1, 2, 3)));
    assert_eq!(Vector::from([0, 1, 2, 3, 4]), Vector::from((0, 1, 2, 3, 4)));
    assert_eq!(
        Vector::from([0, 1, 2, 3, 4, 5]),
        Vector::from((0, 1, 2, 3, 4, 5))
    );
    assert_eq!(
        Vector::from([0, 1, 2, 3, 4, 5, 6]),
        Vector::from((0, 1, 2, 3, 4, 5, 6))
    );
    assert_eq!(
        Vector::from([0, 1, 2, 3, 4, 5, 6, 7]),
        Vector::from((0, 1, 2, 3, 4, 5, 6, 7))
    );
    assert_eq!(
        Vector::from([0, 1, 2, 3, 4, 5, 6, 7, 8]),
        Vector::from((0, 1, 2, 3, 4, 5, 6, 7, 8))
    );
    assert_eq!(
        Vector::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        Vector::from((0, 1, 2, 3, 4, 5, 6, 7, 8, 9))
    );
    assert_eq!(
        Vector::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        Vector::from((0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10))
    );
    assert_eq!(
        Vector::from([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]),
        Vector::from((0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11))
    );
}

#[test]
fn vector_from_slice() {
    assert_eq!(
        Vector::<_, 3>::from(&[0, 1, 2][..]),
        Vector::from((0, 1, 2))
    );
}

#[test]
fn vector_from_vec() {
    assert_eq!(Vector::<_, 3>::from(vec![0, 1, 2]), Vector::from((0, 1, 2)));
}

#[test]
fn vector_components() {
    let vector = Vector::from((1, 2, 3, 4));
    assert_eq!(vector.x(), 1);
    assert_eq!(vector.y(), 2);
    assert_eq!(vector.z(), 3);
    assert_eq!(vector.w(), 4);
}

#[test]
fn vector_debug() {
    let vector = Vector::from((1, 2, 3, 4));
    assert_eq!(format!("{:?}", vector), "[1, 2, 3, 4]");
}

#[test]
fn vector_components_ref() {
    let vector = Vector::from((1, 2, 3, 4));
    assert_eq!(vector.x_ref(), &1);
    assert_eq!(vector.y_ref(), &2);
    assert_eq!(vector.z_ref(), &3);
    assert_eq!(vector.w_ref(), &4);
}

#[test]
fn vector_components_mut() {
    let mut vector = Vector::from((1, 2, 3, 4));
    *vector.x_mut() = 5;
    assert_eq!(vector.x(), 5);
}

#[test]
fn vector_add() {
    let a = Vector::from((1, 2, 3));
    let b = Vector::from((1, -2, 3));
    let c = Vector::from((2, 0, 6));
    let vector = a + b;
    assert_eq!(vector, c);

    let vector = a + &b;
    assert_eq!(vector, c);

    let vector = &a + &b;
    assert_eq!(vector, c);
}

#[test]
fn vector_sub() {
    let a = Vector::from((1, 2, 3));
    let b = Vector::from((1, -2, 1));
    let c = Vector::from((0, 4, 2));

    let vector = a - b;
    assert_eq!(vector, c);

    let vector = a - &b;
    assert_eq!(vector, c);

    let vector = &a - &b;
    assert_eq!(vector, c);
}

#[test]
fn vector_mul() {
    let a = Vector::from((1, -2, 3));
    let b = 2;
    let c = Vector::from((2, -4, 6));

    let vector = a * b;
    assert_eq!(vector, c);

    let vector = &a * b;
    assert_eq!(vector, c);

    let vector = a * &b;
    assert_eq!(vector, c);

    let vector = &a * &b;
    assert_eq!(vector, c);
}

#[test]
fn vector_add_assign() {
    let a = Vector::from((1, 2, 3));
    let b = Vector::from((1, -2, 3));
    let c = Vector::from((2, 0, 6));

    let mut vector = a.clone();
    vector += b;
    assert_eq!(vector, c);

    let mut vector = a.clone();
    vector += &b;
    assert_eq!(vector, c);
}

#[test]
fn vector_sub_assign() {
    let a = Vector::from((1, 2, 3));
    let b = Vector::from((1, -2, 1));
    let c = Vector::from((0, 4, 2));

    let mut vector = a.clone();
    vector -= b;
    assert_eq!(vector, c);

    let mut vector = a.clone();
    vector -= &b;
    assert_eq!(vector, c);
}

#[test]
fn vector_as_slice() {
    let vector = Vector::from((1, 2, 3, 4));
    assert_eq!(vector.as_slice(), &[1, 2, 3, 4]);
}

#[test]
fn vector_iter() {
    let vector = Vector::from((1, 2, 3, 4));
    let values: Vec<_> = vector.iter().collect();
    assert_eq!(values, vec![&1, &2, &3, &4]);
}

#[test]
fn vector_into_iter() {
    let vector = Vector::from((1, 2, 3, 4));
    let values: Vec<_> = vector.into_iter().collect();
    assert_eq!(values, vec![1, 2, 3, 4]);
}

#[test]
fn vector_into_iter_rev() {
    let vector = Vector::from((1, 2, 3, 4));
    let values: Vec<_> = vector.into_iter().rev().collect();
    assert_eq!(values, vec![4, 3, 2, 1]);
}

#[test]
fn vector_into_iter_skip_rev() {
    let vector = Vector::from((1, 2, 3, 4));
    let values: Vec<_> = vector.into_iter().skip(1).rev().skip(1).collect();
    assert_eq!(values, vec![3, 2]);
}

#[test]
fn vector_into_iter_count() {
    let vector = Vector::from((1, 2, 3, 4));
    assert_eq!(vector.into_iter().count(), 4);
    assert_eq!(vector.into_iter().skip(1).count(), 3);
    assert_eq!(vector.into_iter().skip(1).rev().skip(1).count(), 2);
}

#[test]
fn vector_collect() {
    let vector: Vector<_, 4> = vec![1, 2, 3, 4].into_iter().collect();
    assert_eq!(vector, Vector::from([1, 2, 3, 4]));
}

#[test]
fn vector_collect_too_long() {
    let _vector: Vector<_, 3> = vec![1, 2, 3, 4, 5].into_iter().collect();
}

#[test]
#[should_panic]
fn vector_collect_too_short() {
    let _vector: Vector<_, 4> = vec![1, 2].into_iter().collect();
}

#[test]
fn vector_new() {
    const VECTOR: Vector<i64, 2> = Vector::new([1, 2]);
    assert_eq!(VECTOR, Vector::from([1, 2]));
}

#[test]
fn vector_unsigned_abs() {
    assert_eq!(
        Vector::<u8, 3>::from((1, 2, 3)).abs(),
        Vector::<u8, 3>::from((1, 2, 3))
    );
}

#[test]
fn vector_signed_abs() {
    assert_eq!(
        Vector::<i8, 3>::from((-1, 2, -3)).abs(),
        Vector::<i8, 3>::from((1, 2, 3))
    );
}

#[test]
fn vector_unsigned_l1_norm() {
    assert_eq!(Vector::<u8, 3>::from((1, 2, 3)).l1_norm(), 6);
}

#[test]
fn vector_signed_l1_norm() {
    assert_eq!(Vector::<i8, 3>::from((-1, 2, -3)).l1_norm(), 6);
}

#[test]
fn vector_dot() {
    let a = Vector::from((1, 2, 3));
    let b = Vector::from((4, 5, 6));
    assert_eq!(a.dot(&b), 32);
    assert_eq!(b.dot(&a), 32);
}

#[test]
fn vector_reduced() {
    assert_eq!(
        Vector::from((4, -8, 12)).reduced(),
        Vector::from([1, -2, 3])
    )
}

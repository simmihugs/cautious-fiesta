fn main() {
    let mut v = (1..10).collect::<Vec<i32>>();
    v.iter().for_each(|x| {
        println!("{x}");
    });

    update2(&mut v);
    //update(&mut v);

    println!("");
    v.iter().for_each(|x| {
        println!("{x}");
    });
}

fn update(v: &mut Vec<i32>) {
    v.retain(|x| *x % 2 == 0);
}

fn update3(v: &mut Vec<i32>) {
    // O(N) because needs to shift everything into new place
    v.remove(3);
}

fn update2(v: &mut Vec<i32>) {
    // like remove, but faster, because last element gets put into place of replaced element
    // so messes order up
    v.swap_remove(3);
}

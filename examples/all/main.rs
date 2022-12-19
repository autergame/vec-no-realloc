extern crate vec_no_realloc;

use vec_no_realloc::{VecNoRealloc, vnr};

fn main() {
    let test3: VecNoRealloc<u32> = vnr![];
    println!("{}", test3);

    let test4: VecNoRealloc<u32> = vnr![5; 10];
    println!("{}", test4);

    let test5: VecNoRealloc<u32> = vnr![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    println!("{}", test5);

    let mut test2 = VecNoRealloc::<String>::with_capacity(10);

    test2.push("test".to_string());
    println!("{}", test2);

    let valstr = test2.pop();
    println!("{:?} {}", valstr, test2);

    test2.push("tset".to_string());
    println!("{:?} {}", valstr, test2);

    let mut test = VecNoRealloc::<usize>::new();

    for i in 0..15 {
        test.push(i);
    }
    println!("{}", test);

    println!("len: {} capacity: {}", test.len(), test.capacity());

    println!("{:?} {:?}", test.to_vec(), test.to_vec_ref());

    let mut val = test.pop().unwrap();
    val *= 25;
    test.push(30);
    val *= 50;
    println!("{:?} {}", val, test[14]);

    for item in &test {
        print!("{} ", item);
    }
    println!();

    for item in &mut test {
        *item *= 2;
    }
    println!("{}", test);

    for i in 0..9 {
        test[i] *= 3;
    }
    
	for i in 0..16 {
		print!("{:?} ", test.get(i));
	}
    println!();

    for i in 0..10 {
        test.push(i);
    }

	for i in 0..25 {
		print!("{:?} ", test[i]);
	}
    println!();

    for i in 5..20 {
        test[i] *= 4;
    }
    println!("{}", test);

    let topop = test.len() + 1;
    for i in 0..topop {
		if (i % 5) == 0 {
			println!();
			println!("{:?}", test);
			println!();
		}
        print!("{:?}", test.pop_del(true));
        if i < topop - 1 {
            print!(", ");
        }
    }
    println!();

    println!("{:#?}", test);
}

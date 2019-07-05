
/// How to use a strategies and create them from a factory.
mod factory {
    #[derive(Debug)]
    pub struct Stuff {
        val: usize,
    }

    pub trait Strategy {
        fn do_stuff(&self) -> Stuff;
    }

    // Dynamic dispatch, ploymorphisation => compiler will emit only one function and the code will use a vtable to find the right method to call.
    pub fn algorithm(strategy: &dyn Strategy) -> Stuff {
        strategy.do_stuff()
    }

    // Dynamic dispatch, ploymorphisation => compiler will emit only one function and the code will use a vtable to find the right method to call.
    // Static dispatch, monomorphisation => compiler will emit two functions. One for
    // algorithm2(S1) and one for algorithm2(S2)
    // => The compiler will emit _3_ versions of the method, i.e., one for dynamic dispatch and two
    // for static dispatch
    // Use rustc --emit llvm-ir and look for `; main::factory::algorithm2` in the resulting file
    pub fn algorithm2<T: Strategy + ?Sized>(strategy: &T) -> Stuff {
        strategy.do_stuff()
    }

    pub struct S1 {}
    impl Strategy for S1 {
        fn do_stuff(&self) -> Stuff {
            Stuff { val: 1 }
        }
    }

    pub struct S2 {}
    impl Strategy for S2 {
        fn do_stuff(&self) -> Stuff {
            Stuff { val: 2 }
        }
    }

    pub fn factory(s: usize) -> Box<dyn Strategy> {
        match s {
            1 => Box::new(S1 {}),
            2 => Box::new(S2 {}),
            _ => panic!("No such strategy"),
        }
    }

    pub fn main() {
        let s1_box: Box<Strategy> = factory(1);
        let s1 = S1 {};

        let _ = algorithm(&*s1_box); // dyn dispatch; compiler does not known what concret type is in the Box.
        let _ = algorithm(&s1); // dyn dispatch; compiler knows the exact type but the method only takes trait objects

        let _ = algorithm2(&*s1_box); // dyn dispatch; compiler does not known what concret type is in the Box.
        let _ = algorithm2(&s1); // static dispatch; compiler knows the exact type and the method takes concrete types


        let s2_box: Box<Strategy> = factory(2);
        let s2 = S2 {};

        let _ = algorithm(&*s2_box); // dyn dispatch; compiler does not known what concret type is in the Box.
        let _ = algorithm(&s2); // dyn dispatch; compiler knows the exact type but the method only takes trait objects

        let _ = algorithm2(&*s2_box); // dyn dispatch; compiler does not known what concret type is in the Box.
        let _ = algorithm2(&s2); // static dispatch; compiler knows the exact type and the method takes concrete types

        let s3: Box<Strategy> = factory(2);
        let s3_ref: &dyn Strategy = &*s3;
        let _ = algorithm(s3_ref); // dyn dispatch; compiler does not known what concret type is in the Box.
        let _ = algorithm2(s3_ref); // dyn dispatch; compiler does not known what concret type is in the Box.
     }

}

fn main() {
    factory::main();
}

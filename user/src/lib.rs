use exporter::Cake;

// This function will make compilation fail if T does not implement Cake.
fn assert_is_cake<T: Cake>() {}

mod a {
    use super::*;

    // This struct name starts with a Vowel, so the Cake proc_macro will
    // generate a NewSchoolCake struct, followed by its Cake trait
    // implementation.
    #[derive(Cake)]
    struct A;

    // This functions checks at compile-time that Cake is implemented for
    // NewSchoolCake.
    fn test_cake_implementation() {
        // Here, we get the following error:
        //
        // error[E0277]: the trait bound `a::NewSchoolCake: core::Cake` is not satisfied
        //   --> user/src/lib.rs:18:26
        //      |
        //   4  | fn assert_is_cake<T: Cake>() {}
        //      |                      ---- required by this bound in `assert_is_cake`
        //   ...
        //   28 |         assert_is_cake::<NewSchoolCake>();
        //      |                          ^^^^^^^^^^^^^ the trait `core::Cake` is not implemented for `a::NewSchoolCake`
        assert_is_cake::<NewSchoolCake>();
    }
}

mod b {
    use super::*;

    // This struct name starts with a consonant, so the Cake proc_macro will
    // generate the Cake trait implemetation of OldSchoolCake, followed by the
    // OldSchoolCake declaration.
    #[derive(Cake)]
    struct B;

    // This functions checks at compile-time that Cake is implemented for
    // OldSchoolCake.
    fn test_cake_implementation() {
        assert_is_cake::<OldSchoolCake>();
    }
}

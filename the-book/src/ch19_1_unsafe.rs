// 1. Using unsafe to dereference a raw pointer

#[test]
fn raw1() {
    let mut num = 5;

    // const and mut pointers, they can be created outside the unsafe area
    // but can't be accessed.  Also, mutable and immutable pointers can point
    // to the same location without the borrow checker complaining, yey!
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        assert_eq!(5, *r1);
        assert_eq!(5, *r2);
        *r2 = 26;
        assert_eq!(26, *r1);
        assert_eq!(26, *r2);
    }
}

// 2. Using unsafe to call an unsafe function

unsafe fn dangerous() -> i32 {
    let foo = 6;
    let p1 = &foo as *const i32;
    
    // we can use unsafe code without the unsafe block inside an unsafe function
    *p1 + 4
}

#[test]
fn unsafe_fn() {
    unsafe {
        assert_eq!(10, dangerous());
    }

}

// external functions are always unsafe.  Not sure what magic makes
// this work without me telling where to link abs 
extern "C" {
    fn abs(input: i32) -> i32;
}

#[test]
fn extern1() {
    unsafe {
        assert_eq!(26, abs(-26));
    }
}

// 3. Using unsafe to access or modify a mutable static variable
static mut COUNTER : i32 = 0;

fn inc_counter() {
    unsafe {
        COUNTER += 1;
    }
}

#[test]
fn mut_stat_var() {
    unsafe {
        assert_eq!(0, COUNTER);
    }
    inc_counter();
    unsafe {
        assert_eq!(1, COUNTER);
    }
}

// 4. Using Unsafe to implement an unsafe trait.  
// TODO: not sure exactly what an unsafe trait is.
unsafe trait UnsafeFoo {
}

unsafe impl UnsafeFoo for i32 
}

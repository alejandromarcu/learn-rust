#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]


mod ch19_1_unsafe;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io;
use std::io::Read;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)] // This trait allows for printing
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(u8, u8, u8);

#[test]
fn struct1() {
    let mut qq = User {
        username: String::from("Alejandro Marcu"),
        email: "amarcu@gmail.com".to_owned(),
        sign_in_count: 0,
        active: true,
    };

    qq.sign_in_count = 1;

    assert!(qq.active);
    assert_eq!(qq.sign_in_count, 1);
    assert_eq!(qq.username, "Alejandro Marcu");

    // field init shorthand
    let username = "Pepe".to_owned();
    let active = false;
    let _u2 = User {
        username,
        active,
        email: "a@a".to_owned(),
        sign_in_count: 0,
    };

    // struct update
    let u3 = User {
        active: false,
        ..qq // copy all the other fields from qq
    };

    assert!(!u3.active);
    assert_eq!(u3.username, "Alejandro Marcu");

    println!("{:?}", u3); // this is to try using the Debug trait for the struct
}

#[test]
fn struct2() {
    let color = Color(255, 255, 0);
    assert_eq!(color.0, 255);
    assert_eq!(color.2, 0);
}

struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    pub fn area(&self) -> i32 {
        self.height * self.width
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

// you can define a class in more than 1 impl
impl Rectangle {
    // This is an "associated function", i.e. a static method, can be used as a constructor
    pub fn square(dim: i32) -> Rectangle {
        Rectangle {
            width: dim,
            height: dim,
        }
    }
}

#[test]

fn method1() {
    let rect = Rectangle {
        width: 10,
        height: 20,
    };
    assert_eq!(200, rect.area());

    assert!(rect.can_hold(&Rectangle {
        width: 10,
        height: 10
    }));

    let sq = Rectangle::square(26);
    assert_eq!(26, sq.width);
    assert_eq!(26, sq.height);
}

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(String),
    V6(String),
}

// You can define methods for an enum
impl IpAddr {
    pub fn foo(&self) -> &IpAddr {
        self
    }
}

#[test]
fn enum1() {
    let four = IpAddrKind::V4;

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
    home.foo();
}

enum MyOption<T> {
    Some(T),
    None,
}

// impl <T> MyOption<T> {
//     pub fn unwrap_or(&self, other : &T) -> T {
//         match self {
//             MyOption::Some(x) => other,
//             MyOption::None => other
//         }
//     }
// }

#[test]
fn enum2() {
    let some_number = MyOption::Some(26);
    let none_number: MyOption<i32> = MyOption::None;
}

#[test]
fn match1() {
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let str_home = match home {
        IpAddr::V4(s) => "ip4 ".to_owned() + &s,
        IpAddr::V6(s) => "ip6 ".to_owned() + &s,
    };

    assert_eq!("ip4 127.0.0.1", str_home);
}

#[test]
fn iflet1() {
    let x = Some(26);
    let mut count = 0;

    // it's like doing just 1 branch of match
    if let Some(z) = x {
        count += z;
    }

    if let Some(26) = x {
        count += 100;
    }

    if let None = x {
        count += 1;
    }

    assert_eq!(126, count);
}

pub mod blah {
    pub fn foo() {}
}

mod sound {
    // re-exporting a name
    pub use crate::blah;

    fn privatish() -> i8 {
        26
    }

    pub mod instrument {
        pub fn clarinet() -> i8 {
            // super goes one module up
            super::privatish()
        }
    }
}

#[test]
fn mod1() {
    // absolute path, starts with "crate"
    assert_eq!(26, crate::sound::instrument::clarinet());

    // relative path
    assert_eq!(26, sound::instrument::clarinet());

    // just trying the re-export
    sound::blah::foo();
}

#[test]
fn vec1() {
    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    // accessing the vector with square brackets it doesn't check boundaries
    assert_eq!(2, v1[1]);
    // with get it returns an Optional
    assert_eq!(Some(&2), v1.get(1));

    // macro for easy initialization
    let mut v2 = vec![1, 2, 3];

    // iterating over the vector
    for i in &v2 {
        println!("{}", i);
    }

    // iterating and mutating
    for i in &mut v2 {
        *i += 100;
    }
    assert_eq!(101, v2[0]);
}

#[test]
fn str1() {
    let mut s = String::new();
    s.push('H');
    s.push_str("ello ");
    s += "World!";
    assert_eq!("Hello World!", s);

    let s1 = String::from("Cucu");
    let s2 = format!("{}, {}", s, s1);
    assert_eq!("Hello World!, Cucu", s2);

    let hello = "Здравствуйте";
    // this takes 4 bytes, and we don't know how many actual chars it represent, in this example 2
    let part = &hello[0..4];
    assert_eq!("Зд", part);

    // how to iterate through a string
    for ch in hello.chars() {
        println!("{}", ch);
    }

    // Quick hack to get a char from an ASCII string
    let ch_ascii: char = s.as_bytes()[0] as char;
    assert_eq!('H', ch_ascii);
}

#[test]
fn hashmap1() {
    let mut scores = HashMap::new();
    let blue = String::from("Blue");
    scores.insert(blue, 10);
    scores.insert(String::from("Yellow"), 50);

    //println!("{}", blue); The hashmap took ownership of blue, so this gives a compiler error

    // iterating over the hashmap
    for (k, v) in &scores {
        println!("{}, {}", k, v);
    }

    // insert overrides the previous value if it existed
    scores.insert(String::from("Yellow"), 60);

    // this would just write the value if there was not a previous value
    scores.entry(String::from("Red")).or_insert(26);
    scores.entry(String::from("Red")).or_insert(666);
    assert_eq!(Some(&26), scores.get(&String::from("Red")));

    // Updating values
    let text = "hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // this returns a mutable reference to the entry
        *count += 1;
    }
    assert_eq!(Some(&2), map.get("world"));

    // A hashmap can be created from collecting tuples
    let squares: HashMap<_, _> = (1..5).map(|x| (x, x * x)).collect();
    assert_eq!(Some(&16), squares.get(&4));
}

#[test]
fn result1() {
    let f = File::open("doesntexist.qq");
    assert!(f.is_err());

    // not sure how reliable is the path
    let file = File::open("src/main.rs");
    let _file = match file {
        Ok(fx) => fx,
        Err(error) => panic!("Expected to find the file! Instead got error {}", error),
    };

    // Another way to deal with this, more compact
    let _file2 = File::open("src/main.rs").expect("Failed to open file");
}

fn read_file(fname: &String) -> Result<String, io::Error> {
    // The question mark operator will unwrap if the result is ok, or otherwise return from the function with Err.
    let mut f = File::open(fname)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

#[test]
fn question_mark1() {
    assert!(read_file(&"blah.qq".to_owned()).is_err());
    assert!(read_file(&"src/main.rs".to_owned()).is_ok());
}

// Custom types for validation
struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!("The value needs to be between 1 and 100");
        }
        Guess { value }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

fn take_guess(guess: &Guess) -> i32 {
    guess.value()
}

#[test]
fn custom_types_for_validation() {
    let g = Guess::new(26);
    assert_eq!(26, take_guess(&g));
}

// Generic data types
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn only_on_f32(&self) {}
}

#[test]
fn generic1() {
    let my_point = Point { x: 26, y: 10 };
    assert_eq!(&26, my_point.x());

    let my_f_point = Point { x: 1.1, y: 2.2 };
    my_f_point.only_on_f32();
}

// Traits: we can use them to specify that a generic can be any type that has a certain behavior.
// It's similar to an interface.
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Shortcut way of passing a trait as a parameter
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify2<T: Summary>(item1: T, item2: T) {
    println!("news: {}, {}", item1.summarize(), item2.summarize());
}

// We can specify multiple trait bounds.
pub fn notify3(item: impl Summary + Display) {
    println!("Breaking news! {}: {}", item.summarize(), item);
}

impl Summary for String {
    fn summarize(&self) -> String {
        if self.len() > 5 {
            format!("{}...", &self[0..5])
        } else {
            self.clone()
        }
    }
}

// Another syntax to make it more clear when you have many traits
#[allow(dead_code)]
fn some_function<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Clone + Summary,
{
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut biggest = list[0];
    for &elem in list.iter() {
        if elem > biggest {
            biggest = elem;
        }
    }
    biggest
}

fn largest_with_ref<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];
    for elem in list.iter() {
        if elem > biggest {
            biggest = &elem;
        }
    }
    biggest
}

// Example of a blanket implementation.  For anything that has the trait Display, we can onw add the
// MehString trait automatically
pub trait MehString {
    fn meh_string(&self) -> String;
}

impl<T: Display> MehString for T {
    fn meh_string(&self) -> String {
        format!("meh {}", self)
    }
}

#[test]
fn traits1() {
    assert_eq!(
        String::from("Hello..."),
        String::from("Hello World!").summarize()
    );
    assert_eq!(String::from("Hey"), String::from("Hey").summarize());
    let ints = vec![1, 2, 3, 10, 4, 5, 3, 2];
    assert_eq!(10, largest(&ints[..]));

    let strings = vec!["Hey".to_owned(), "Cucu".to_owned(), "Abc".to_owned()];
    assert_eq!("Hey".to_owned(), *largest_with_ref(&strings[..]));

    assert_eq!(
        String::from("meh hello"),
        String::from("hello").meh_string()
    );
    assert_eq!(String::from("meh 26"), 26.meh_string());
}

// Lifetimes
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// lifetime in a struct
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[test]
fn lifetimes1() {
    assert_eq!("this is long", longest("this is long", "meh"));

    let text = String::from("Hello world. yadda yadda yadda");

    let first_sentence = text.split('.').next().expect("No end of sentence!");

    let e = ImportantExcerpt {
        part: first_sentence,
    };

    assert_eq!(e.part, "Hello world");
}

// Testing
#[cfg(test)]
mod tests {
    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic(expected = "ITO")]
    fn oohoh() {
        panic!("ITO");
    }

    // would be run using
    // cargo test -- --ignored
    #[test]
    #[ignore]
    fn expensive_slow_test_ignored_by_default() {
        assert_eq!(1 + 1, 2);
    }
}

// To see what passing tests printed as well, run with:
// cargo test -- --nocapture

// Rust supports 2 types of tests:
// - unit tests, in the same file than the code, they can test private functions
// - initegration tests, in tests/ directory

#[test]
fn closures1() {
    // type inference
    let sq = |x| x * x;
    assert_eq!(4, sq(2));

    // explicit
    let sq2 = |x: f32| -> f32 { x * x };
    assert_eq!(2.25, sq2(1.5));
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    result: Option<u32>,
    input: u32,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T, input: u32) -> Cacher<T> {
        Cacher {
            calculation,
            result: None,
            input,
        }
    }

    fn calculate(&mut self) -> u32 {
        //self.result = self.result.or_else(self.calculation);
        if self.result == None {
            self.result = Some((self.calculation)(self.input));
        }
        self.result.unwrap()
    }
}

#[test]
fn closures2() {
    let sq = |x| {
        println!("Calculating");
        x * x
    };
    let mut calc = Cacher::new(sq, 3);
    assert_eq!(9, calc.calculate());
    assert_eq!(9, calc.calculate());
}

#[test]
fn closures3() {
    let x = vec![1, 2, 3];
    // move indicates that the captured variables are NOT borrowed and will be moved.
    let eq_to_x = move |z| z == x;
    eq_to_x(vec![1, 2]);

    // this wouldn't work, because we moved x
    //println!("{:?}", x);
}

// Iterators:
// - iter()        : over immutable references
// - iter_mut()    : over mutable references
// - into_owner()  : takes ownership
#[test]
fn iterator1() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}

// Creating an iterator
struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn iter2() {
    let s = Counter::new().sum::<u32>();
    assert_eq!(15, s);
}

// 14. More about Cargo and Crates
// Cargo has two main profiles, dev and release (use --release to build or run release).

// this is just to demo "pub use" in lib.rs
//use crate::PrimaryColor;

// In order to split big projects, use cargo workspaces: https://doc.rust-lang.org/stable/book/ch14-03-cargo-workspaces.html

// To install a binary crate (e.g. a tool), use for example:
// cargo install ripgrep

// *** 15. Smart pointers ***
// Box
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

#[test]
fn box1() {
    let list : List = Cons(100, Box::new(Cons(20, Box::new(Cons(3, Box::new(Nil))))));

    let mut head = &list;
    let mut sum = 0;
    loop {
        match head {
            Nil => break,
            Cons(n, next) => {
                sum += n;
                head = next;
            }
        }
    }
    assert_eq!(123, sum);
}

// ** Deref **
struct MyBox<T> (T);

impl<T> MyBox<T> {
    fn new(x : T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
} 

#[test]
fn deref1() {
    let x = MyBox(26);
    assert_eq!(26, *x);
}

// Deref coercion can call deref on the argument to convert to the expected type
fn test_deref_coercion(name : &str) {
    println!("Hello {}", name);
}

#[test]
fn deref2() {
    test_deref_coercion(&String::from("Ale"));
    test_deref_coercion(&Box::new(String::from("Ale")));
}

// ** Refereced Counted Smart Pointer **
enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}

fn sum_listrc(list : &ListRc) -> i32 {
    let mut head = list;
    let mut total = 0;
    loop {
        match head {
            ListRc::Nil => break,
            ListRc::Cons(num, next) => {
                total += num;
                head = &next; 
            }
        }
    }
    total
}

#[test]
fn rc1() {
    let a = Rc::new(ListRc::Cons(5, Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil)))));
    let b = ListRc::Cons(3, Rc::clone(&a));
    let c = ListRc::Cons(4, Rc::clone(&a));

    assert_eq!(15, sum_listrc(&a));
    assert_eq!(18, sum_listrc(&b));
    assert_eq!(19, sum_listrc(&c));
}

// ** 15.6 RefCell **
// It allows to bend the rule of either 1 mutable reference or immutable references.
// Still, this is checked during runtime (vs normally checked during compiling)
// The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler
// is unable to understand and guarantee that.
// Mutating the value inside an immutable value is the interior mutability pattern.

#[test]
fn refcell1() {
    let r : RefCell<String> = RefCell::new(String::from("Hello"));
    {
        let val = r.borrow();
        assert_eq!("Hello", *val);
    }
    r.borrow_mut().push_str(" World!");
    assert_eq!("Hello World!", *r.borrow());
}

#[test]
#[should_panic]
fn refcell2() {
    let r : RefCell<String> = RefCell::new(String::from("Hello"));
    let giveme = r.borrow();
    r.borrow_mut().push_str(" World!"); // oh no, we're borrowing mutable something that was already borrowed!
    println!("{}", *giveme);
}

// 15.6 Reference cycles can leak memory
// TODO: follow examples
// You can use Weak references that don't imply ownership (as opposed to Strong), and the risk
// is that they could be invalid if the memory was deallocated.

// 16. Fearless Concurrency
// TODO

// 17. OOP
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components : Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// 17.3 Implementing an OOP pattern
struct Post {}
impl Post {
    pub fn new() -> DraftPost {
        DraftPost{ content : String::new() }
    }
}

struct DraftPost {
    content : String,
}
impl DraftPost {
    pub fn add_text(&mut self, text : &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReview {
        PendingReview { content : self.content }
    }
}

struct PendingReview {
    content : String,
}
impl PendingReview {
    pub fn approve(self) -> Approved {
        Approved { content : self.content }
    }
}
struct Approved {
    content : String,
}
impl Approved {
    pub fn content(&self) -> &str {
        &self.content
    }
}

#[test]
fn test_blog() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}

// 19.6 Macros
#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut v = Vec::new();
            $(
                v.push($x);
                v.push($x);
            )*
            v
        }
    }
}

#[test]
fn macro1() {
    let v1 = vec![1,1,2,2,3,3];
    let v2 = my_vec![1,2,3];
    assert_eq!(v1, v2);
}


fn main() {
   println!("Hello, world!");
}


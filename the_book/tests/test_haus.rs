use the_book;
use the_book::lib::haus;

#[test]
fn test_add() {
    let result = the_book::add(2, 2);
    assert_eq!(result, 4);
}

#[test]
fn test_yell() {
    let opt = ", world!";
    let into = haus::yell();
    assert_ne!(into, opt, "`{into}` != `{opt}`");
}

#[test]
#[should_panic]
fn access_it() {
    let vec: Vec<i32> = (1..10).collect();
    vec[100];
}

#[test]
//#[should_panic(expected = "Paremter between 0 and 10")]
#[should_panic(expected = "index out of bounds")]
fn access_it_twice() {
    let vec: Vec<i32> = (1..10).collect();
    vec[100];
}

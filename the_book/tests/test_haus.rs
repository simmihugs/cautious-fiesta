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
    assert_eq!(into, opt, "`{into}` != `{opt}`");
}

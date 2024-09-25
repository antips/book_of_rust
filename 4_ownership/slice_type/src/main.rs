fn main() {
    without_slice_type();
    string_slices_syntax();
    with_slice_type();
    slice_to_allow_both_mutable_string_and_literal_string();
}

fn without_slice_type() {
    let mut s = String::from("hello world");

    let _word = _first_word_v1(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

fn _first_word_v1(s: &String) -> usize {
    let bytes = s.as_bytes(); // Convert string to an array of bytes

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn string_slices_syntax() {
    let s = String::from("hello world");

    let hello = &s[..5]; // Equivalent to [0..5]
    let world = &s[6..]; // Equivalent to [6..len], with "let len = s.len();"

    println!("hello={hello}, world={world}");
}

fn with_slice_type() {
    let mut _s = String::from("hello world");

    let word = _first_word_v2(&_s);

    // _s.clear(); // Compilation error if we execute this !!

    println!("the first word is: {word}");
}

fn _first_word_v2(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn slice_to_allow_both_mutable_string_and_literal_string() {
    let my_string = String::from("hello world");

    // `_first_word_v3` works on slices of `String`s, whether partial or whole
    let _word = _first_word_v3(&my_string[0..6]);
    let _word = _first_word_v3(&my_string[..]);
    // `_first_word_v3` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = _first_word_v3(&my_string);

    let my_string_literal = "hello world";

    // `_first_word_v3` works on slices of string literals, whether partial or whole
    let _word = _first_word_v3(&my_string_literal[0..6]);
    let _word = _first_word_v3(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = _first_word_v3(my_string_literal);
}

fn _first_word_v3(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

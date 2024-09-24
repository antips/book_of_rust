fn main() {
    reference_for_calculate_length();
    immutable_borrowed_value_cannot_be_modified();
    mutable_reference();
    illegal_multiple_mut_refs();
    legal_multiple_mut_refs();
}

fn reference_for_calculate_length() {
    let s1 = String::from("hello");
    let len = _calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");
}

fn _calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn immutable_borrowed_value_cannot_be_modified() {
    let s = String::from("hello");
    _change_immutable_ref(&s);
}

fn _change_immutable_ref(_some_string: &String) {
    // some_string.push_str(", world"); // Does NOT work, because refs are immutable:
    // "cannot borrow `*some_string` as mutable, as it is behind a `&` reference"
}

fn mutable_reference() {
    let mut s = String::from("hello");
    println!("[mutable_reference] Before: {s}");
    _change_mutable_ref(&mut s);
    println!("[mutable_reference] After : {s}");
}

fn _change_mutable_ref(some_string: &mut String) {
    some_string.push_str(", world");
}

fn illegal_multiple_mut_refs() {
    let mut s = String::from("hello");

    let _r1 = &mut s;
    // let r2 = &mut s; // Compilation will fail: "cannot borrow `s` as mutable
    //more than once at a time"

    // println!("{}, {}", r1, r2);
}

fn legal_multiple_mut_refs() {
    let mut s = String::from("hello");

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let _r2 = &mut s;
}

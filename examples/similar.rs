use similar::{ChangeTag, TextDiff};

fn main() {
    let diff = TextDiff::from_lines(
        "Hello, WOrld, \nThis is an example.\nthis is Rust\n",
        "Hallo, world, \nThis is a example.\nThis is Rust\n",
    );

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "- ",
            ChangeTag::Insert => "+ ",
            ChangeTag::Equal => "",
        };
        print!("{}{}", sign, change);
    }
}

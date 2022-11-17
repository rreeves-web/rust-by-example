// The `Rust Playground`[url]https://play.rust-lang.org/[/url] is a way to
// experiment with Rust code through a web interface.

// Using it with mdbook

// In `mdbook`[url]https://github.com/rust-lang/mdBook[/url], you can make
// code examples playable and editable.
fn main() {
    println!("Hello, world!");
}
// This allows the reader to both run your code sample, but also modify and tweak
// it. The key here is adding the word `editable` to your codefence block
// separated by a comma.
```rust,editable
//...place your code here
```

// Additionally, you can add `ignore` if you want `mdbook` to skip your code when 
// it builds and tests.

// Using it with docs

// You may have noticed in some of the `official Rust docs` a button that says "Run",
// which opens the code sample up in a new tab in Rust Playground. This feature is
// enabled if you use the @[doc] attribute called `html_playground_url`.
//https://doc.rust-lang.org/rustdoc/the-doc-attribute.html#html_playground_url

// See also:
// - https://play.rust-lang.org/
// - https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html

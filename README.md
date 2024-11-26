[![Build Status](https://github.com/idanarye/subqueue/workflows/CI/badge.svg)](https://github.com/idanarye/subqueue/actions)

# Subqueue

## Usage

Prerequisites:

* Git: https://git-scm.com/ (alternatively you can download it directly from GitHub)
* Rust: https://rustup.rs/

1. Clone the repository:
    ```sh
    git clone https://github.com/idanarye/subqueue
    ```
2. Enter the workdir:
    ```sh
    cd subqueue
    ```
3. Run it with the blog's URL
    ```sh
    cargo run html https://blog-name.substack.com > blog-entries.html
    ```
    Replace `https://blog-name.substack.com` with the URL of the blog you want to scrape. If that blog has it's own domain, feel free to use that instead of the `***.substack.com` one.

    Note that this step can take some time, because Cargo will need to build the application (the first time) and because Substack rate-limits the requests.

This will generate an HTML page named `blog-entries.html` with links to all the blog's entries. Use that page to binge-read.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

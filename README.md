# Scrape books from a website

Find all books that have `Rust` in their name and save the title, poster URL, and book URL to a `books.csv` file and the poster to an image with the same name as it's on the site.

## Try it out!
1. Install [Rust](https://rustup.rs/)
2. Run the app
```bash
$ cargo run --release
```
It will output all books with a keyword `Rust` in them, save those to a `books.csv` file, and save posters for those books locally

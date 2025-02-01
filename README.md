# 🔍 Searcher - Your Ultimate File Detective 🚀

Welcome to **Searcher**, a blazing-fast file manager built with Rust! ⚡ It scans files in the `../files` directory and helps you find the *hidden gems* of text you're looking for. Whether you're hunting for keywords, secret messages, or just checking file contents, Searcher has got your back! 😄

## ✨ Features

- **Ultra-Fast Search**: Thanks to Rust's concurrency powers, Searcher scans multiple files in parallel! 🚀
- **Simple & Intuitive**: No complicated commands—just run and find your data. 📂
- **Efficient Filtering**: Only scans `.txt` files to keep things speedy. ⚡

## 🚀 How to Use

1. **Clone the Repository:**
   ```bash
   git clone https://github.com/piter231/searcher.git
   cd searcher
   ```

2. **Build & Run:**
   ```bash
   cargo run --release DATA
   ```

   Replace `DATA` with the word or phrase you want to search for.

3. **Example Output:**
   ```
   Finished `release` profile [optimized] target(s) in 0.05s
   Running `target/release/searcher DATA`
   file:
   ../files/data.txt
   Content:
   YOU FOUND DATA!!!
   ```

## 🛠️ How It Works

- **Input**: Provide the search query as a command-line argument.
- **Search**: The app scans all `.txt` files inside `../files/`.
- **Output**: Displays file paths and matching lines containing your query.

## 📦 Project Structure

```
searcher/
├── src/
│   └── main.rs
├── Cargo.toml
└── ../files/   # Your target files live here
```

## 🤔 Why Use Searcher?

- Super **fast and lightweight**. 🏎️
- Built with **Rust's concurrency** magic. ✨
- **Simple CLI** for effortless searching. 💻

## 💡 Future Improvements

- Recursive directory search
- Support for more file types
- Highlighting matched keywords

## 🙌 Contributing

Feel free to fork, improve, and send pull requests! Contributions are *welcome* with open arms. 🤗

## 📄 License

Licensed under the MIT License.

---

Happy Searching! 🔍✨


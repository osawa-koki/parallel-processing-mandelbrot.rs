# parallel-processing-mandelbrot.rs

🧌🧌🧌 Rustで並列処理でマンデルブロ集合を描画してみる！  

## 実行方法

DevContainerに入り、以下のコマンドを実行します。  

```shell
# モジュールのインストール
cargo install --path .

# ベンチマークの実行  
# 実行結果は`./target/criterion/report/index.html`に出力されます。
cargo bench

# 開発用の実行
cargo run

# ビルド＆実行
cargo build --release && ./target/release/parallel-processing-mandelbrot-rs
```

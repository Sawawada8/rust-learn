# 統合テストについて

project が liblary で lib.rs を持つときは、下記の記述で
```rs
extern crate vcr_cassettes_server;
```

main.rs を持つ project は、統合テストはできない感じ
というのも、複数のlib.rs を作成する形で、構成していけば、
main.rs で呼び出す処理は、非常にシンプルになる
そのシンプルなファイルをテストする必要が無い

https://doc.rust-jp.rs/book-ja/ch11-03-test-organization.html

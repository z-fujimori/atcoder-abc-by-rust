# メソッド集
- 部分文字列判定 
    ```rust
    let result = "abcdef".contains("bcd");
    println!("{}", result); // true
    ```


# i文字目からj文字目までを切り出す
```rust
    let s = "あいうえおかきくけこ";
    
    let n = 2; // 飛ばす文字数 (i - 1)
    let m_count = 4; // 切り出す文字数 (j - i + 1)
    
    let sub: String = s.chars()
        .skip(n)
        .take(m_count)
        .collect();
        
    println!("{}", sub); // うえお
```
byteでよければ下記はO(1)。上はO(n)かかる。
```rust
    let bytes = s.as_bytes();
    let s2: &[u8] = &bytes[head..head + t_len];
```
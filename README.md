# THIS PROJECT MOVED TO GITLAB
[https://gitlab.com/radiobrowser/libraries/icecast-stats](https://gitlab.com/radiobrowser/libraries/icecast-stats)

# Icecast Server Status
Decode server status from an [icecast server](https://icecast.org/docs/icecast-trunk/server_stats/).

## Usage example
```rust
use icecast_stats::fetch;

let url_str = "https://stream.example.com:8000/somestream";
let stats = fetch(url_str).unwrap();
println!("{:#?}", stats);
```

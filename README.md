# Magnet URL Parser

A simple, efficient magnet URL parser in Rust with zero dependencies.

## What is a magnet URL?

A magnet URL is a URI scheme that identifies files by their hash, normally used in peer-to-peer file sharing networks (like BitTorrent). Magnet links identify a torrent you want to download and tell the torrent client how to download it. They make it very easy to share files over the internet, using a combination of DHT and trackers to find peers who can share the file with you.

## Features

- Zero dependencies for lightweight integration
- Simple, efficient string parsing
- Comprehensive magnet URL component support
- Builder pattern for easy creation
- Proper error handling
- 100% safe Rust

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
magnet-url = "3.0.0"
```

### Parsing a Magnet URL

```rust
use magnet_url::Magnet;

fn main() {
    let magnet_str = "magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel";
    
    // Parse the magnet URL
    match Magnet::new(magnet_str) {
        Ok(magnet) => {
            println!("Display name: {:?}", magnet.display_name());
            println!("Hash type: {:?}", magnet.hash_type());
            println!("Hash: {:?}", magnet.hash());
            println!("Trackers: {:?}", magnet.trackers());
        },
        Err(err) => {
            println!("Error parsing magnet URL: {}", err);
        }
    }
}
```

### Creating a Magnet URL

Use the builder pattern to create magnet URLs:

```rust
use magnet_url::MagnetBuilder;

fn main() {
    let magnet = MagnetBuilder::new()
        .display_name("My Torrent")
        .hash_type("btih")
        .hash("1234567890abcdef1234567890abcdef12345678")
        .length(12345)
        .add_tracker("udp://tracker.example.com:6969")
        .add_trackers(&["udp://tracker2.example.com:6969", "wss://tracker3.example.com"])
        .web_seed("https://example.com/seed")
        .build();
    
    println!("Generated magnet URL: {}", magnet.to_string());
}
```

### Error Handling

The library uses proper error handling with the `Result` type:

```rust
use magnet_url::{Magnet, MagnetError};

fn main() {
    // This will return an Err(MagnetError::NotAMagnetURL)
    let result = Magnet::new("https://example.com");
    
    match result {
        Ok(magnet) => {
            println!("Display name: {:?}", magnet.display_name());
        },
        Err(MagnetError::NotAMagnetURL) => {
            println!("The provided string is not a valid magnet URL");
        }
    }
}
```

### Converting to String

You can convert a `Magnet` instance back to a string:

```rust
use magnet_url::MagnetBuilder;

fn main() {
    let magnet = MagnetBuilder::new()
        .display_name("My Torrent")
        .hash_type("btih")
        .hash("1234567890abcdef1234567890abcdef12345678")
        .build();
    
    // Convert to a magnet URL string
    let magnet_url = magnet.to_string();
    println!("{}", magnet_url);
    // Output: magnet:?xt=urn:btih:1234567890abcdef1234567890abcdef12345678&dn=My%20Torrent
}
```

## Supported Magnet Components

All standard magnet URL components are supported:

- `dn` - Display Name
- `xt` - Exact Topic (hash type and hash)
- `xl` - Exact Length
- `tr` - Tracker URL
- `kt` - Keyword Topic
- `ws` - Web Seed
- `xs` - Exact Source
- `as` - Acceptable Source
- `mt` - Manifest Topic

### Accessing Magnet Components

Each component can be accessed through getter methods:

```rust
use magnet_url::Magnet;

fn main() {
    let magnet = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel").unwrap();
    
    // Access components
    println!("Display name: {:?}", magnet.display_name());
    println!("Hash type: {:?}", magnet.hash_type());
    println!("Hash: {:?}", magnet.hash());
    println!("Exact Length: {:?}", magnet.length());
    println!("Tracker URLs: {:?}", magnet.trackers());
    println!("Web Seed: {:?}", magnet.web_seed());
    println!("Source: {:?}", magnet.source());
    println!("Search Keywords: {:?}", magnet.search_keywords());
    println!("Acceptable Source: {:?}", magnet.acceptable_source());
    println!("Manifest: {:?}", magnet.manifest());
}
```

## Performance

The library uses simple string parsing techniques without any regex or other heavy dependencies, making it very efficient for parsing magnet URLs. Benchmark results show that parsing a typical magnet URL takes around 500-600 nanoseconds, and generating a magnet URL string takes about 1.3-1.4 microseconds.

## License

This project is licensed under the MIT License - see the LICENSE.md file for details.
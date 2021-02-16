[![Build Status](https://www.travis-ci.com/billyb2/parse-magnet-rs.svg?branch=main)](https://www.travis-ci.com/billyb2/parse-magnet-rs)
# The Rust Magnet URL Parser!
# Intro
magnet_url-rs has the goal of, as you may have guessed, parsing the parts of magnets. It does
this using some relatively simple regexes. The crate is designed to be very simple and efficient,
with a lot of flexibility. It's also designed to be relatively easy to handle errors, and
modification of it's source is greatly encouraged through documentation and it's license.

## How to use this crate
Parsing a magnet is very simple:

 ```
 use magnet_url::Magnet;
 let magnet-url = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent");
 ```

This returns the Magnet struct, which is made up of the fields listed below this section. To
access one of these fields is also very simple:

 ```
 use magnet_url::Magnet;
 let magnet-url = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent");
 println!("{:?}", magnet-url.dn);
 ```

If you'd like to modify parts of the magnet_url to customize it, that can be done as well!

 ```
 use magnet_url::Magnet;
 let mut magnet-url = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent");
 println!("{:?}", magnet-url.dn);
 magnet-url.dn = String::from("hello_world");
 println!("{:?}", magnet-url.dn);
 ```

In fact, you can construct your own magnet url as well, as long as you fill in all the
parameters!

 ```
 use magnet_url::Magnet;
 let magnet-url =
 //Note, this magnet won't actually download, sorry :/
 Magnet {
     dn: "hello_world".to_string(),
     hash_type: "sha1".to_string(),
     xt: "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed".to_string(),
     xl: 1234567890,
     tr:
         {
             let mut tr_vec = Vec::new();
             tr_vec.push("https://example.com/".to_string())
             tr_vec
         },
     kt: "cool+stuff".to_string(),
     ws: String::new(),
     acceptable_source: String::new(),
     mt: String::new(),

 };
```

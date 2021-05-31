# What is a magnet url
A magnet is a URI scheme that identifies files by their hash,
normally used in peer to peer file sharing networks (like
Bittorrent). Basically, a magnet link identifies a torrent you
want to download, and tells the torrent client how to download
it. They make it very easy to share files over the internet,
and use a combination of DHT and trackers to tell your torrent
client where other peers who can share the file with you are.

# Why is magnet_url
While working on a side project, I realized that I had the
misfortune of trying to get the component parts of a magnet-url
and then do further processing of them. I quickly wrote some
Regex for it, but then I realized that this would be really
useful for other projects that are dealing with torrents in
Rust. By making it modifiable, too, it would allow for the
creation of custom magnet links, which would also be useful for
torrent based projects.

# Why use magnet_url
magnet_url has the goal of, as you may have guessed, parsing the parts of magnets. It does
this using some relatively simple regexes. The crate is designed to be very simple and efficient,
with a lot of flexibility. It's also designed to be relatively easy to handle errors, and
modification of its source is greatly encouraged through documentation and its license.

## How to use this crate
Parsing a magnet is very simple:

 ```rust
 use magnet_url::Magnet;
 let magneturl = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent");
 ```

This returns the Magnet struct, which is made up of the fields listed below this section. To
access one of these fields is also very simple:

 ```rust
 use magnet_url::Magnet;
 let magneturl = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent");
 println!("{:?}", magneturl.dn);
 ```

If you'd like to modify parts of the magnet_url to customize it, that can be done as well!

 ```rust
 use magnet_url::Magnet;
 let mut magneturl = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent");
 println!("{:?}", magneturl.dn);
 magneturl.dn = String::from("hello_world");
 println!("{:?}", magneturl.dn);
 ```

In fact, you can construct your own magnet url as well, as long as you fill in all the
parameters!

 ```rust
 use magnet_url::Magnet;
 //Note, this magnet won't actually download, sorry :/
 Magnet {
     dn: "hello_world".to_string(),
     hash_type: "sha1".to_string(),
     xt: "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed".to_string(),
     xl: 1234567890,
     tr:
         {
             let mut tr_vec = Vec::new();
             tr_vec.push("https://example.com/".to_string());
             tr_vec
         },
     kt: "cool+stuff".to_string(),
     ws: String::new(),
     acceptable_source: String::new(),
     mt: String::new(),
     xs: String::new(),
 };
 ```

From a Magnet struct, you can generate a magnet string again

 ```rust
 use magnet_url::Magnet;
 //Note, this magnet won't actually download, sorry :/
 let magnet_struct = Magnet {
     dn: "hello_world".to_string(),
     hash_type: "sha1".to_string(),
     xt: "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed".to_string(),
     xl: 1234567890,
     tr: vec!["https://example.com/".to_string()],
     kt: "cool+stuff".to_string(),
     ws: String::new(),
     acceptable_source: String::new(),
     mt: String::new(),
     xs: String::new(),
 };

 let magnet_string = magnet_struct.to_string();
 println!("{}", magnet_string);
 ```

Invalid magnet url's will result in a `panic!` (This will be changed to an error in v2.0.0
 ```rust
 use magnet_url::Magnet;
 #[should_panic]
 let _magnet_link = Magnet::new("https://example.com");
 ```

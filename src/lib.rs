use regex::Regex;

#[macro_use]
extern crate lazy_static;

///The regexes used to identify specific parts of the magnet
const MAGNET_SPEC_RE_STR: &str = r"magnet:\?";
const DISPLAY_NAME_RE_STR: &str = r"dn=([A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\{}\-]*)(&|$|\s)";
const EXACT_TOPIC_RE_STR: &str = r"xt=urn:(sha1|btih|ed2k|aich|kzhash|md5|tree:tiger):([A-Fa-f0-9]+|[A-Za-z2-7]+)";
const ADDRESS_TRACKER_RE_STR: &str = r"tr=([A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\{}\-]*)(&|$|\s)";
const KEYWORD_TOPIC_RE_STR: &str = r"kt=([A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\{}\-]*)(&|$|\s)";
const EXACT_SOURCE_RE_STR: &str = r"xs=((\w+)://[A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\\-]*)(&|$|\s)";
const EXACT_LENGTH_RE_STR: &str = r"xl=(\d*)(&|$|\s)";
const WEB_SEED_RE_STR: &str = r"ws=([A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\{}\-]*)(&|$|\s)";
const ACCEPTABLE_SOURCE_RE_STR: &str = r"as=((\w+)://[A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\\-]*)(&|$|\s))";
const MANIFEST_TOPIC_RE_STR: &str = r"mt=((\w+)://[A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\\-]*|urn:(sha1|btih|ed2k|aich|kzhash|md5|tree:tiger):([A-Fa-f0-9]+|[A-Za-z2-7]+))(&|$|\s))";


///# Intro
/// magnet-url-rs has the goal of, as you may have guessed, parsing the parts of magnets. It does
/// this using some relatively simple regexes. The crate is designed to be very simple and efficient,
/// with a lot of flexibility. It's also designed to be relatively easy to handle errors, and
/// modification of it's source is greatly encouraged through documentation and it's license.
///
/// ## How to use this crate
/// Parsing a magnet is very simple:
///
/// ```
/// use magnet-url-rs:Magnet;
/// let magnet_url = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent");
/// ```
///
/// This returns the Magnet struct, which is made up of the fields listed below this section. To
/// access one of these fields is also very simple:
///
/// ```
/// use magnet-url-rs:Magnet;
/// let magnet_url = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent");
/// println!("{:?}", magnet_url.dn);
/// ```
///
/// If you'd like to modify parts of the magnet_url to customize it, that can be done as well!
///
/// ```
/// use magnet-url-rs:Magnet;
/// let mut magnet_url = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent");
/// println!("{:?}", magnet_url.dn);
/// magnet_url.dn = String::from("hello_world");
/// println!("{:?}", magnet_url.dn);
/// ```
///
/// In fact, you can construct your own magnet url as well, as long as you fill in all the
/// parameters!
///
/// ```
/// use magnet-url-rs:Magnet;
/// let magnet_url =
/// //Note, this magnet won't actually download, sorry :/
/// Magnet {
///     dn: "hello_world".to_string(),
///     hash_type: "sha1".to_string(),
///     xt: "2aae6c35c94fcfb415dbe95f408b9ce91ee846ed".to_string(),
///     xl: 1234567890,
///     tr:
///         {
///             let mut tr_vec = Vec::new();
///             tr_vec.push("https://example.com/".to_string())
///             tr_vec
///         },
///     kt: "cool+stuff".to_string(),
///     ws: String::new(),
///     acceptable_source: String::new(),
///     mt: String::new(),
///
/// };
/// ```
pub struct Magnet {
    ///Display Name of the torrent
    pub dn: String,
    ///type of hash used in the exact topic
    pub hash_type: String,
    ///eXact Topic: URN containing the file hash. The URN is specific to the protocol so a file hash
    /// URN under btih (BitTorrent) would be completely different than the file hash URN for ed2k
    pub xt: String,
    ///eXact Length: The size (in bytes)
    ///The length is isize instead of usize since it makes error handling easier, as -1 is given if
    /// no length is set. I considered making it a String, but decided against it since it's simpler
    /// for the developer when they can just deal with an integer
    pub xl: isize,
    ///eXact Source: Either an HTTP (or HTTPS, FTP, FTPS, etc.) download source for the file pointed
    /// to by the Magnet link, the address of a P2P source for the file or the address of a hub (in
    /// the case of DC++), by which a client tries to connect directly, asking for the file and/or
    /// its sources. This field is commonly used by P2P clients to store the source, and may include
    /// the file hash.
    pub xs: String,
    ///address TRacker: Tracker URL; used to obtain resources for BitTorrent downloads without a
    /// need for DHT support. The value must be URL encoded
    pub tr: Vec<String>,
    ///Keyword Topic: Specifies a string of search keywords to search for in P2P networks, rather
    /// than a particular file. Also set as a vector since there will likely be more than one
    pub kt: String,
    ///Web Seed: The payload data served over HTTP(S)
    pub ws: String,
    ///Acceptable Source: Refers to a direct download from a web server. Regarded as only a
    /// fall-back source in case a client is unable to locate and/or download the linked-to file in its supported P2P network(s)
    ///as is a reserved keyword in Rust, so unfortunately this library must use the full name
    pub acceptable_source: String,
    ///Manifest Topic: Link to the metafile that contains a list of magneto (MAGMA –
    /// MAGnet MAnifest); i.e. a link to a list of links
    pub mt: String,
}

impl Magnet {
    /**Given a magnet URL, identify the specific parts, and return the Magnet struct. If the program
    can't identify a specific part of the magnet, then it will either give an empty version of what
    its value would normally be (such as an empty string, an empty vector, or in the case of xl, -1)
    */
    pub fn new (magnet_str: &str) -> Magnet {
        lazy_static! {
            static ref DISPLAY_NAME_RE: Regex = Regex::new(DISPLAY_NAME_RE_STR).unwrap();
            static ref EXACT_TOPIC_RE: Regex = Regex::new(EXACT_TOPIC_RE_STR).unwrap();
            static ref EXACT_LENGTH_RE: Regex = Regex::new(EXACT_LENGTH_RE_STR).unwrap();
            static ref ADDRESS_TRACKER_RE: Regex = Regex::new(ADDRESS_TRACKER_RE_STR).unwrap();
            static ref KEYWORD_TOPIC_RE: Regex = Regex::new(KEYWORD_TOPIC_RE_STR).unwrap();
            static ref EXACT_SOURCE_RE: Regex = Regex::new(EXACT_SOURCE_RE_STR).unwrap();
            static ref WEB_SEED_RE: Regex = Regex::new(WEB_SEED_RE_STR).unwrap();
            static ref ACCEPTABLE_SOURCE_RE: Regex = Regex::new(ACCEPTABLE_SOURCE_RE_STR).unwrap();
            static ref MANIFEST_TOPIC_RE: Regex = Regex::new(MANIFEST_TOPIC_RE_STR).unwrap();
            static ref MAGNET_RE: Regex = Regex::new(MAGNET_RE_STR).unwrap();
        }

        let validate_regex = |regex: &Regex, re_group_index| -> String {
            match regex.captures(magnet_str) {
                Some(m) => m.get(re_group_index).map_or("", |m| m.as_str()).to_string(),
                None => String::new()
            }

        };

        Magnet {
            dn: validate_regex(&DISPLAY_NAME_RE, 1),
            hash_type: validate_regex(&EXACT_TOPIC_RE, 1),
            xt: validate_regex(&EXACT_TOPIC_RE, 2),
            xl: validate_regex(&EXACT_LENGTH_RE, 1).parse().unwrap_or(-1),
            xs: validate_regex(&EXACT_SOURCE_RE, 1),
            tr: {
                let mut tr_vec: Vec<String> = Vec::new();
                // Since tr is a vector, I can't just use the validate_regex function
                if ADDRESS_TRACKER_RE.is_match(magnet_str) {
                    for tr in ADDRESS_TRACKER_RE.captures_iter(magnet_str) {
                        tr_vec.push(tr.get(1).map_or("", |m| m.as_str()).to_string());
                    }
                }
                tr_vec

            },
            kt: validate_regex(&KEYWORD_TOPIC_RE, 1),
            ws: validate_regex(&WEB_SEED_RE, 1),
            acceptable_source: validate_regex(&ACCEPTABLE_SOURCE_RE, 1),
            mt: validate_regex(&MANIFEST_TOPIC_RE, 1),

        }
    }
}

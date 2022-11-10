use std::error::Error;
use regex::Regex;
use std::fmt;
use std::fmt::{Display, Formatter};

#[macro_use]
extern crate lazy_static;

///The regexes used to identify specific parts of the magnet
const DISPLAY_NAME_RE_STR: &str = r"dn=([A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\{}\-]*)(&|$|\s)";
const EXACT_TOPIC_RE_STR: &str = r"xt=urn:(sha1|btih|ed2k|aich|kzhash|md5|tree:tiger):([A-Fa-f0-9]+|[A-Za-z2-7]+)";
const ADDRESS_TRACKER_RE_STR: &str = r"tr=([A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\{}\-]*)(&|$|\s)";
const KEYWORD_TOPIC_RE_STR: &str = r"kt=([A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\{}\-]*)(&|$|\s)";
const EXACT_SOURCE_RE_STR: &str = r"xs=((\w+)[A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\\-]*)(&|$|\s)";
const EXACT_LENGTH_RE_STR: &str = r"xl=(\d*)(&|$|\s)";
const WEB_SEED_RE_STR: &str = r"ws=([A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\{}\-]*)(&|$|\s)";
const ACCEPTABLE_SOURCE_RE_STR: &str = r"as=((\w+)[A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\\-]*)(&|$|\s)";
const MANIFEST_TOPIC_RE_STR: &str = r"mt=((\w+)[A-Za-z0-9!@#$%^:*<>,?/()_+=.{}\\-]*|urn:(sha1|btih|ed2k|aich|kzhash|md5|tree:tiger):([A-Fa-f0-9]+|[A-Za-z2-7]+))(&|$|\s)";

///# What is a magnet url
///A magnet is a URI scheme that identifies files by their hash,
/// normally used in peer to peer file sharing networks (like
/// Bittorrent). Basically, a magnet link identifies a torrent you
/// want to download, and tells the torrent client how to download
/// it. They make it very easy to share files over the internet,
/// and use a combination of DHT and trackers to tell your torrent
/// client where other peers who can share the file with you are.
///
///# Why is magnet_url
///While working on a side project, I realized that I had the
/// misfortune of trying to get the component parts of a magnet-url
/// and then do further processing of them. I quickly wrote some
/// Regex for it, but then I realized that this would be really
/// useful for other projects that are dealing with torrents in
/// Rust. By making it modifiable, too, it would allow for the
/// creation of custom magnet links, which would also be useful for
/// torrent based projects.
///
///# Why use magnet_url
/// magnet_url has the goal of, as you may have guessed, parsing the parts of magnets. It does
/// this using some relatively simple regexes. The crate is designed to be very simple and efficient,
/// with a lot of flexibility. It's also designed to be relatively easy to handle errors, and
/// modification of its source is greatly encouraged through documentation and its license.
///
/// ## How to use this crate
/// Parsing a magnet is very simple:
///
/// ```
/// use magnet_url::Magnet;
/// let magneturl = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent");
/// ```
///
/// This returns the Magnet struct, which is made up of the fields listed below this section, wrapped aroud a Result<Magnet, MagnetError>. To
/// access one of these fields is also very simple:
///
/// ```
/// use magnet_url::Magnet;
/// let magneturl = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent").unwrap();
/// println!("{:?}", magneturl.display_name);
/// ```
///
/// If you'd like to modify parts of the magnet_url to customize it, that can be done as well!
///
/// ```
/// use magnet_url::Magnet;
/// let mut magneturl = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent").unwrap();
/// println!("{:?}", magneturl.display_name);
/// magneturl.display_name = Some(String::from("hello_world"));
/// println!("{:?}", magneturl.display_name);
/// ```
///
/// In fact, you can construct your own magnet url as well, as long as you fill in all the
/// parameters!
///
/// ```
/// use magnet_url::Magnet;
/// //Note, this magnet won't actually download, sorry :/
/// Magnet {
///     display_name: Some("hello_world".to_string()),
///     hash_type: Some("sha1".to_string()),
///     hash: Some("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed".to_string()),
///     length: Some(1234567890),
///     trackers: vec!["https://example.com/".to_string()],
///     search_keywords: Some("cool+stuff".to_string()),
///     web_seed: None,
///     acceptable_source: None,
///     manifest: None,
///     source: None,
/// };
/// ```
///
/// From a Magnet struct, you can generate a magnet string again
///
/// ```
/// use magnet_url::Magnet;
/// //Note, this magnet won't actually download, sorry :/
/// let magnet_struct = Magnet {
///     display_name: Some("hello_world".to_string()),
///     hash_type: Some("sha1".to_string()),
///     hash: Some("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed".to_string()),
///     length: Some(1234567890),
///     trackers: vec!["https://example.com/".to_string()],
///     search_keywords: Some("cool+stuff".to_string()),
///     web_seed: None,
///     acceptable_source: None,
///     manifest: None,
///     source: None,
/// };
///
/// let magnet_string = magnet_struct.to_string();
/// println!("{}", magnet_string);
/// ```
///
/// Invalid magnet url's will result in an Error, which can be handled appropriately
/// ```#[should_panic]
/// use magnet_url::Magnet;
/// let _magnet_link = Magnet::new("https://example.com").unwrap();
/// ```

/// The various ways the new function can fail
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum MagnetError {
    NotAMagnetURL,
}

impl Display for MagnetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "provided link is not a valid magnet URL")
    }
}

impl Error for MagnetError {}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Magnet {
    /// Display Name of the torrent
    pub display_name: Option<String>,
    /// Type of hash used in the exact topic
    pub hash_type: Option<String>,
    /// (xt / exact topic) Torrent hash.
    pub hash: Option<String>,
    /// (xl): The size (in bytes) of the torrent.
    pub length: Option<u64>,
    /// (ex): Either an HTTP (or HTTPS, FTP, FTPS, etc.) download source for the file pointed
    /// to by the Magnet link, the address of a P2P source for the file or the address of a hub (in
    /// the case of DC++), by which a client tries to connect directly, asking for the file and/or
    /// its sources. This field is commonly used by P2P clients to store the source, and may include
    /// the file hash.
    pub source: Option<String>,
    /// Tracker URLs (tr) used to obtain resources for BitTorrent downloads without a
    /// need for DHT support. The value must be URL encoded
    pub trackers: Vec<String>,
    /// (kt) A string of search keywords to search for in P2P networks, rather
    /// than a particular file. Also set as a vector since there will likely be more than one
    pub search_keywords: Option<String>,
    /// (ws) The payload data served over HTTP(S)
    pub web_seed: Option<String>,
    /// (as) Refers to a direct download from a web server. Regarded as only a
    /// fall-back source in case a client is unable to locate and/or download the linked-to file in its supported P2P network(s)
    /// as is a reserved keyword in Rust, so unfortunately this library must use the full name
    pub acceptable_source: Option<String>,
    /// (mt) Link to the metafile that contains a list of magneto (MAGMA – MAGnet MAnifest); i.e. a link to a list of links
    pub manifest: Option<String>,
}

impl Magnet {

    /**Given a magnet URL, identify the specific parts, and return the Magnet struct. If the program
    can't identify a specific part of the magnet, then it will either give an empty version of what
    its value would normally be (such as an empty string, an empty vector, or in the case of xl, -1).
    It also doesn't validate whether the magnet url is good, which makes it faster, but dangerous!
    Only use this function if you know for certain that the magnet url given is valid.
    */
    pub fn new_no_validation (magnet_str: &str) -> Magnet {
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
        }

        let validate_regex = |regex: &Regex, re_group_index| -> Option<String> {
            match regex.captures(magnet_str) {
                Some(m) => m.get(re_group_index).map_or(None, |m| Some(m.as_str().to_string())),
                None => None
            }

        };

        Magnet {
            display_name: validate_regex(&DISPLAY_NAME_RE, 1),
            hash_type: validate_regex(&EXACT_TOPIC_RE, 1),
            hash: validate_regex(&EXACT_TOPIC_RE, 2),
            // Using a slightly modified match statement so it doesn't parse from str to String to int
            length: {
                match &EXACT_LENGTH_RE.captures(magnet_str) {
                    Some(m) => m.get(1).map_or(None, |m| Some(m.as_str().parse().unwrap())),
                    None => None,
                }

            },
            source: validate_regex(&EXACT_SOURCE_RE, 1),
            trackers: {
                let mut tr_vec: Vec<String> = Vec::new();
                // Since tr is a vector, I can't just use the validate_regex function
                for tr in ADDRESS_TRACKER_RE.captures_iter(magnet_str) {
                    tr_vec.push(tr.get(1).map_or(String::new(), |m| m.as_str().to_string()));
                }

                tr_vec

            },
            search_keywords: validate_regex(&KEYWORD_TOPIC_RE, 1),
            web_seed: validate_regex(&WEB_SEED_RE, 1),
            acceptable_source: validate_regex(&ACCEPTABLE_SOURCE_RE, 1),
            manifest: validate_regex(&MANIFEST_TOPIC_RE, 1),

        }
    }

    /// The recommended way of creating magnets. The same as new_no_validation, but does validation
    #[inline]
    pub fn new(magnet_str: &str) -> Result<Magnet, MagnetError> {
        if !magnet_str.starts_with("magnet:?") {
            Err(MagnetError::NotAMagnetURL)

        } else {
            Ok(Magnet::new_no_validation(magnet_str))

        }

    }
}

impl fmt::Display for Magnet {
    /*
    This generates a magnet url string given a Magnet struct
    */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut magnet_string = String::from("magnet:?");

        if let Some(xt) = &self.hash {
            magnet_string = format!("{}{}{}:{}", magnet_string, "xt=urn:", self.hash_type.as_ref().unwrap_or(&String::new()), xt);
        }

        let add_to_mag_string = |p_name: String, p_val: &Option<String>| -> String {
            if let Some(p_val) = p_val {
                format!("&{}={}", p_name, p_val)

            } else {
                String::new()

            }
        };

        magnet_string = format!("{}{}", magnet_string, add_to_mag_string(String::from("dn"), &self.display_name));

        if let Some(xl) = &self.length {
            magnet_string = format!("{}&xl={}", magnet_string, xl);
        }

        magnet_string = {
            let mut tr_string = String::new();
            for tracker in &self.trackers {
                tr_string = format!("{}&tr={}", tr_string, tracker);
            }

            format!("{}{}", magnet_string, tr_string)
        };

        magnet_string = format!("{}{}", magnet_string, add_to_mag_string(String::from("ws"), &self.web_seed));
        magnet_string = format!("{}{}", magnet_string, add_to_mag_string(String::from("xs"), &self.source));
        magnet_string = format!("{}{}", magnet_string, add_to_mag_string(String::from("kt"), &self.search_keywords));
        magnet_string = format!("{}{}", magnet_string, add_to_mag_string(String::from("as"), &self.acceptable_source));
        magnet_string = format!("{}{}", magnet_string, add_to_mag_string(String::from("mt"), &self.manifest));


        write!(f, "{}", magnet_string)

    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::{Magnet, MagnetError};

    #[test]
    fn sintel_test() {
        const MAGNET_STR: &str = "magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent";
        let magnet_link = Magnet::new(MAGNET_STR).unwrap();

        assert_eq!(magnet_link.display_name, Some("Sintel".to_string()));
        assert_eq!(magnet_link.hash_type, Some("btih".to_string()));
        assert_eq!(magnet_link.hash, Some("08ada5a7a6183aae1e09d831df6748d566095a10".to_string()));
        assert_eq!(magnet_link.length, None);
        assert_eq!(magnet_link.source, Some("https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent".to_string()));
        assert_eq!(magnet_link.trackers[0], "udp%3A%2F%2Fexplodie.org%3A6969".to_string());
        assert_eq!(magnet_link.trackers[1], "udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969".to_string());
        assert_eq!(magnet_link.trackers[2], "udp%3A%2F%2Ftracker.empire-js.us%3A1337".to_string());
        assert_eq!(magnet_link.trackers[3], "udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969".to_string());
        assert_eq!(magnet_link.trackers[4], "udp%3A%2F%2Ftracker.opentrackr.org%3A1337".to_string());
        assert_eq!(magnet_link.trackers[5], "wss%3A%2F%2Ftracker.btorrent.xyz".to_string());
        assert_eq!(magnet_link.trackers[6], "wss%3A%2F%2Ftracker.fastcast.nz".to_string());
        assert_eq!(magnet_link.trackers[7], "wss%3A%2F%2Ftracker.openwebtorrent.com".to_string());
        assert_eq!(magnet_link.web_seed, Some("https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F".to_string()));
        assert_eq!(magnet_link.source, Some("https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent".to_string()));
        assert_eq!(magnet_link.search_keywords, None);
        assert_eq!(magnet_link.acceptable_source, None);
        assert_eq!(magnet_link.manifest, None);

        //Need to recreate a magnet struct from the string, since the elements could be in any order
        assert_eq!(Magnet::new(&magnet_link.to_string()).unwrap(), magnet_link);
        //Also tests PartialEq
        assert_eq!(Magnet::new(&magnet_link.to_string()).unwrap() == magnet_link, true);
    }

    #[test]
    fn invalid_magnet_test() {
        let result = Magnet::new("https://example.com");
        assert_eq!(result, Err(MagnetError::NotAMagnetURL));
        let err = result.unwrap_err();
        assert_eq!(err.to_string(), "provided link is not a valid magnet URL");
        assert!(err.source().is_none());
    }

    #[test]
    fn not_equal_magnet_test() {
        //These two torrents aren't even close to equal
        const MAGNET_STR_1: &str = "magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent";
        const MAGNET_STR_2: &str = "magnet:?xt=urn:btih:da826adb2ba4933500d83c19bbdfa73ee28f34d5&dn=devuan%5Fbeowulf&tr=udp%3A%2F%2F9.rarbg.me%3A2710%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.cyberia.is%3A6969%2Fannounce";

        let magnet_link_1 = Magnet::new(MAGNET_STR_1);
        let magnet_link_2 = Magnet::new(MAGNET_STR_2);

        //These two torrents, on the other hand, are very similar
        const MAGNET_STR_3: &str = "magnet:?xt=urn:btih:da826adb2ba4933500d83c19bbdfa73ee28f34d5&dn=devuan%5Fbeowulf&tr=udp%3A%2F%2F9.rarbg.me%3A2710%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.cyberia.is%3A6969%2Fannounce";
        const MAGNET_STR_4: &str = "magnet:?xt=urn:btih:da826adb2ba4933500d83c19bbdfa73ee28f34d5&dn=devuan%5Fbeowulf&tr=udp%3A%2F%2F9.rarbg.me%3A2710%2Fannounce&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.cyberia.is%3A6969%2Fannounce&tr=https://example.com/fake_tracker";

        let magnet_link_3 = Magnet::new(MAGNET_STR_3);
        let magnet_link_4 = Magnet::new(MAGNET_STR_4);

        assert_ne!(magnet_link_1, magnet_link_2);
        assert_ne!(magnet_link_3, magnet_link_4);

        //magnet_link_2 and magnet_link_3 are exactly the same
        assert_eq!(magnet_link_2, magnet_link_3);
        //Tests PartialEq instead of Debug
        assert_eq!(magnet_link_2 == magnet_link_3, true);

    }
}

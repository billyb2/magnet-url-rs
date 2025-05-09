//! # What is a magnet url
//! A magnet is a URI scheme that identifies files by their hash,
//! normally used in peer to peer file sharing networks (like
//! Bittorrent). Basically, a magnet link identifies a torrent you
//! want to download, and tells the torrent client how to download
//! it. They make it very easy to share files over the internet,
//! and use a combination of DHT and trackers to tell your torrent
//! client where other peers who can share the file with you are.
//!
//! # Why use magnet_url
//! magnet_url has the goal of, as you may have guessed, parsing the parts of magnets. It does
//! this using simple string parsing techniques for maximum efficiency. The crate is designed 
//! to be very simple and efficient, with a lot of flexibility. It's also designed to be 
//! relatively easy to handle errors.
//!
//! ## How to use this crate
//! Parsing a magnet is very simple:
//!
//! ```
//! use magnet_url::Magnet;
//! let magneturl = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent").unwrap();
//! ```
//!
//! This returns the Magnet struct, which lets you access all parts of the magnet URL through getter methods:
//!
//! ```
//! use magnet_url::Magnet;
//! let magneturl = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent").unwrap();
//! println!("{:?}", magneturl.display_name());
//! ```
//!
//! You can construct your own magnet URLs using the builder pattern:
//!
//! ```
//! use magnet_url::MagnetBuilder;
//! 
//! // Note, this magnet won't actually download, sorry :/
//! let magnet = MagnetBuilder::new()
//!     .display_name("hello_world")
//!     .hash_type("sha1")
//!     .hash("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed")
//!     .length(1234567890)
//!     .add_tracker("https://example.com/")
//!     .search_keywords("cool+stuff")
//!     .build();
//!
//! let magnet_string = magnet.to_string();
//! println!("{}", magnet_string);
//! ```
//!
//! Invalid magnet URLs will result in an Error, which can be handled appropriately:
//! ```
//! use magnet_url::{Magnet, MagnetError};
//!
//! // This will return an Err(MagnetError::NotAMagnetURL)
//! let result = Magnet::new("https://example.com");
//!
//! match result {
//!     Ok(magnet) => {
//!         // Process the valid magnet
//!         println!("Display name: {:?}", magnet.display_name());
//!     },
//!     Err(MagnetError::NotAMagnetURL) => {
//!         // Handle invalid magnet URL
//!         println!("The provided string is not a valid magnet URL");
//!     }
//! }
//! ```

use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

/// The various ways the Magnet parsing can fail
#[derive(Debug, Clone, Hash, PartialEq)]
pub enum MagnetError {
    /// The provided string is not a valid magnet URL
    NotAMagnetURL,
}

impl Display for MagnetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            MagnetError::NotAMagnetURL => write!(f, "provided link is not a valid magnet URL"),
        }
    }
}

impl Error for MagnetError {}

/// Represents a parsed magnet URL with all its components
#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Magnet {
    /// Display Name of the torrent
    display_name: Option<String>,
    /// Type of hash used in the exact topic
    hash_type: Option<String>,
    /// (xt / exact topic) Torrent hash
    hash: Option<String>,
    /// (xl): The size (in bytes) of the torrent
    length: Option<u64>,
    /// (xs): Download source for the file or the address of a P2P source
    source: Option<String>,
    /// Tracker URLs (tr) used to obtain resources for BitTorrent downloads
    trackers: Vec<String>,
    /// (kt) Search keywords to search for in P2P networks
    search_keywords: Option<String>,
    /// (ws) The payload data served over HTTP(S)
    web_seed: Option<String>,
    /// (as) Direct download from a web server as a fall-back source
    acceptable_source: Option<String>,
    /// (mt) Link to the metafile that contains a list of magneto
    manifest: Option<String>,
}

impl Magnet {
    /// Parse a magnet URL string into a Magnet struct
    ///
    /// # Arguments
    ///
    /// * `magnet_str` - A string slice containing the magnet URL
    ///
    /// # Returns
    ///
    /// * `Result<Magnet, MagnetError>` - A Result containing the parsed Magnet or an error
    ///
    /// # Errors
    ///
    /// Returns `Err(MagnetError::NotAMagnetURL)` if the string does not start with "magnet:?"
    ///
    /// # Example
    ///
    /// ```
    /// use magnet_url::Magnet;
    /// let magnet = Magnet::new("magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel").unwrap();
    /// ```
    pub fn new(magnet_str: &str) -> Result<Magnet, MagnetError> {
        if !magnet_str.starts_with("magnet:?") {
            return Err(MagnetError::NotAMagnetURL);
        }
        
        Ok(Self::new_no_validation(magnet_str))
    }

    /// Parse a magnet URL string without validating the prefix
    ///
    /// This function is used internally by `new` and should only be used directly
    /// if you know the string is a valid magnet URL.
    fn new_no_validation(magnet_str: &str) -> Magnet {
        let mut magnet = Magnet {
            display_name: None,
            hash_type: None,
            hash: None,
            length: None,
            source: None,
            trackers: Vec::new(),
            search_keywords: None,
            web_seed: None,
            acceptable_source: None,
            manifest: None,
        };

        // Skip the magnet:? prefix
        let params_str = magnet_str.trim_start_matches("magnet:?");
        
        // Split parameters by &
        for param in params_str.split('&') {
            if let Some((key, value)) = param.split_once('=') {
                match key {
                    "dn" => magnet.display_name = Some(value.to_string()),
                    "xt" => {
                        // Handle xt=urn:hash_type:hash format
                        if let Some(urn_part) = value.strip_prefix("urn:") {
                            if let Some((hash_type, hash)) = urn_part.split_once(':') {
                                magnet.hash_type = Some(hash_type.to_string());
                                magnet.hash = Some(hash.to_string());
                            }
                        }
                    },
                    "xl" => {
                        if let Ok(len) = value.parse::<u64>() {
                            magnet.length = Some(len);
                        }
                    },
                    "tr" => magnet.trackers.push(value.to_string()),
                    "kt" => magnet.search_keywords = Some(value.to_string()),
                    "ws" => magnet.web_seed = Some(value.to_string()),
                    "xs" => magnet.source = Some(value.to_string()),
                    "as" => magnet.acceptable_source = Some(value.to_string()),
                    "mt" => magnet.manifest = Some(value.to_string()),
                    _ => {} // Ignore unknown parameters
                }
            }
        }
        
        magnet
    }

    /// Get the display name of the torrent
    pub fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    /// Get the hash type used in the exact topic
    pub fn hash_type(&self) -> Option<&str> {
        self.hash_type.as_deref()
    }

    /// Get the torrent hash
    pub fn hash(&self) -> Option<&str> {
        self.hash.as_deref()
    }

    /// Get the size (in bytes) of the torrent
    pub fn length(&self) -> Option<u64> {
        self.length
    }

    /// Get the download source for the file
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }

    /// Get the tracker URLs
    pub fn trackers(&self) -> &[String] {
        &self.trackers
    }

    /// Get the search keywords
    pub fn search_keywords(&self) -> Option<&str> {
        self.search_keywords.as_deref()
    }

    /// Get the web seed URL
    pub fn web_seed(&self) -> Option<&str> {
        self.web_seed.as_deref()
    }

    /// Get the acceptable source
    pub fn acceptable_source(&self) -> Option<&str> {
        self.acceptable_source.as_deref()
    }

    /// Get the manifest link
    pub fn manifest(&self) -> Option<&str> {
        self.manifest.as_deref()
    }
}

impl fmt::Display for Magnet {
    /// Generates a magnet URL string from the Magnet struct
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut magnet_string = String::from("magnet:?");

        // Add the hash (required for a valid magnet)
        if let Some(hash) = &self.hash {
            magnet_string = format!(
                "{}{}{}:{}",
                magnet_string,
                "xt=urn:",
                self.hash_type.as_ref().unwrap_or(&String::new()),
                hash
            );
        }

        // Helper function to add parameters
        let add_param = |name: &str, value: &Option<String>, base: &str| -> String {
            if let Some(val) = value {
                format!("{}&{}={}", base, name, val)
            } else {
                base.to_string()
            }
        };

        // Add optional parameters
        magnet_string = add_param("dn", &self.display_name, &magnet_string);

        if let Some(len) = &self.length {
            magnet_string = format!("{}&xl={}", magnet_string, len);
        }

        // Add tracker URLs
        for tracker in &self.trackers {
            magnet_string = format!("{}&tr={}", magnet_string, tracker);
        }

        // Add remaining optional parameters
        magnet_string = add_param("ws", &self.web_seed, &magnet_string);
        magnet_string = add_param("xs", &self.source, &magnet_string);
        magnet_string = add_param("kt", &self.search_keywords, &magnet_string);
        magnet_string = add_param("as", &self.acceptable_source, &magnet_string);
        magnet_string = add_param("mt", &self.manifest, &magnet_string);

        write!(f, "{}", magnet_string)
    }
}

/// Builder for creating Magnet URLs
///
/// # Example
///
/// ```
/// use magnet_url::MagnetBuilder;
///
/// let magnet = MagnetBuilder::new()
///     .display_name("My Torrent")
///     .hash_type("btih")
///     .hash("1234567890abcdef1234567890abcdef12345678")
///     .add_tracker("udp://tracker.example.com:6969")
///     .build();
///
/// println!("{}", magnet.to_string());
/// ```
pub struct MagnetBuilder {
    magnet: Magnet,
}

impl MagnetBuilder {
    /// Create a new MagnetBuilder
    pub fn new() -> Self {
        Self {
            magnet: Magnet {
                display_name: None,
                hash_type: None,
                hash: None,
                length: None,
                source: None,
                trackers: Vec::new(),
                search_keywords: None,
                web_seed: None,
                acceptable_source: None,
                manifest: None,
            }
        }
    }

    /// Set the display name of the torrent
    pub fn display_name(mut self, name: &str) -> Self {
        self.magnet.display_name = Some(name.to_string());
        self
    }

    /// Set the hash type used in the exact topic
    pub fn hash_type(mut self, hash_type: &str) -> Self {
        self.magnet.hash_type = Some(hash_type.to_string());
        self
    }

    /// Set the torrent hash
    pub fn hash(mut self, hash: &str) -> Self {
        self.magnet.hash = Some(hash.to_string());
        self
    }

    /// Set the size (in bytes) of the torrent
    pub fn length(mut self, length: u64) -> Self {
        self.magnet.length = Some(length);
        self
    }

    /// Set the download source for the file
    pub fn source(mut self, source: &str) -> Self {
        self.magnet.source = Some(source.to_string());
        self
    }

    /// Add a tracker URL
    pub fn add_tracker(mut self, tracker: &str) -> Self {
        self.magnet.trackers.push(tracker.to_string());
        self
    }

    /// Add multiple tracker URLs
    pub fn add_trackers(mut self, trackers: &[&str]) -> Self {
        self.magnet.trackers.extend(trackers.iter().map(|t| t.to_string()));
        self
    }

    /// Set the search keywords
    pub fn search_keywords(mut self, keywords: &str) -> Self {
        self.magnet.search_keywords = Some(keywords.to_string());
        self
    }

    /// Set the web seed URL
    pub fn web_seed(mut self, web_seed: &str) -> Self {
        self.magnet.web_seed = Some(web_seed.to_string());
        self
    }

    /// Set the acceptable source
    pub fn acceptable_source(mut self, source: &str) -> Self {
        self.magnet.acceptable_source = Some(source.to_string());
        self
    }

    /// Set the manifest link
    pub fn manifest(mut self, manifest: &str) -> Self {
        self.magnet.manifest = Some(manifest.to_string());
        self
    }

    /// Build the Magnet struct
    pub fn build(self) -> Magnet {
        self.magnet
    }
}

impl Default for MagnetBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Magnet, MagnetBuilder, MagnetError};
    use std::error::Error;

    #[test]
    fn sintel_test() {
        const MAGNET_STR: &str = "magnet:?xt=urn:btih:08ada5a7a6183aae1e09d831df6748d566095a10&dn=Sintel&tr=udp%3A%2F%2Fexplodie.org%3A6969&tr=udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969&tr=udp%3A%2F%2Ftracker.empire-js.us%3A1337&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337&tr=wss%3A%2F%2Ftracker.btorrent.xyz&tr=wss%3A%2F%2Ftracker.fastcast.nz&tr=wss%3A%2F%2Ftracker.openwebtorrent.com&ws=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F&xs=https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent";
        let magnet_link = Magnet::new(MAGNET_STR).unwrap();

        assert_eq!(magnet_link.display_name(), Some("Sintel"));
        assert_eq!(magnet_link.hash_type(), Some("btih"));
        assert_eq!(
            magnet_link.hash(),
            Some("08ada5a7a6183aae1e09d831df6748d566095a10")
        );
        assert_eq!(magnet_link.length(), None);
        assert_eq!(
            magnet_link.source(),
            Some("https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent")
        );
        assert_eq!(
            magnet_link.trackers()[0],
            "udp%3A%2F%2Fexplodie.org%3A6969"
        );
        assert_eq!(
            magnet_link.trackers()[1],
            "udp%3A%2F%2Ftracker.coppersurfer.tk%3A6969"
        );
        assert_eq!(
            magnet_link.trackers()[2],
            "udp%3A%2F%2Ftracker.empire-js.us%3A1337"
        );
        assert_eq!(
            magnet_link.trackers()[3],
            "udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969"
        );
        assert_eq!(
            magnet_link.trackers()[4],
            "udp%3A%2F%2Ftracker.opentrackr.org%3A1337"
        );
        assert_eq!(
            magnet_link.trackers()[5],
            "wss%3A%2F%2Ftracker.btorrent.xyz"
        );
        assert_eq!(
            magnet_link.trackers()[6],
            "wss%3A%2F%2Ftracker.fastcast.nz"
        );
        assert_eq!(
            magnet_link.trackers()[7],
            "wss%3A%2F%2Ftracker.openwebtorrent.com"
        );
        assert_eq!(
            magnet_link.web_seed(),
            Some("https%3A%2F%2Fwebtorrent.io%2Ftorrents%2F")
        );
        assert_eq!(
            magnet_link.source(),
            Some("https%3A%2F%2Fwebtorrent.io%2Ftorrents%2Fsintel.torrent")
        );
        assert_eq!(magnet_link.search_keywords(), None);
        assert_eq!(magnet_link.acceptable_source(), None);
        assert_eq!(magnet_link.manifest(), None);

        //Need to recreate a magnet struct from the string, since the elements could be in any order
        assert_eq!(Magnet::new(&magnet_link.to_string()).unwrap(), magnet_link);
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
    }

    #[test]
    fn builder_test() {
        // Test creating a Magnet using the builder
        let magnet = MagnetBuilder::new()
            .display_name("Test")
            .hash_type("btih")
            .hash("1234567890abcdef1234567890abcdef12345678")
            .length(12345)
            .add_tracker("udp://tracker1.example.com:6969")
            .add_tracker("udp://tracker2.example.com:6969")
            .search_keywords("test+keywords")
            .web_seed("https://example.com/seed")
            .acceptable_source("https://example.com/download")
            .manifest("https://example.com/manifest")
            .source("https://example.com/source")
            .build();

        // Convert to string and back to ensure all fields are properly serialized
        let magnet_str = magnet.to_string();
        let parsed_magnet = Magnet::new(&magnet_str).unwrap();

        // Verify all fields match
        assert_eq!(parsed_magnet.display_name(), Some("Test"));
        assert_eq!(parsed_magnet.hash_type(), Some("btih"));
        assert_eq!(parsed_magnet.hash(), Some("1234567890abcdef1234567890abcdef12345678"));
        assert_eq!(parsed_magnet.length(), Some(12345));
        assert_eq!(parsed_magnet.trackers().len(), 2);
        assert_eq!(parsed_magnet.trackers()[0], "udp://tracker1.example.com:6969");
        assert_eq!(parsed_magnet.trackers()[1], "udp://tracker2.example.com:6969");
        assert_eq!(parsed_magnet.search_keywords(), Some("test+keywords"));
        assert_eq!(parsed_magnet.web_seed(), Some("https://example.com/seed"));
        assert_eq!(parsed_magnet.acceptable_source(), Some("https://example.com/download"));
        assert_eq!(parsed_magnet.manifest(), Some("https://example.com/manifest"));
        assert_eq!(parsed_magnet.source(), Some("https://example.com/source"));

        // Ensure the magnet URL starts with the correct prefix
        assert!(magnet_str.starts_with("magnet:?xt=urn:"));
        
        // Ensure all fields are present in the string
        assert!(magnet_str.contains("&dn=Test"));
        assert!(magnet_str.contains("&tr=udp://tracker1.example.com:6969"));
        assert!(magnet_str.contains("&tr=udp://tracker2.example.com:6969"));
        assert!(magnet_str.contains("&xl=12345"));
        assert!(magnet_str.contains("&kt=test+keywords"));
        assert!(magnet_str.contains("&ws=https://example.com/seed"));
        assert!(magnet_str.contains("&as=https://example.com/download"));
        assert!(magnet_str.contains("&mt=https://example.com/manifest"));
        assert!(magnet_str.contains("&xs=https://example.com/source"));
    }
    
    #[test]
    fn add_trackers_test() {
        // Test the add_trackers method
        let trackers = ["udp://tracker1.example.com:6969", "udp://tracker2.example.com:6969"];
        
        let magnet = MagnetBuilder::new()
            .hash_type("btih")
            .hash("1234567890abcdef1234567890abcdef12345678")
            .add_trackers(&trackers)
            .build();
            
        assert_eq!(magnet.trackers().len(), 2);
        assert_eq!(magnet.trackers()[0], "udp://tracker1.example.com:6969");
        assert_eq!(magnet.trackers()[1], "udp://tracker2.example.com:6969");
    }
    
    #[test]
    fn empty_optional_fields_test() {
        // Test with minimal fields to ensure optional fields are handled correctly
        let magnet = MagnetBuilder::new()
            .hash_type("btih")
            .hash("1234567890abcdef1234567890abcdef12345678")
            .build();

        // Convert to string and back
        let magnet_str = magnet.to_string();
        let parsed_magnet = Magnet::new(&magnet_str).unwrap();

        // Verify all fields match
        assert_eq!(parsed_magnet.display_name(), None);
        assert_eq!(parsed_magnet.hash_type(), Some("btih"));
        assert_eq!(parsed_magnet.hash(), Some("1234567890abcdef1234567890abcdef12345678"));
        assert_eq!(parsed_magnet.length(), None);
        assert!(parsed_magnet.trackers().is_empty());
        assert_eq!(parsed_magnet.search_keywords(), None);
        assert_eq!(parsed_magnet.web_seed(), None);
        assert_eq!(parsed_magnet.acceptable_source(), None);
        assert_eq!(parsed_magnet.manifest(), None);
        assert_eq!(parsed_magnet.source(), None);
    }
}
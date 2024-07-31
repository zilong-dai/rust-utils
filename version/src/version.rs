use build_time::build_time_utc;
use const_format::{concatcp, str_split};

// // generates a `const BUILD_TIME: &str`
// build_timestamp!("%A %Y-%m-%d / %H:%M:%S");

/// The human readable name of the client
// pub const NAME_CLIENT: &str = env!("CARGO_PKG_NAME");

/// The latest version from Cargo.toml.
pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The build timestamp.
pub const BUILD_TIMESTAMP: &str = build_time_utc!();

// pub const SHORT_VERSION: &str = concatcp!(CARGO_PKG_VERSION);

pub const LONG_VERSION: &str = concatcp!(
    "Version: ",
    CARGO_PKG_VERSION,
    "\n",
    "Build Timestamp: ",
    BUILD_TIMESTAMP,
    "\n",
    "Build Profile: ",
    BUILD_PROFILE_NAME
);

pub(crate) const BUILD_PROFILE_NAME: &str = {
    // Derived from https://stackoverflow.com/questions/73595435/how-to-get-profile-from-cargo-toml-in-build-rs-or-at-runtime
    // We split on the path separator of the *host* machine, which may be different from
    // `std::path::MAIN_SEPARATOR_STR`.
    const OUT_DIR: &str = env!("OUT_DIR");
    let unix_parts = str_split!(OUT_DIR, std::path::MAIN_SEPARATOR);
    unix_parts[unix_parts.len() - 4]
};

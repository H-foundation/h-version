use std::cmp::Ordering;
use std::fmt::{Debug, Display};

///
///
/// # Example
/// parts of version can be ignored
/// ```
/// use std::cmp::Ordering::Greater;
/// use h_version::Version;
/// let version1 = Version::parse("1:123.543.57-alpha+001");
/// let version2 = Version::parse("1:123.543.56-beta+002");
/// assert_eq!(version1.cmp(&version2),Greater);
/// ```
pub struct Version {
    pub epoch: Option<u64>, // epochs (e.g., "1:2.3.4")
    pub components: Vec<String>, // Main version components (e.g., 1.2.3)
    pub pre_release: Option<String>, // Pre-release tag (e.g., "alpha", "beta", "Snapshot", "rc")
    pub build_metadata: Option<String>, // Build metadata (e.g., "+001")
}
impl Version {
    /// makes a version from a str.
    /// # Example
    /// ```
    /// use h_version::Version;
    /// let version = Version::parse("1:23423.553.845-rc+255");
    /// let version = version.to_string();
    /// assert_eq!(version,"1:23423.553.845-rc+255".to_string());
    pub fn parse(version_str: &str) -> Self {
        // Handle epochs
        let mut parts = version_str.splitn(2, ':');
        let epoch = parts.next().and_then(|s| s.parse::<u64>().ok());
        let rest = parts.next().unwrap_or(version_str);

        // Split into main version and build metadata
        let mut parts = rest.splitn(2, '+');
        let version_part = parts.next().unwrap_or(rest);
        let build_metadata = parts.next().map(|s| s.to_string());

        // Split into main version and pre-release
        let mut parts = version_part.splitn(2, '-');
        let main_version = parts.next().unwrap_or(version_part);
        let pre_release = parts.next().map(|s| s.to_string());

        // Split main version into components
        let components: Vec<String> = main_version
            .split(['.','-']) // Split on `.` or `-`
            .map(|s| s.to_string())
            .collect();

        Version {
            epoch,
            components,
            pre_release,
            build_metadata,
        }
    }
}
impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.epoch == other.epoch
            && self.components == other.components
            && self.pre_release == other.pre_release
            && self.build_metadata == other.build_metadata
    }
    fn ne(&self, other: &Self) -> bool {
        self.epoch != other.epoch
            || self.components != other.components
            || self.pre_release != other.pre_release
            || self.build_metadata != other.build_metadata
    }
}
impl Eq for Version {}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare epochs
        if let Some(epoch_cmp) = self.epoch.partial_cmp(&other.epoch) {
            if epoch_cmp != Ordering::Equal {
                return epoch_cmp;
            }
        }

        // Compare main components
        for (a, b) in self.components.iter().zip(&other.components) {
            let a_num = a.parse::<u64>().ok();
            let b_num = b.parse::<u64>().ok();

            match (a_num, b_num) {
                // Numeric comparison
                (Some(a_num), Some(b_num)) => {
                    let cmp = a_num.cmp(&b_num);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                // If one is numeric and the other is not, the numeric one is smaller
                (Some(_), None) => return Ordering::Less,
                (None, Some(_)) => return Ordering::Greater,
                // comparison for non-numeric components
                (None, None) => {
                    let cmp = a.cmp(b);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
            }
        }

        // If main components are equal, compare pre-releases
        match (&self.pre_release, &other.pre_release) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Greater, // No pre-release is greater
            (Some(_), None) => Ordering::Less, // Pre-release is less
            (Some(a), Some(b)) => a.to_lowercase().cmp(&b.to_lowercase()), // Compare pre-releases (because alpha, beta and rc are in order there is no need to compare them one by one. just compare the strings of them.)
        }
    }
}
impl Debug for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let epoch = self.epoch.unwrap_or_default();
        let components = &self.components;
        let pre_release = self.pre_release.clone().unwrap_or_default();
        let build_metadata = self.epoch.unwrap_or_default();
        write!(f, "epoch:{epoch} components:{components:?} pre_release:{pre_release} build_metadata:{build_metadata}")
    }
}
impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        // epoch
        let epoch = self.epoch;
        if let Some(epoch) = epoch {
            string += epoch.to_string().as_str();
            string += ":";
        }
        // components
        let components = &self.components;
        for component in components {
            string += component.as_str();
            string += ".";
        }
        string.remove(string.len() - 1);
        // pre_release
        let pre_release = self.pre_release.clone();
        if let Some(pre_release) = pre_release {
            string += "-";
            string += pre_release.as_str();
        }
        let build_metadata = self.build_metadata.clone();
        if let Some(build_metadata) = build_metadata {
            string += "+";
            string += build_metadata.as_str();
        }
        write!(f, "{}",string)
    }
}
impl Default for Version {
    fn default() -> Self {
        Version::parse("0.0.1")
    }
}
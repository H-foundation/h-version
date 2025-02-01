use std::cmp::Ordering;
use std::fmt::{Debug, Display};

pub struct Version {
    epoch: Option<u64>, // For Debian-style epochs (e.g., "1:2.3.4")
    components: Vec<String>, // Main version components (e.g., ["1", "2", "3"])
    pre_release: Option<String>, // Pre-release tag (e.g., "alpha", "beta", "Snapshot")
    build_metadata: Option<String>, // Build metadata (e.g., "+001")
}
impl Version {
    pub fn parse(version_str: &str) -> Self {
        // Handle Debian-style epochs (e.g., "1:2.3.4")
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
    pub fn to_string(&self) -> String {
        let mut string = String::new();
        // epoch
        let epoch = self.epoch;
        if epoch.is_some() {
            string += epoch.unwrap().to_string().as_str();
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
        if pre_release.is_some() {
            string += "-";
            string += pre_release.unwrap().as_str();
        }
        let build_metadata = self.build_metadata.clone();
        if build_metadata.is_some() {
            string += "+";
            string += build_metadata.unwrap().as_str();
        }
        string
    }
}
impl PartialEq for Version {
    fn eq(&self, other: &Self) -> bool {
        self.epoch == other.epoch
            && self.components == other.components
            && self.pre_release == other.pre_release
            && self.build_metadata == other.build_metadata
    }
}
impl Eq for Version {}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare epochs (Debian-style)
        if let Some(epoch_cmp) = self.epoch.partial_cmp(&other.epoch) {
            if epoch_cmp != Ordering::Equal {
                return Some(epoch_cmp);
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
                        return Some(cmp);
                    }
                }
                // If one is numeric and the other is not, the numeric one is smaller
                (Some(_), None) => return Some(Ordering::Less),
                (None, Some(_)) => return Some(Ordering::Greater),
                // Lexicographic comparison for non-numeric components
                (None, None) => {
                    let cmp = a.cmp(b);
                    if cmp != Ordering::Equal {
                        return Some(cmp);
                    }
                }
            }
        }

        // If main components are equal, compare pre-releases
        match (&self.pre_release, &other.pre_release) {
            (None, None) => Some(Ordering::Equal),
            (None, Some(_)) => Some(Ordering::Greater), // No pre-release is greater
            (Some(_), None) => Some(Ordering::Less), // Pre-release is less
            (Some(a), Some(b)) => Some(a.cmp(b)), // Compare pre-releases lexicographically
        }
    }
}
impl Ord for Version {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
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
        write!(f, "{}",self.to_string())
    }
}
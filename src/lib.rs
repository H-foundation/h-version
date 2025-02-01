mod version;
#[cfg(test)]
mod tests {
    use crate::version::Version;
    #[test]
    fn version_test() {
        let v1 = Version::parse("1.2.3-alpha+001");
        let v2 = Version::parse("1.2.3-beta+002");
        let v3 = Version::parse("2023.03.01");
        let v4 = Version::parse("2.sjf.5djf");
        let v5 = Version::parse("1:2.3.4");
        let v6 = Version::parse("1.0.0-SNAPSHOT");

        assert_eq!(v1 < v2, true); // true
        assert_eq!(v1 > v3, false); // false
        assert_eq!(v4 == v4, true); // true
        assert_eq!(v5 > v1, true); // true
        assert_eq!(v6 < v1, true); // true
    }
    #[test]
    fn printing(){
        let v1 = Version::parse("1.2.3-alpha+001");
        let v2 = Version::parse("2023.03.01");
        let v3 = Version::parse("2.sjf.5djf");
        let v4 = Version::parse("1:2.3.4");
        let v5 = Version::parse("1.0.0-SNAPSHOT");

        let v1_display = format!("{}", v1);
        assert_eq!(v1_display, "1.2.3-alpha+001");
        let v1_debug = format!("{:?}", v1);
        assert_eq!(v1_debug, "epoch:0 components:[\"1\", \"2\", \"3\"] pre_release:alpha build_metadata:0");

        let v2_display = format!("{}", v2);
        assert_eq!(v2_display, "2023.03.01");
        let v2_debug = format!("{:?}", v2);
        assert_eq!(v2_debug, "epoch:0 components:[\"2023\", \"03\", \"01\"] pre_release: build_metadata:0");

        let v3_display = format!("{}", v3);
        assert_eq!(v3_display, "2.sjf.5djf");
        let v3_debug = format!("{:?}", v3);
        assert_eq!(v3_debug, "epoch:0 components:[\"2\", \"sjf\", \"5djf\"] pre_release: build_metadata:0");

        let v4_display = format!("{}", v4);
        assert_eq!(v4_display, "1:2.3.4");
        let v4_debug = format!("{:?}", v4);
        assert_eq!(v4_debug, "epoch:1 components:[\"2\", \"3\", \"4\"] pre_release: build_metadata:1");

        let v5_display = format!("{}", v5);
        assert_eq!(v5_display, "1.0.0-SNAPSHOT");
        let v5_debug = format!("{:?}", v5);
        assert_eq!(v5_debug, "epoch:0 components:[\"1\", \"0\", \"0\"] pre_release:SNAPSHOT build_metadata:0");
    }
}

use pretty_assertions::assert_eq;

use super::*;

#[test]
fn evaluate_compatibility_classifies_exact_patch_older_and_newer_versions() {
    assert_eq!(
        evaluate_compatibility("v3.3.5", "v3.3.5"),
        Compatibility::ExactMatch {
            server: "v3.3.5".into(),
        }
    );

    assert_eq!(
        evaluate_compatibility("3.3.5", "v3.3.5"),
        Compatibility::ExactMatch { server: "3.3.5".into() }
    );

    assert_eq!(
        evaluate_compatibility("v3.3.4", "v3.3.5"),
        Compatibility::Compatible {
            server: "v3.3.4".into(),
            target: "v3.3.5",
        }
    );

    assert_eq!(
        evaluate_compatibility("v3.2.0", "v3.3.5"),
        Compatibility::ServerOlder {
            server: "v3.2.0".into(),
            target: "v3.3.5",
        }
    );

    assert_eq!(
        evaluate_compatibility("v3.4.0", "v3.3.5"),
        Compatibility::ServerNewer {
            server: "v3.4.0".into(),
            target: "v3.3.5",
        }
    );
}

#[test]
fn evaluate_compatibility_handles_prerelease_and_build_metadata() {
    let rc = evaluate_compatibility("v3.3.5-rc.1", "v3.3.5");
    assert_eq!(
        rc,
        Compatibility::Compatible {
            server: "v3.3.5-rc.1".into(),
            target: "v3.3.5",
        }
    );
    assert!(rc.is_compatible());

    let build = evaluate_compatibility("v3.4.0+build.42", "v3.3.5");
    assert_eq!(
        build,
        Compatibility::ServerNewer {
            server: "v3.4.0+build.42".into(),
            target: "v3.3.5",
        }
    );
    assert!(!build.is_compatible());
}

#[test]
fn evaluate_compatibility_flags_invalid_semver_as_unparsable() {
    for invalid in ["", "unknown", "v3", "3.3.x", "1.2.3.4"] {
        let res = evaluate_compatibility(invalid, "v3.3.5");
        assert_eq!(
            res,
            Compatibility::Unparsable {
                server: invalid.into(),
                target: "v3.3.5",
            }
        );
        assert!(!res.is_compatible());
    }
}

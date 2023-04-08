
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 67,
                        patch: 0,
                        pre: vec![],
                        build: vec![],
                    },
                    host: "x86_64-unknown-linux-gnu".to_owned(),
                    short_version_string: "rustc 1.67.0 (fc594f156 2023-01-24)".to_owned(),
                    commit_hash: Some("fc594f15669680fa70d255faec3ca3fb507c3405".to_owned()),
                    commit_date: Some("2023-01-24".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                }
            }
            
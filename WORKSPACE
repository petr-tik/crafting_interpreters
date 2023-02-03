workspace(name = "crafting_interpreters")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")


http_archive(
    name = "rules_cc",
    sha256 = "188e88d911b69bb77b391e02a4a5fdec2313dd01f0305d97cd3451e38f7ace10",
    strip_prefix = "rules_cc-d545fa4f798f2a0b82f556b8b0ec59a93c100df7",
    urls = [
        "https://github.com/bazelbuild/rules_cc/archive/d545fa4f798f2a0b82f556b8b0ec59a93c100df7.zip",
    ],
)

load("@rules_cc//cc:repositories.bzl", "rules_cc_toolchains")

rules_cc_toolchains()

http_archive(
    name = "rules_rust",
    sha256 = "7fb9b4fe1a6fb4341bdf7c623e619460ecc0f52d5061cc56abc750111fba8a87",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.7.0/rules_rust-v0.7.0.tar.gz"],
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")
load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_deps")

rules_rust_dependencies()

rust_register_toolchains()

rust_analyzer_deps()

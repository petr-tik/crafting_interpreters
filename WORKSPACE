workspace(name = "crafting_interpreters")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")


http_archive(
    name = "rules_cc",
    urls = ["https://github.com/bazelbuild/rules_cc/releases/download/0.0.9/rules_cc-0.0.9.tar.gz"],
    sha256 = "2037875b9a4456dce4a79d112a8ae885bbc4aad968e6587dca6e64f3a0900cdf",
    strip_prefix = "rules_cc-0.0.9",
)

load("@rules_cc//cc:repositories.bzl", "rules_cc_toolchains")

rules_cc_toolchains()

http_archive(
    name = "bazel_skylib",
    sha256 = "f24ab666394232f834f74d19e2ff142b0af17466ea0c69a3f4c276ee75f6efce",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/1.4.0/bazel-skylib-1.4.0.tar.gz",
        "https://github.com/bazelbuild/bazel-skylib/releases/download/1.4.0/bazel-skylib-1.4.0.tar.gz",
    ],
)

load("@bazel_skylib//:workspace.bzl", "bazel_skylib_workspace")

bazel_skylib_workspace()

http_archive(
    name = "rules_rust",
    sha256 = "a761d54e49db06f863468e6bba4a13252b1bd499e8f706da65e279b3bcbc5c52",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.36.2/rules_rust-v0.36.2.tar.gz"],
)

http_archive(
    name = "bobs_test_suite",
    sha256 = "aea5c365255be196b8f0719820bd5a93f4d21f9f2e17d78de9419d0e6815fa96",
    urls = [
        "https://github.com/munificent/craftinginterpreters/archive/01e6f5b8f3e5dfa65674c2f9cf4700d73ab41cf8.zip"
    ],
    strip_prefix = "craftinginterpreters-01e6f5b8f3e5dfa65674c2f9cf4700d73ab41cf8",
    build_file_content ="""exports_files(glob(["test/**/*.lox"]), visibility=["//rules:__pkg__", "//jlox:__pkg__"])""",
)

load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")
load("@rules_rust//tools/rust_analyzer:deps.bzl", "rust_analyzer_dependencies")

rules_rust_dependencies()

rust_register_toolchains(
    edition = "2021",
    versions = [
        "1.75.0"
    ],
)

rust_analyzer_dependencies()

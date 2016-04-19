pkg_name=hab-sup
pkg_origin=core
pkg_version=0.4.0
pkg_maintainer="The Habitat Maintainers <humans@habitat.sh>"
pkg_license=('apachev2')
pkg_source=nosuchfile.tar.gz
pkg_deps=(
  core/busybox-static
  core/glibc core/gcc-libs core/libarchive core/libsodium core/openssl
)
pkg_build_deps=(core/coreutils core/cacerts core/rust core/gcc)
pkg_bin_dirs=(bin)

program=$pkg_name

do_prepare() {
  # Used by Cargo to fetch registries/crates/etc.
  export SSL_CERT_FILE=$(pkg_path_for cacerts)/ssl/cert.pem
  build_line "Setting SSL_CERT_FILE=$SSL_CERT_FILE"

  export rustc_target="x86_64-unknown-linux-gnu"
  build_line "Setting rustc_target=$rustc_target"

  export LIBARCHIVE_LIB_DIR=$(pkg_path_for libarchive)/lib
  export LIBARCHIVE_INCLUDE_DIR=$(pkg_path_for libarchive)/include
  export OPENSSL_LIB_DIR=$(pkg_path_for openssl)/lib
  export OPENSSL_INCLUDE_DIR=$(pkg_path_for openssl)/include
  export SODIUM_LIB_DIR=$(pkg_path_for libsodium)/lib
}

do_build() {
  pushd $PLAN_CONTEXT > /dev/null
  cargo clean --target=$rustc_target --verbose
  cargo build \
    -j $(nproc) \
    --target=$rustc_target \
    --release \
    --verbose
  popd > /dev/null
}

do_install() {
  install -v -D $PLAN_CONTEXT/target/$rustc_target/release/$program \
    $pkg_prefix/bin/$program
}

do_strip() {
  strip $pkg_prefix/bin/$program
}

# Turn the remaining default phases into no-ops
do_download() {
  return 0
}

do_verify() {
  return 0
}

do_unpack() {
  return 0
}
{
  mkShell,
  rust-analyzer-unwrapped,
  rustfmt,
  clippy,
  cargo,
  taplo,
  rustc,
  rustPlatform,
  gnuplot,
}:
mkShell {
  strictDeps = true;

  nativeBuildInputs = [
    cargo
    rustc

    rust-analyzer-unwrapped
    (rustfmt.override {asNightly = true;})
    clippy
    taplo

    gnuplot # For Criterion.rs plots
  ];

  env.RUST_SRC_PATH = "${rustPlatform.rustLibSrc}";
}

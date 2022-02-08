#[macro_use]
extern crate build_cfg;

#[build_cfg_main]
fn main()
{
  if build_cfg!(feature = "broadwell")
  {
    println!("cargo:rerun-if-changed=src/perf/broadwell.c");
    println!("cargo:rerun-if-changed=src/perf/broadwell.h");
    cc::Build::new()
            .file("src/perf/broadwell.c")
            .compile("libperf_broadwell.a");
  }
  else if build_cfg!(feature = "m5")
  {
    println!("cargo:rustc-link-search=/usr/lib");
    println!("cargo:rustc-link-lib=m5");
  }
}

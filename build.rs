use std::{fs, env};

const SOURCE: &str = "mDNSResponder-1096.40.7";

fn main() {
  let outdir = env::var("OUT_DIR").expect("env OUT_DIR");

  cc::Build::new()
    .file(format!(r#"deps\{}\mDNSShared\dnssd_clientstub.c"#, SOURCE))
    .file(format!(r#"deps\{}\mDNSShared\dnssd_ipc.c"#, SOURCE))
    .file(r#"src\shim.c"#)
    .define("WIN32", None)
    .define("_WIN32", None)
    .define("NDEBUG", None)
    .define("_CONSOLE", None)
    .define("NOT_HAVE_GETOPT", None)
    .define("NOT_HAVE_SETLINEBUF", None)
    .define("NOT_HAVE_SA_LEN", None)
    .define("WIN32_LEAN_AND_MEAN", None)
    .define("_CRT_SECURE_NO_DEPRECATE", None)
    .define("_CRT_SECURE_CPP_OVERLOAD_STANDARD_NAMES", Some("1"))
    .define("MDNS_DEBUGMSGS", None)
    .define("USE_TCP_LOOPBACK", None)
    .include(format!(r#"deps\{}\mDNSShared"#, SOURCE))
    .compile("dnssd");

  fs::copy(format!(r#"deps\{}\mDNSShared\dns_sd.h"#, SOURCE), r#"sdk\Include\dns_sd.h"#).unwrap();
  fs::copy(format!(r#"{}\dnssd.lib"#, outdir), r#"sdk\Lib\x64\dnssd.lib"#).unwrap();
}
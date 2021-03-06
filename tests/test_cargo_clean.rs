use support::{project, execs, main_file, basic_bin_manifest, cargo_dir};
use hamcrest::{assert_that, existing_dir, is_not};

fn setup() {
}

test!(cargo_clean_simple {
    let p = project("foo")
              .file("Cargo.toml", basic_bin_manifest("foo").as_slice())
              .file("src/foo.rs", main_file(r#""i am foo""#, []).as_slice());

    assert_that(p.cargo_process("cargo-build"), execs().with_status(0));
    assert_that(&p.build_dir(), existing_dir());

    assert_that(p.process(cargo_dir().join("cargo-clean")),
                execs().with_status(0));
    assert_that(&p.build_dir(), is_not(existing_dir()));
})

test!(different_dir {
    let p = project("foo")
              .file("Cargo.toml", basic_bin_manifest("foo").as_slice())
              .file("src/foo.rs", main_file(r#""i am foo""#, []).as_slice())
              .file("src/bar/a.rs", "");

    assert_that(p.cargo_process("cargo-build"), execs().with_status(0));
    assert_that(&p.build_dir(), existing_dir());

    assert_that(p.process(cargo_dir().join("cargo-clean"))
                 .cwd(p.root().join("src")),
                execs().with_status(0).with_stdout(""));
    assert_that(&p.build_dir(), is_not(existing_dir()));
})

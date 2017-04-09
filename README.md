llvm_bindings
---

Documentation (Doesn't Exist yet)

The goal of this crate is to provide a semi-understandable high level interface into the LLVM and its internals. This
crate builds ontop of the [`llvm-sys`](https://bitbucket.org/tari/llvm-sys.rs), which is a rather _raw_ one to one Rust
to C bindings built from the LLVM's C-FFI which exposed from the core LLVM-C++ codebase.


### Using this crate

Add to your `Cargo.toml`

```
[dependencies]
llvm_bind = "0.1.0"
```

You may want to see the build notes as this requires binding against local LLVM headers which can be _slighly_ nontrivial.

---

### Build Generic Notes

The LLVM's C interface is not stable, and will never be stable. Apple doesn't give a shit. That being said when you
bind _to_ that interface you have to _equally_ fluid... at least until there starts being a _stable_ llvm branch.

That being said `llvm-sys` the raw C++ to C to Rust bindings for the LLVM are not stable, nor consistent across
LLVM versions. `llvm-sys`'s version _may_ have to change based on your local llvm verion. This isn't _rocket science_
`llvm-sys = 40.0.*` just means the local llvm version is 4.0. While `llvm-sys = 39.0.*` implies the local llvm version
is 3.9._something_ got it?

In its current form this library is developed on Fedora25. Which is shipping LLVM v3.9.1 so the local `llvm-sys` is `39.0.*`

### Building Fedora25

You will need to install _some_ system libraries

```
sudo dnf install gcc gcc-c++ llvm-devel redhat-rpm-config ncurses-devel
```

Provided that is done, and you have a valid `rust` and `cargo` install you should be good.

---

### Build Ubuntu (TODO)

----

### Building OSX (TODO)

---

### Building Windows (LMAO)



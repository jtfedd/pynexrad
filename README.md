# pynexrad

## About
This package wraps https://github.com/danielway/nexrad with some
additional data processing and exposes python bindings to download
and retrieve NEXRAD data from AWS. It was written to move expensive
computations out of python and into rust to improve the performance
of my 3d radar viewer project: https://github.com/jtfedd/3d-radar.
The bindings are rather closely coupled to that project and may not
be suitable for general use.

## Running Python Examples
There are some python examples which demonstrate using the bindings
to retrieve NEXRAD data as well as some reference examples which use
https://github.com/ARM-DOE/pyart/ to compare against.

### Install dependencies

```
pip install -r requirements-dev.txt
```

### Build and install pynexrad wheel locally

```
maturin develop
```

### Run examples

```
python examples/<example>.py
```

## Running the Rust viewer
This package contains an example viewer which is written in Rust to
help validate that the radar data is being processed correctly.
It should be built in release mode because it is very inefficient
and it will have poor performance otherwise.

```
cargo run --release --example viewer
```

## Testing changes locally with 3d-radar
From the 3d-radar project, with its virtual environment active, cd
to this project and run `maturin develop`. This will overwrite the
wheel that was installed in the virtual environment with a
development wheel based on your local changes. THen you can cd back
to the 3d-radar project directory and run as normal. When you are
finished and wish to return to a release build, you can simply
`pip uninstall pynexrad` and reinstall dependencies for pip to
download and install a release build.

# Getting Started with KepLemon

## Installation

### From PyPI

KepLemon can be installed directly for any operating system from a command line using `pip install keplemon`.

### From a Local Build

For python users, the preferred installation method is through [PyPI](https://www.pypi.org); however, the package can be
installed locally by following the steps below.

1. `git clone https://github.com/citra-space/keplemon.git`
2. `cargo install cargo-make`
3. `cargo make build-<os>-<architecture>` (e.g. for Linux x86 `cargo make build-linux-amd`)
4. `pip install target/wheels/*.whl`

## Environment Settings

Although not required, it is recommended to explicitly apply the settings in this section before using KepLemon to avoid
unexpected behaviors and inaccurate calculations.

### CPU Management

By default, KepLemon will have access to all available cores when performing parallel functions.  Limit this by calling
`set_thread_count` **_before_** using other KepLemon functions.

```python
>>> from keplemon import set_thread_count, get_thread_count
>>> set_thread_count(4)
>>> get_thread_count()
4
```

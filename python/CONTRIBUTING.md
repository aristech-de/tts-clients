# Contributing to the Python Client

Before being able to build the python client, create a virtual environment called `venv` in the [python](.) directory:

```sh
python3 -m venv venv
```

**Note:** You could also use a different name for the virtual environment, but we made some preconfigurations (e.g. for vscode) that expect the virtual environment to be called `venv`. These configurations make sure that the correct python interpreter is used and that the generated python code is found.

Then activate the virtual environment:

```sh
source venv/bin/activate
```

Then install the required packages:

```sh
pip install -e .[dev]
```

## Run Examples

To run the examples while using the local version of the package, run the package with the `PYTHONPATH` environment variable set to the src directory:

```sh
PYTHONPATH=src python examples/recognize.py
```

## Generating Python Code from Protobuf

To generate python from the proto file, run:

```sh
./proto2Python.sh
```

## Building the Package

To build the library, make sure build and twine are up to date:

```sh
python -m pip install --upgrade build twine
```

Then to build, run:

```sh
rm -rf dist && python -m build
```

## Publishing the Package

To make a test upload to TestPyPi, after building it as described above, run:

```sh
python -m twine upload --repository testpypi dist/*
```

You can then install the package from TestPyPi:

```sh
pip install --index-url https://test.pypi.org/simple/ aristech-tts-client --extra-index-url https://pypi.org/simple
```

To make a final upload to PyPi, run:

```sh
python -m twine upload dist/*
```

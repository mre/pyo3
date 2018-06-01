# Changelog

## 0.2.7 (2018-05-18)

* Fix nightly breakage with proc_macro_path

## 0.2.6 (2018-04-03)

* Fix compatibility with TryFrom trait #137

## 0.2.5 (2018-02-21)

* CPython 3.7 support
* Embedded CPython 3.7b1 crashes on initialization #110
* Generated extension functions are weakly typed #108
* call_method*() crashes when the method does not exist #113
* Allow importing exceptions from nested modules #116

## 0.2.4 (2018-01-19)

* Allow to get mutable ref from PyObject #106
* Drop `RefFromPyObject` trait
* Add Python::register_any() method
* Fix impl `FromPyObject` for `Py<T>`
* Mark method that work with raw pointers as unsafe #95


## 0.2.3 (11-27-2017)

* Proper `c_char` usage #93
* Remove use of now unneeded 'AsciiExt' trait
* Rustup to 1.23.0-nightly 2017-11-07

## 0.2.2 (09-26-2017)

* Rustup to 1.22.0-nightly 2017-09-30

## 0.2.1 (09-26-2017)

* Fix rustc const_fn nightly breakage

## 0.2.0 (08-12-2017)

* Added inheritance support #15
* Added weakref support #56
* Allow to add gc support without implementing PyGCProtocol #57
* Refactor `PyErr` implementation. Drop `py` parameter from constructor.
* Added subclass support #64
* Added `self.__dict__` supoort #68
* Added `pyo3::prelude` module #70
* Better `Iterator` support for PyTuple, PyList, PyDict #75
* Introduce IntoPyDictPointer similar to IntoPyTuple #69

## 0.1.0 (07-23-2017)

* Initial release

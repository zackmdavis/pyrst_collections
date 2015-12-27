What happens if I try to expose Rust's collections types in Python?

```
$ python3
Python 3.4.3 (default, Oct 14 2015, 20:28:29) 
[GCC 4.8.4] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> from pyrst_collections import SortedStringCounter
hello Rust—Python simple FFI world
>>> c = SortedStringCounter()
>>> for count, item in enumerate(("foo", "bar", "quux", "rah"), start=1):
...     for _ in range(count):
...         c.increment(item)
... 
>>> c
SortedStringCounter({"bar": 2, "foo": 1, "quux": 3, "rah": 4})
```

"pyrst" is pronounced like "pierced."


## Bibliography

* [the book](http://doc.rust-lang.org/book/ffi.html)
* [Jim Fleming](http://siciarz.net/24-days-of-rust-calling-rust-from-other-languages/)
* [Andrew Oppenlander](http://oppenlander.me/articles/rust-ffi)
* [Zbigniew Siciarz](http://siciarz.net/24-days-of-rust-calling-rust-from-other-languages/)

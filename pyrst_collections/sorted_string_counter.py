import ctypes
from ctypes import c_char_p, c_void_p

libpyrst = ctypes.CDLL("./underworld/target/debug/libpyrst_collections.so")

libpyrst.hello_world()


libpyrst.increment.argtypes = (c_void_p, c_char_p)
libpyrst.represent.restype = c_char_p


class SortedStringCounter:
    def __init__(self):
        self._counter = libpyrst.sorted_string_counter()

    def increment(self, key):
        libpyrst.increment(self._counter, key.encode())

    def __getitem__(self, key):
        return libpyrst.get(self._counter, key.encode())

    def __repr__(self):
        return "SortedStringCounter({})".format(
            libpyrst.represent(self._counter).decode()
        )

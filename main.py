import rust_interop
rust_interop.py_print()
print(rust_interop.py_add(1, 2))
try:
    rust_interop.py_fail(True)
except ValueError as e:
    print(f"Failed as expected: '{e}'")
Current status: you can call render with a dict


	import pytera
	pt_instance = pytera.new("templates/*")
	pt_instance.render("index.html", {"title": "Welcome"})

More in [the tests](tests/test_pytera.py).

Develop
=======

I use Poetry as venv manager and maturin to build it, so all maturin commands need to be prefixed
with `poetry run`:

	poetry run maturin develop
	poetry run python



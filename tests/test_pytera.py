

import pytera

def test_one_off():
    assert pytera.one_off("Hello {{ world }}", {"world": 'world!'}, autoescape = False) == "Hello world!"

def test_render_templates_from_folder():
    pt_instance = pytera.new("tests/resources/templates/*.txt")
    assert pt_instance
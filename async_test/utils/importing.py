from importlib.util import spec_from_file_location, module_from_spec
from pathlib import Path

from async_test.utils.aio import async_func


def import_from_path(path: Path):
    spec = spec_from_file_location(path.name, str(path))
    imported_file = module_from_spec(spec)
    spec.loader.exec_module(imported_file)

    return imported_file


async_import_from_path = async_func(import_from_path)

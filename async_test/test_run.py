from __future__ import annotations

import asyncio
import inspect
import traceback
import time
from pathlib import Path
from typing import Sequence, Coroutine, Callable

from async_test.exceptions import SkippedTestError
from async_test.utils.importing import import_from_path


class AsyncTestWrapper:
    def __init__(self, test: Callable[..., Coroutine], name: str, test_run: TestRunner):
        """
        Wrapper for a test coroutine function that executes and records the outcome of the test

        :param test:
                The test coroutine function
        :param name:
                The name of the test
        :param test_run:
                The TestRunner object being used to run the test
        """
        self.test = test
        self.name = name
        self.test_run = test_run
        self.timeout = self.test_run.test_timeout_s
        self.description = inspect.getdoc(test)
        self.result: str | None = None
        self.traceback: str | None = None
        self.duration: str | None = None

    def __hash__(self):
        return hash((self.test, self.name, self.test_run, self.timeout, self.description))

    def __repr__(self):
        return f"AsyncTestWrapper({self.name})"

    def __await__(self):
        async def coroutine_func():
            """
            Run test and record outcome of the test
            """

            start = time.time()
            coroutine = self.test()

            try:
                await asyncio.wait_for(coroutine, self.timeout)
            except SkippedTestError:
                self.result = "Skipped"
                self.traceback = traceback.format_exc()
                self.test_run.skipped.append(self)
            except (asyncio.TimeoutError, asyncio.CancelledError):
                coroutine.close()
                self.result = "Error"
                self.traceback = traceback.format_exc()
                self.test_run.errors.append(self)
            except AssertionError:
                self.result = "Failed"
                self.traceback = traceback.format_exc()
                self.test_run.failures.append(self)
            except Exception:
                self.result = "Error"
                self.traceback = traceback.format_exc()
                self.test_run.errors.append(self)
            else:
                self.result = "Passed"
                self.traceback = ""
                self.test_run.passed.append(self)
            finally:
                self.duration = time.time() - start

        return coroutine_func().__await__()


class TestRunner:
    def __init__(self, test_dirs: Sequence[Path], test_indicator: str = "test", test_timeout_s: int = 600):
        """
        An object that takes a sequence of directories; finds all mock_tests contained within that have the
        test_indicator and runs them concurrently

        :param test_dirs: Sequence[Path]
                A sequence of paths to directories of mock_tests or test cases
        :param test_indicator: str
                default: "test"
                A indication string that is contained in the name of all test directories and test files
                Example: test_run.py & test_indicator = "test"
        :param test_timeout_s: int
                default: 600
                The maximum to allow a test to run before stopping the test and adding it to errors
        """
        self.test_dirs = test_dirs
        self.test_indicator = test_indicator
        self.test_timeout_s = test_timeout_s
        self.tests = []

        self.errors = []
        self.errors_on_import = []
        self.failures = []
        self.skipped = []
        self.passed = []

        self.find_tests()

    def find_tests(self):
        """
        Finds all the mock_tests and imports them from the path
        """
        for test_dir in self.test_dirs:
            if test_dir.is_dir():
                for test_file in test_dir.glob(f"**/*{self.test_indicator}*.py"):
                    self.import_tests(test_file)
            else:
                self.import_tests(test_dir)

    def import_tests(self, path: Path):
        """
        Imports the mock_tests from path and appends them to self.mock_tests
        :param path: Path of the test file containing a test / multiple mock_tests
        """
        try:
            test_file = import_from_path(path)
        except (SystemExit, Exception):
            self.errors_on_import.append((path.stem, traceback.format_exc()))
        else:
            tests = (
                AsyncTestWrapper(func, func_name, self)
                for func_name, func in inspect.getmembers(test_file, inspect.iscoroutinefunction)
                if "test" in func_name
            )

            for test in tests:
                self.tests.append(test)

    async def run(self):
        """
        Runs all tests in self.tests using asyncio.gather
        """
        await asyncio.gather(*self.tests)


if __name__ == "__main__":
    async def main():
        test_runner = TestRunner([Path("../mock_tests")], test_indicator="mock")
        await test_runner.run()
        x = 0


    asyncio.run(main())

import functools

from async_test.exceptions import SkippedTestError


def skip(reason: str = None):
    def decorator(func):
        @functools.wraps(func)
        async def wrapper(*_, **__):
            raise SkippedTestError(reason)

        return wrapper

    return decorator

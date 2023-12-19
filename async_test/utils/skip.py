from async_test.exceptions import SkippedTestError


def skip_test(reason: str = None):
    def decorator(_):
        def wrapper(*_, **__):
            raise SkippedTestError(reason)

        return wrapper

    return decorator

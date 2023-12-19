import asyncio
import functools
from typing import Callable, Coroutine, Any


def async_func(func: Callable) -> Callable[..., Coroutine[Any, Any, Any]]:
    """
    A decorator that allows the decorated function to run asynchronously using await keyword

    :param func: Callable to run as a coroutine
    :return: Coroutine function
    """
    async def wrapper(*args: Any, **kwargs: Any) -> Any:
        loop = asyncio.get_running_loop()
        return await loop.run_in_executor(None, functools.partial(func, *args, **kwargs))

    return wrapper

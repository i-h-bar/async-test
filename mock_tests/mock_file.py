import asyncio


async def test_stuff_function():
    """I have a description"""
    print("I have tested stuff")


async def test_stuff_async():
    """I have a description"""
    print("I tested other stuff")
    await asyncio.sleep(1)
    print("I waited")

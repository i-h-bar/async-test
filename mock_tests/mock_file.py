import asyncio


async def test_stuff_function():
    print("I have tested stuff")


async def test_stuff_async():
    print("I tested other stuff")
    await asyncio.sleep(1)
    print("I waited")

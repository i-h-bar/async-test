import asyncio

async def test_1_equals_2():
    await asyncio.sleep(1.5)
    assert 1 == 2, "1 doesnt equal 2"


async def test_1_equals_1():
    await asyncio.sleep(2.5)
    assert 1 == 1, "1 is 1"


async def test_2_equals_2():
    await asyncio.sleep(3)
    assert 1 == 1, "1 is 1"
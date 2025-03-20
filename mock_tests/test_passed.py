import asyncio

async def test_case():
    await asyncio.sleep(3)
    assert 1 == 1


async def test_extra():
    await asyncio.sleep(2)
    assert 2 == 2


async def test_timeout():
    await asyncio.sleep(6)
    assert 2 == 2

import asyncio

async def test_case():
    await asyncio.sleep(1.5)
    assert 1 == 2, "1 doesnt equal 2"

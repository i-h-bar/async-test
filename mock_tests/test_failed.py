import asyncio

async def test_case():
    await asyncio.sleep(2)
    assert 1 == 2, "Failure details"

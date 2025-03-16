import asyncio

async def test_case():
    await asyncio.sleep(0.5)
    assert 1 == 2, "Failure details"

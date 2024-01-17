import asyncio


async def test_pass():
    """This test should pass"""
    assert 2 == 2


async def test_fail():
    """This test should fail"""
    assert 1 == 2

from async_test import skip


async def test_error():
    """This test should error"""
    assert 1 + "2" == 3


@skip("This test is broken")
async def test_skip():
    """This test should be skipped"""
    raise TypeError("This test is broken")

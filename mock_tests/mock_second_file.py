from async_test import skip_test


async def another_test_function():
    """I have a description"""
    print("another_test_function")


@skip_test("skip_test")
async def test_skip():
    """I have a description"""
    print("You ran a skipped test")

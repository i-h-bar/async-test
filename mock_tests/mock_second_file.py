from async_test import skip_test


async def another_test_function():
    print("another_test_function")


@skip_test("skip_test")
async def test_skip():
    print("You ran a skipped test")

from mockai.openai import services

payload = {"model": "mock", "messages": [{"role": "user", "content": "Hello!"}]}


async def test_generate_openai_completion_response():
    result = await services.generate_openai_completion_response(
        payload=payload,
        responses=None,
        mock_response='f:{"name": "mom", "arguments": { "number": "1"}}',
    )

    print(result)

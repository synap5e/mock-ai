import pytest
from testcontainers.core.container import DockerContainer
from testcontainers.core.waiting_utils import wait_for_logs


@pytest.fixture(scope="session", autouse=True)
def mockai_server():
    with DockerContainer("ajaczero/mock-ai:test").with_bind_ports(
        8100, 8100
    ) as container:
        wait_for_logs(
            container, "Uvicorn running on http://0.0.0.0:8100", raise_on_exit=True
        )
        yield container

import pytest
from unittest.mock import patch
from moirai.core.tasks.start_task import StartTask


@pytest.fixture
def start_task():
    return StartTask(
        task_id="start_task",
        name="StartTask",
        parameters=[],
        inputs=[],
        outputs=[
            {
                "variable_name": "output_time",
                "value": 0.0,
                "type": "float",
                "role": "output",
            }
        ],
        edges=[{"condition": "success", "target": ""}],
    )


@patch("time.perf_counter", return_value=123.456)
def test_execute(mock_perf_counter, start_task):
    start_task.execute()
    output_time = start_task.get_output("output_time").value
    assert isinstance(output_time, float)
    assert output_time == 123.456


if __name__ == "__main__":
    pytest.main()

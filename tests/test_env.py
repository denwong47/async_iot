# -*- coding: utf-8 -*-
from async_iot import config


def test_env():
    """
    Assert that the PYTEST flag is actually set.
    """
    assert config.env.PYTEST_IS_RUNNING

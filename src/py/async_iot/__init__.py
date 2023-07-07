# -*- coding: utf-8 -*-
"""
================================
 async_iot
================================

An asynchronous unified interface for various IoT devices around a private household.

This project includes a Rust binary backend:

- :mod:`lib_async_iot` which can be loaded as
  :attr:`~async_iot.bin`.
"""

from . import decorators
from . import lib_async_iot as _ffi
from .config import logger

logger = logger.get(__name__)

logger.info("Welcome to async_iot!")
logger.debug(
    "An asynchronous unified interface for various IoT devices around a private household."
)
logger.warning(
    "If you see this message, this means that the Python package was installed "
    "correctly, and __init__.py had run during import."
)
logger.error(
    "This package is currently empty; it does not contain anything of any usefulness."
)
logger.critical("Please populate me!")

# -*- coding: utf-8 -*-
"""
==================
 Logger Utilities
==================

For getting a logger with the correct settings, and with colours and icons injected.
"""
import enum
import functools
import logging
from typing import Callable

from .env import LOGGING_LEVEL


class Brightness(str, enum.Enum):
    BRIGHT = "\033[1m"
    DIM = "\033[2m"
    RESET = "\033[22m"


class Colour(str, enum.Enum):
    BLACK = "\033[30m"
    RED = "\033[31m"
    GREEN = "\033[32m"
    YELLOW = "\033[33m"
    BLUE = "\033[34m"
    MAGENTA = "\033[35m"
    CYAN = "\033[36m"
    WHITE = "\033[37m"
    RESET = "\033[39m"


class Background(str, enum.Enum):
    BLACK = "\033[40m"
    RED = "\033[41m"
    GREEN = "\033[42m"
    YELLOW = "\033[43m"
    BLUE = "\033[44m"
    MAGENTA = "\033[45m"
    CYAN = "\033[46m"
    WHITE = "\033[47m"
    RESET = "\033[49m"


# Short hand for bold coloured texts
black = (
    lambda msg: f"{Colour.BLACK}{Brightness.BRIGHT}{msg}{Brightness.RESET}{Colour.RESET}"
)
red = (
    lambda msg: f"{Colour.RED}{Brightness.BRIGHT}{msg}{Brightness.RESET}{Colour.RESET}"
)
green = (
    lambda msg: f"{Colour.GREEN}{Brightness.BRIGHT}{msg}{Brightness.RESET}{Colour.RESET}"
)
yellow = (
    lambda msg: f"{Colour.YELLOW}{Brightness.BRIGHT}{msg}{Brightness.RESET}{Colour.RESET}"
)
blue = (
    lambda msg: f"{Colour.BLUE}{Brightness.BRIGHT}{msg}{Brightness.RESET}{Colour.RESET}"
)
magenta = (
    lambda msg: f"{Colour.MAGENTA}{Brightness.BRIGHT}{msg}{Brightness.RESET}{Colour.RESET}"
)
cyan = (
    lambda msg: f"{Colour.CYAN}{Brightness.BRIGHT}{msg}{Brightness.RESET}{Colour.RESET}"
)
white = (
    lambda msg: f"{Colour.WHITE}{Brightness.BRIGHT}{msg}{Brightness.RESET}{Colour.RESET}"
)


def wrap_format(format: str):
    """
    Add a prefix to a logging function.
    """

    def _decorator(func: Callable[[str], None]):
        @functools.wraps(func)
        def _wrapper(msg: str, *args, **kwargs):
            return func(format.format(msg=msg), *args, **kwargs)

        return _wrapper

    return _decorator


def get(name: str) -> logging.Logger:
    """
    Get a logger with the specified name.
    """
    logger = logging.getLogger(name)
    logger.setLevel(LOGGING_LEVEL)
    logger.addHandler(logging.StreamHandler())

    logger.debug = wrap_format("\U0001F528 {msg}")(logger.debug)
    logger.info = wrap_format(f"\u2139 {Colour.CYAN}" "{msg}" f"{Colour.RESET}")(
        logger.info
    )
    logger.warn = wrap_format(f"\u26A0 {Colour.YELLOW}" "{msg}" f"{Colour.RESET}")(
        logger.warn
    )
    logger.error = wrap_format(
        f"\u2604 {Colour.RED}{Brightness.BRIGHT}"
        "{msg}"
        f"{Brightness.RESET}{Colour.RESET}"
    )(logger.error)
    logger.critical = wrap_format(
        f"\u2604 {Background.RED}{Brightness.BRIGHT}"
        "{msg}"
        f"{Brightness.RESET}{Background.RESET}"
    )(logger.critical)

    return logger

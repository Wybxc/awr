from . import awr as _awr

_awr.init(_awr)
__version__ = _awr.__version__
__build__ = _awr.__build__

from .awr import At, Client, Dynamic, Face, MessageContent, Password

try:
    from .awr import QrCode
except ImportError:
    pass

__all__ = [
    "Client",
    "Password",
    "QrCode",
    "Dynamic",
    "At",
    "Face",
    "MessageContent",
]

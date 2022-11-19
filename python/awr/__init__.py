from . import awr as _awr

_awr.init(_awr)
__version__ = _awr.__version__

from .awr import At, Client, Dynamic, Face, MessageContent, Password

__all__ = [
    "Client",
    "Password",
    "Dynamic",
    "At",
    "Face",
    "MessageContent",
]

# 检查 avilla 是否启用
try:
    from avilla.core import Avilla as _

    from .avilla import *

    __all__.extend(avilla.__all__)

except ImportError:
    pass

"""
基于 [`ricq`] 包装，供 Python 使用的 QQ 无头客户端。

更多信息请参考 [`login`] 和 [`client`] 模块。

# Examples
```python
import awr
import asyncio

async def main():
    ## 登录账号
    client = await awr.Dynamic().login(12345678, "./bots")
    ## 读取群列表
    print(await client.get_group_list())
    ## 保持连接
    await client.alive()
    
try:
    asyncio.run(main())
except KeyboardInterrupt:
    import sys
    sys.exit(0)
```

[`ricq`]: https://docs.rs/ricq/latest/ricq/
"""

from . import _awr

_awr.init(_awr)
__version__ = _awr.__version__
__build__ = _awr.__build__

from ._awr import Client

try:
    from ._awr import QrCode
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

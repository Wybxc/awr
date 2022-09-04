<center>

# AWR

![Build Status](https://img.shields.io/github/workflow/status/Wybxc/awr/CI)
[![GitHub Issues](https://img.shields.io/github/issues/Wybxc/awr)](https://github.com/Wybxc/awr/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/Wybxc/awr)](https://github.com/Wybxc/awr/pulls)

![LICENCE](https://img.shields.io/github/license/Wybxc/awr)
[![Docs Status](https://img.shields.io/github/workflow/status/Wybxc/awr/API%20Document?label=docs)](https://awr-latest.netlify.app)
[![GitHub](https://img.shields.io/github/last-commit/Wybxc/awr?label=GitHub)](https://github.com/Wybxc/awr)

</center>

基于 [`ricq`](https://github.com/lz1998/ricq) 包装，供 Python 使用的 QQ 无头客户端。

## 开始

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
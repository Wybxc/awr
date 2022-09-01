import asyncio

from launart import ExportInterface, Launart, Service

from avilla.core import Abstract, Avilla, BaseProtocol, Land, Platform
from avilla.core.utilles.event_parser import AbstractEventParser

from .awr import LoginMethod


class AWREventParser(AbstractEventParser["AWRProtocol"]):
    def get_event_type(self, raw: dict) -> str:
        ...


class AWRProtocol(BaseProtocol):
    """Avilla 的 ricq 后端。

    # Example
    ```python
    from creart import create
    from graia.amnesia.builtins.aiohttp import AiohttpService
    from graia.broadcast import Broadcast

    from avilla.core import Avilla, MessageReceived, Relationship, Selector
    from awr import *

    broadcast = create(Broadcast)
    avilla = Avilla(broadcast, [
        AWRProtocol(
            accounts = {
                12345678: Password("password"),
            },
            data_folder = "./bots",
        )
    ], [])

    @broadcast.receiver(MessageReceived)
    async def on_message_received(event: MessageReceived, rs: Relationship):
        if Selector.fragment().as_dyn().group("*").member("master-account").match(rs.ctx):
            await rs.send_message("Hello, Avilla!")

    avilla.launch_manager.launch_blocking(loop=broadcast.loop)
    ```
    """

    platform = Platform(
        Land(
            "qq",
            [{"name": "Wybxc"}],
            humanized_name="AWR - avilla with ricq.",
        ),
        Abstract(
            protocol="ricq",
            maintainers=[{"name": "lz1998"}],
            humanized_name="ricq protocol",
        ),
    )

    event_parser = AWREventParser()

    accounts: dict[int, LoginMethod]
    """用户账户登录信息。"""

    data_folder: str
    """bot 数据储存目录。
    
    特定账户的数据将存放在 `<data_folder>/<account>` 目录中。
    """

    def __init__(
        self, accounts: dict[int, LoginMethod] = {}, data_folder: str = "./bots"
    ):
        """
        - `accounts` - 用户的 QQ 号、密码、登录方式等信息。
        """
        self.accounts = accounts

    def ensure(self, avilla: Avilla):
        # Ensureable 用于注册各种东西, 包括 Service, ResourceProvider 等.
        avilla.launch_manager.add_service(AWRService(self))


class AWRInterface(ExportInterface["AWRService"]):
    """接口服务。

    主动调用 API 支持。"""

    service: "AWRService"

    def __init__(self, service: "AWRService"):
        self.service = service


class AWRService(Service):
    """后台服务。

    保持长连接，接收并处理消息。"""

    supported_interface_types = {AWRInterface}

    def required(self):
        return set()

    def stages(self):
        return {"preparing", "blocking"}

    def get_interface(self, interface_type):
        if interface_type is not AWRInterface:
            raise TypeError(f"不支持的接口类型：{interface_type}")
        return AWRInterface(self)

    protocol: AWRProtocol

    def __init__(self, protocol: AWRProtocol):
        self.protocol = protocol

    async def launch(self, manager: Launart):
        async with self.stage("preparing"):
            # 登录账号
            handles = []
            for uin, method in self.protocol.accounts.items():
                handle = await method.login(uin, self.protocol.data_folder)
                handles.append(handle)

        async with self.stage("blocking"):
            # 阻塞等待消息
            exit_mark = asyncio.create_task(manager.status.wait_for_sigexit())
            while not exit_mark.done():
                await asyncio.wait(
                    [asyncio.gather(*handles), exit_mark],
                    return_when=asyncio.FIRST_COMPLETED,
                )


__all__ = ["AWRProtocol"]

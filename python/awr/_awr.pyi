from __future__ import annotations

import datetime
import enum
from typing import Any, Callable, Iterator, Literal, Sequence, Tuple, overload

################################################################################
# lib.rs

__version__: str
__build__: dict[str, Any]

def init(module: Any) -> None:
    """初始化 AWR 环境：
    - 设置日志输出。
    - 打印版本信息。
    """

################################################################################
# login.rs

class Protocol(enum.Enum):
    """协议。

    | 协议 | 说明 |
    | --- | --- |
    | `IPAD` | iPad 协议 |
    | `ANDROID_PHONE` | Android 手机协议 |
    | `ANDROID_WATCH` | Android 手表协议 |
    | `MAC_OS` | MacOS 客户端协议 |
    | `QI_DIAN` | 企点协议 |
    """

    IPAD = enum.auto()
    ANDROID_PHONE = enum.auto()
    ANDROID_WATCH = enum.auto()
    MAC_OS = enum.auto()
    QI_DIAN = enum.auto()

class AliveHandle:
    """登录保持。

    awr 在登录完成后，并不会主动阻塞程序，而是需要用户手动开始保持连接。
    这给了用户更多的控制权，比如用户可以将登录保持纳入自己的异步任务实现中。

    `auto_reconnect` 方法返回一个 Future，理论上此 Future 会无限期等待，除非有错误发生。
    等待期间，awr 会自动进行断线重连。

    如果需要更细粒度的控制，可以使用 `alive` 和 `reconnect` 方法，分别用于保持连接和重连。
    `auto_reconnect` 方法实际上是不断循环调用 `alive` 和 `reconnect` 方法。

    # Examples
    ```python
    client, alive = awr.login(12345678, password="xxxxxx", protocol=awr.Protocol.IPad)

    while True:
        await alive.alive()
        print("连接断开，正在重连...")
        await alive.reconnect()
        print("重连成功")
    ```
    """

    async def alive(self):
        """等待，直到连接断开。

        # Note
        此方法的 Python 绑定带有借用检查，同一时间只能有一个调用。
        重复调用会引发 `RuntimeError`。
        """
    async def reconnect(self):
        """断线重连。

        # Safety
        此方法不会检查连接是否已经断开，如果连接未断开，调用此方法会导致不可预知的行为。

        建议只在 `alive` 方法返回后调用此方法。

        # Note
        此方法的 Python 绑定带有借用检查，同一时间只能有一个调用。
        重复调用会引发 `RuntimeError`。
        """
    async def auto_reconnect(self):
        """开始自动断线重连。

        此方法相当于无限循环调用 `alive` 和 `reconnect` 方法。

        # Note
        此方法的 Python 绑定带有借用检查，并且消耗所有权。
        调用此方法后，对此对象的后续使用会引发 `RuntimeError`。
        """

async def login_with_password(
    uin: int,
    password: str,
    protocol: Protocol,
    data_folder: str = "./bots",
) -> Tuple[Client, AliveHandle]:
    """使用密码登录。"""

async def login_with_password_md5(
    uin: int,
    password_md5: bytes,
    protocol: Protocol,
    data_folder: str = "./bots",
) -> Tuple[Client, AliveHandle]:
    """使用密码 MD5 登录。"""

async def login_with_qrcode(
    uin: int,
    show_qrcode: Callable[[bytes], None],
    data_folder: str = "./bots",
) -> Tuple[Client, AliveHandle]:
    """使用二维码登录。

    二维码图片会通过 `show_qrcode` 回调函数传递给调用者。
    调用者需要自行实现二维码图片的显示。

    # Examples

    下面是一个使用 [Pillow](https://pypi.org/project/Pillow/) 显示二维码图片的例子。

    ```python
    from io import BytesIO
    from PIL import Image

    client, alive = await awr.login(
        12345678,
        show_qrcode = lambda img: Image.open(BytesIO(img)).show(),
    )
    ```
    """

@overload
async def login(
    uin: int, *, password: str, protocol: Protocol, data_folder="./bots"
) -> Tuple[Client, AliveHandle]:
    """登录。"""

@overload
async def login(
    uin: int, *, password_md5: str, protocol: Protocol, data_folder="./bots"
) -> Tuple[Client, AliveHandle]:
    """登录。"""

@overload
async def login(
    uin: int, *, show_qrcode: Callable[[bytes], None], data_folder="./bots"
) -> Tuple[Client, AliveHandle]:
    """登录。"""

################################################################################
# client/mod.rs

class Client:
    """客户端。"""

    @property
    def uin(self) -> int:
        """当前账号的 QQ 号。"""
    async def set_friend_list_cache_time(self, time: datetime.timedelta) -> None:
        """设置好友列表缓存过期时间。"""
    async def set_group_cache_time(self, time: datetime.timedelta) -> None:
        """设置群信息缓存过期时间。"""
    async def set_group_member_list_cache_time(self, time: datetime.timedelta) -> None:
        """设置群成员列表缓存过期时间。"""
    def is_online(self) -> bool:
        """当前账号是否在线。"""
    def account_info(self) -> AccountInfoSelector:
        """构造账号信息选择器。"""
    async def get_account_info(self) -> AccountInfo:
        """获取账号信息。"""
    async def get_friend_list(self) -> FriendList:
        """获取好友列表对象。

        好友列表会被缓存，如果缓存未过期则直接返回缓存的值。
        如果需要强制刷新好友列表缓存，请使用 `flush_friend_list`。
        """
    async def flush_friend_list(self) -> None:
        """刷新好友列表缓存。"""
    def friend(self, uin: int) -> FriendSelector:
        """构造好友选择器。"""
    async def get_friend(self, uin: int) -> Friend | None:
        """获取好友对象。
        好友对象会缓存在好友列表缓存中，如果缓存未过期则直接返回缓存的值。
        如果需要强制刷新好友对象缓存，请使用 `FriendSelector::flush` 或 `flush_friend_list`。
        """
    def friend_group(self, group_id: int) -> FriendGroupSelector:
        """构造好友分组选择器。"""
    async def get_friend_group(self, id: int) -> FriendGroup | None:
        """获取好友分组对象。

        好友分组对象会缓存在好友列表缓存中，如果缓存未过期则直接返回缓存的值。
        如果需要强制刷新好友分组对象缓存，请使用 `FriendGroupSelector::flush` 或 `flush_friend_list`。
        """
    async def create_friend_group(self, name: str) -> None:
        """创建好友分组。
        此方法会强制更新好友列表缓存。
        """
    def group(self, code: int) -> GroupSelector:
        """构造群选择器。"""
    async def get_group(self, code: int) -> Group | None:
        """获取群对象。

        群对象会被缓存，如果缓存未过期则直接返回缓存的值。
        如果需要强制刷新群对象缓存，请使用 `GroupSelector::flush`。"""
    def groups(self, *codes: int) -> MultiGroupSelector:
        """构造多个群选择器。"""
    async def get_groups(self, *codes: int) -> dict[int, Group]:
        """获取多个群对象。

        群对象会被缓存，如果缓存未过期则直接返回缓存的值。
        如果需要强制刷新群对象缓存，请使用 `MultiGroupSelector::flush`。
        """
    def all_groups(self) -> AllGroupSelector:
        """构造所有群选择器。"""
    async def get_all_groups(self) -> dict[int, Group]:
        """获取所有群对象。

        此方法会刷新所有群对象的缓存。
        """
    def group_member(self, group_code: int, uin: int) -> GroupMemberSelector:
        """构造群成员选择器。"""
    async def get_group_member(self, group_code: int, uin: int) -> GroupMember | None:
        """获取群成员对象。

        群成员对象会被缓存，如果缓存未过期则直接返回缓存的值。
        如果需要强制刷新群成员对象缓存，请使用 `GroupMemberSelector::flush`。
        """
    def group_member_list(self, group_code: int) -> GroupMemberListSelector:
        """构造群成员列表选择器。"""
    async def get_group_member_list(self, group_code: int) -> GroupMemberList | None:
        """获取群成员列表。

        群成员列表会被缓存，如果缓存未过期则直接返回缓存的值。
        如果需要强制刷新群成员列表缓存，请使用 `GroupMemberListSelector::flush`。
        """

################################################################################
# client/account_info.rs

class AccountInfo:
    """账号信息。"""

    @property
    def nickname(self) -> str:
        """昵称。"""
    @property
    def age(self) -> int:
        """年龄。"""
    @property
    def gender(self) -> int:
        """性别。"""

class AccountInfoSelector:
    """账号信息选择器。"""

    async def flush(self) -> AccountInfoSelector:
        """刷新缓存。"""
    async def fetch(self) -> AccountInfo:
        """获取账号信息。"""
    async def flush_and_fetch(self) -> AccountInfo:
        """刷新缓存并获取账号信息。"""
    def as_client(self) -> Client:
        """获取客户端引用。"""

################################################################################
# client/friend_group.rs

class FriendGroup:
    """好友分组。"""

    @property
    def id(self) -> int:
        """好友分组编号。"""
    @property
    def name(self) -> str:
        """好友分组名称。"""
    @property
    def friend_count(self) -> int:
        """好友分组好友数。"""
    @property
    def online_count(self) -> int:
        """在线好友数。"""
    @property
    def seq_id(self) -> int:
        """好友分组排序。"""
    async def flush(self) -> FriendGroupSelector:
        """刷新缓存。"""
    async def flush_and_fetch(self) -> FriendGroup | None:
        """刷新缓存并获取好友分组信息。"""
    def as_selector(self) -> FriendGroupSelector:
        """获取好友分组选择器。"""
    def as_client(self) -> Client:
        """获取客户端引用。"""

class FriendGroupSelector:
    """好友分组选择器。"""

    @property
    def id(self) -> int:
        """好友分组编号。"""
    async def flush(self) -> FriendGroupSelector:
        """刷新缓存。"""
    async def fetch(self) -> FriendGroup | None:
        """获取好友分组信息。"""
    async def flush_and_fetch(self) -> FriendGroup | None:
        """刷新缓存并获取好友分组信息。"""
    def as_client(self) -> Client:
        """获取客户端引用。"""

################################################################################
# client/friend_list.rs

class FriendList:
    """好友列表。"""

    @property
    def total_count(self) -> int: ...
    @property
    def online_count(self) -> int: ...
    def friends(self) -> dict[int, Friend]:
        """获取好友信息。"""
    def friend_groups(self) -> dict[int, FriendGroup]:
        """获取所有好友分组信息。"""
    async def flush(self) -> FriendListSelector:
        """刷新缓存。"""
    async def flush_and_fetch(self) -> FriendList:
        """刷新缓存并获取好友列表信息。"""
    def as_selector(self) -> FriendListSelector:
        """获取好友列表选择器。"""
    def as_client(self) -> Client:
        """获取客户端引用。"""

class FriendListSelector:
    """好友列表选择器。"""

    async def flush(self) -> FriendListSelector:
        """刷新缓存。"""
    async def fetch(self) -> FriendList:
        """获取好友列表信息。"""
    async def flush_and_fetch(self) -> FriendList:
        """刷新缓存并获取好友列表信息。"""
    def as_client(self) -> Client:
        """获取客户端引用。"""

################################################################################
# client/friend.rs

class Friend:
    """好友。"""

    @property
    def uin(self) -> int:
        """好友 QQ 号。"""
    @property
    def nickname(self) -> str:
        """好友昵称。"""
    @property
    def remark(self) -> str:
        """好友备注。"""
    @property
    def face_id(self) -> int:
        """好友头像 ID。"""
    @property
    def group_id(self) -> int:
        """好友分组编号。"""
    def friend_group(self) -> FriendGroupSelector:
        """获取所在的好友分组选择器。"""
    async def poke(self) -> None:
        """戳一戳好友。"""
    async def flush(self) -> FriendSelector:
        """刷新缓存。"""
    async def flush_and_fetch(self) -> Friend | None:
        """刷新缓存并获取好友信息。"""
    def as_selector(self) -> FriendSelector:
        """获取好友选择器。"""
    def as_client(self) -> Client:
        """获取客户端引用。"""

class FriendSelector:
    """好友选择器。"""

    @property
    def uin(self) -> int:
        """好友 QQ 号。"""
    async def poke(self) -> None:
        """戳一戳好友。"""
    async def flush(self) -> FriendSelector:
        """刷新缓存。"""
    async def fetch(self) -> Friend | None:
        """获取好友信息。"""
    async def flush_and_fetch(self) -> Friend | None:
        """刷新缓存并获取好友信息。"""
    def as_client(self) -> Client:
        """获取客户端引用。"""

################################################################################
# client/group.rs

class Group:
    """群聊。"""

    @property
    def uin(self) -> int:
        """uin。

        含义可参考：[#181](https://github.com/Mrs4s/MiraiGo/issues/181)。"""
    @property
    def code(self) -> int:
        """群号。"""
    @property
    def name(self) -> str:
        """群名称。"""
    @property
    def memo(self) -> str:
        """入群公告。"""
    @property
    def owner_uin(self) -> int:
        """群主 QQ 号。"""
    @property
    def group_create_time(self) -> int:
        """群创建时间。"""
    @property
    def group_level(self) -> int:
        """群等级。"""
    @property
    def member_count(self) -> int:
        """群成员数。"""
    @property
    def max_member_count(self) -> int:
        """最大群成员数。"""
    @property
    def shut_up_timestamp(self) -> int:
        """全群禁言时间。"""
    @property
    def my_shut_up_timestamp(self) -> int:
        """自己被禁言时间。"""
    @property
    def last_msg_seq(self) -> int | None:
        """最后一条消息的 seq。"""
    def member_list(self) -> GroupMemberListSelector:
        """获取群成员列表选择器。"""
    async def flush(self) -> GroupSelector:
        """刷新缓存。"""
    async def flush_and_fetch(self) -> Group | None:
        """刷新缓存并获取群聊信息。"""
    def as_selector(self) -> GroupSelector:
        """获取群聊选择器。"""
    def as_client(self) -> Client:
        """获取客户端引用。"""

class GroupSelector:
    """群聊选择器。"""

    @property
    def code(self) -> int:
        """群号。"""
    def member_list(self) -> GroupMemberListSelector:
        """获取群成员列表选择器。"""
    def member(self, uin: int) -> GroupMemberSelector:
        """获取群成员选择器。"""
    async def flush(self) -> GroupSelector:
        """刷新缓存。"""
    async def fetch(self) -> Group | None:
        """获取群聊信息。"""
    async def flush_and_fetch(self) -> Group | None:
        """刷新缓存并获取群聊信息。"""
    def as_client(self) -> Client:
        """获取客户端引用。"""

class MultiGroupSelector:
    """多个群聊选择器。"""

    def codes(self) -> list[int]:
        """群号列表。"""
    async def flush(self) -> MultiGroupSelector:
        """刷新缓存。"""
    async def fetch(self) -> dict[int, Group]:
        """获取群聊信息。"""
    async def flush_and_fetch(self) -> Group | None:
        """刷新缓存并获取群聊信息。"""
    def as_client(self) -> Client:
        """获取客户端引用。"""

class AllGroupSelector:
    """所有群聊选择器。"""

    async def flush(self) -> MultiGroupSelector:
        """刷新缓存。"""
    async def fetch(self) -> dict[int, Group]:
        """获取群聊信息。"""
    async def flush_and_fetch(self) -> Group | None:
        """刷新缓存并获取群聊信息。"""
    def as_client(self) -> Client:
        """获取客户端引用。"""

################################################################################
# client/group_member.rs

class GroupMember:
    """群成员。"""
    @property
    def group_code(self) -> int: 
        """群号。"""
    @property
    def uin(self) -> int: 
        """QQ 号。"""
    @property
    def gender(self) -> int: 
        """性别。"""
    @property
    def nickname(self) -> str: 
        """群名片。"""
    @property
    def card_name(self) -> str: 
        """群名片。"""
    @property
    def level(self) -> int: 
        """群等级。"""
    @property
    def join_time(self) -> int: 
        """入群时间。"""
    @property
    def last_speak_time(self) -> int:
        """最后发言时间。"""
    @property
    def special_title(self) -> str: 
        """群头衔。"""
    @property
    def special_title_expire_time(self) -> int: 
        """群头衔过期时间。"""
    @property
    def shut_up_timestamp(self) -> int: 
        """剩余禁言时间。"""
    @property
    def permission(self) -> GroupMemberPermission: 
        """群成员权限。"""

class GroupMemberSelector:
    """群成员选择器。"""
    @property
    def group_code(self) -> int: 
        """群号。"""
    @property
    def uin(self) -> int: 
        """成员 QQ 号。"""
    async def flush(self) -> MultiGroupSelector:
        ...
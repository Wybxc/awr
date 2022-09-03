from typing import Any, Iterator, Literal, Sequence

################################################################################
# lib.rs

__version__: str

def init(module: Any) -> None:
    """初始化 AWR 环境：
    - 设置日志输出。
    - 打印版本信息。
    """

################################################################################
# login.rs

LoginProtocol = Literal[
    "ipad",
    "android",
    "android_phone",
    "watch",
    "android_watch",
    "mac",
    "macos",
    "qidian",
]
"""登录协议。

- `ipad` - iPad 协议。
- `android` `android_phone` - Android 手机协议。
- `watch` `android_watch` - Android 手表协议。
- `mac` `macos` - MacOS 协议。
- `qidian` - 企点协议。
"""

class LoginMethod:
    """登录方式。"""

    def __init__(self, protocol: LoginProtocol = "ipad") -> None:
        """构造登录方式。

        # Arguments
        - `protocol` - 客户端协议。
        """
    async def login(self, uin: int, data_folder: str) -> Client:
        """登录到指定的账号。

        # Arguments
        - `uin` - 用户的 QQ 号。
        - `data_folder` - 数据目录。
        """

class Password(LoginMethod):
    """密码登录。"""

    def __init__(
        self, password: str, protocol: LoginProtocol = "ipad", md5: bool = False
    ) -> None:
        """构造登录方式。

        # Arguments
        - `data_folder` - 数据目录。
        - `protocol` - 客户端协议。
        - `md5` - 是否用密码的 MD5 代替密码。
        """
    async def login(self, uin: int, data_folder: str) -> Client: ...

class QrCode(LoginMethod):
    """二维码登录（仅支持手表协议）。"""

    def __init__(self) -> None:
        """构造登录方式。"""
    async def login(self, uin: int, data_folder: str) -> Client: ...

class Dynamic(LoginMethod):
    """运行时选择登录方式。"""

    def __init__(self, protocol: LoginProtocol | None = "ipad") -> None:
        """构造登录方式。

        # Arguments
        - `protocol` - 客户端协议（可选）。
        """
    async def login(self, uin: int, data_folder: str) -> Client: ...

################################################################################
# client/mod.rs

class Client:
    """QQ 客户端。"""

    async def alive(self) -> None:
        """等待并保持客户端连接。

        多次调用此方法时，后续的调用将直接返回。
        """
    def online(self) -> bool:
        """是否在线。"""
    async def account_info(self) -> AccountInfo:
        """获取账号信息。"""
    async def get_friend_list(self) -> FriendList:
        """获取好友列表。"""
    async def get_group_info(self, group_id: int) -> GroupInfo | None:
        """获取群信息。"""
    async def get_group_infos(self, group_ids: Sequence[int]) -> dict[int, GroupInfo]:
        """批量获取群信息，返回 `{ 群号: 群信息 }` 的字典。"""
    async def get_group_list(self) -> list[GroupInfo]:
        """获取群列表。
        
        # Note
        此方法获取到的 `last_msg_seq` 不可用，如需要此字段请使用 `get_group_info`。
        """

################################################################################
# client/structs.rs

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

class FriendInfo:
    """好友信息。"""

    @property
    def uin(self) -> int:
        """QQ号。"""
    @property
    def nickname(self) -> str:
        """昵称。"""
    @property
    def remark(self) -> str:
        """备注。"""
    @property
    def face_id(self) -> int:
        """"""
    @property
    def group_id(self) -> int:
        """好友分组编号。"""

class FriendGroupInfo:
    """好友分组信息。"""

    @property
    def id(self) -> int:
        """好友分组编号。"""
    @property
    def name(self) -> str:
        """好友分组名称。"""
    @property
    def friend_count(self) -> int:
        """好友数量。"""
    @property
    def online_count(self) -> int:
        """在线好友数量。"""
    @property
    def seq_id(self) -> int:
        """"""

class GroupInfo:
    """群信息。"""

    @property
    def uin(self) -> int:
        """uin。参看：[#181](https://github.com/Mrs4s/MiraiGo/issues/181)"""
    @property
    def code(self) -> int:
        """群号。"""
    @property
    def name(self) -> str:
        """群名。"""
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
        """群成员数量。"""
    @property
    def max_member_count(self) -> int:
        """群最大成员数量。"""
    @property
    def mute_all(self) -> bool:
        """是否全员禁言。"""
    @property
    def my_shut_up_timestamp(self) -> int:
        """自己被禁言剩余时间。"""
    @property
    def last_msg_seq(self) -> int:
        """"""

################################################################################
# client/friend_list.rs

class FriendList:
    """好友列表。"""

    def friends(self) -> Iterator[FriendInfo]:
        """遍历好友信息的迭代器。"""
    def find_friend(self, uin: int) -> FriendInfo | None:
        """查找好友。"""
    def friend_groups(self) -> list[FriendGroupInfo]:
        """获取所有好友分组信息。"""
    def find_friend_group(self, group_id: int) -> FriendGroupInfo | None:
        """查找好友分组。"""

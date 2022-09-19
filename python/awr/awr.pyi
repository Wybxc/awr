from typing import Any, Iterator, Literal, Sequence, TypedDict

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
    @property
    def uin(self) -> int:
        """获取客户端 QQ 号。"""
    def is_online(self) -> bool:
        """是否在线。"""
    def friend(self, uin) -> FriendSelector:
        """获取好友选择器。"""
    async def get_account_info(self) -> AccountInfo:
        """获取账号信息。"""
    async def get_friend_list(self) -> FriendList:
        """获取好友列表。"""
    async def get_friends(self) -> Iterator[Friend]:
        """获取遍历好友信息的迭代器。"""
    async def get_friend(self, uin: int) -> Friend | None:
        """查找指定的好友。"""
    async def get_group(self, group_id: int) -> Group:
        """获取群。"""
    async def get_groups(self, group_ids: Sequence[int]) -> dict[int, Group]:
        """批量获取群，返回 `{ 群号: 群对象 }` 的字典。

        当给出的群号不存在，或者未加入这个群时，将不会在返回值中出现。这意味着返回值长度可能会小于参数长度。
        """
    async def get_group_list(self) -> list[Group]:
        """获取群列表。

        # Note
        此方法获取到的 `last_msg_seq` 不可用，如需要此字段请使用 `get_group` 或 `get_groups`。
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
        """"""
    @property
    def group_id(self) -> int:
        """好友分组编号。"""
    def as_selector(self) -> FriendSelector:
        """获取好友选择器。"""
    async def poke(self) -> None:
        """戳一戳好友。"""
    async def send(self, msg: Sequence[Element]) -> MessageReceipt:
        """发送私聊消息。"""

class FriendSelector:
    """好友选择器。"""

    async def hydrate(self) -> Friend | None:
        """获取好友对象。"""
    async def poke(self) -> None:
        """戳一戳好友。"""
    async def send(self, msg: Sequence[Element]) -> MessageReceipt:
        """发送私聊消息。"""
    async def recall(self, receipt: MessageReceipt) -> None:
        """撤回消息。"""

################################################################################
# client/friend_group.rs

class FriendGroup:
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

################################################################################
# client/friend_list.rs

class FriendList:
    """好友列表。"""

    @property
    def total_count(self) -> int:
        """好友数量。"""
    @property
    def online_count(self) -> int:
        """在线好友数量。"""
    def friends(self) -> Iterator[Friend]:
        """遍历好友信息的迭代器。"""
    def find_friend(self, uin: int) -> Friend | None:
        """查找好友。"""
    def friend_groups(self) -> list[FriendGroup]:
        """获取所有好友分组信息。"""
    def find_friend_group(self, group_id: int) -> FriendGroup | None:
        """查找好友分组。"""

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
        """最后一条消息的 seq。

        只有通过 `get_group` 或 `get_groups` 获取的群才有此字段。
        """

################################################################################
# client/message_receipt.rs

class MessageReceipt:
    def msg_time(self) -> int: ...
    def seqs(self) -> list[int]: ...
    def rands(self) -> list[int]: ...
    async def recall(self) -> None:
        """撤回消息。"""

################################################################################
# message/elements.rs

class Text(TypedDict):
    type: Literal["text"]
    text: str

class At(TypedDict):
    type: Literal["at"]
    target: int

Element = str | Text | At

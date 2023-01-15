//! 常量定义。
//!
//! 常量可以被对应的环境变量覆盖，环境变量名为 `AWR_` 加上常量名，如 `AWR_FRIEND_LIST_CACHE_TIME`。

use konst::{option, primitive::parse_u64, unwrap_ctx};
use std::time::Duration;

/// 好友列表缓存时间，单位秒，默认 3600 秒。
pub const FRIEND_LIST_CACHE_TIME: Duration = Duration::from_secs(unwrap_ctx!(parse_u64(
    option::unwrap_or!(option_env!("AWR_FRIEND_LIST_CACHE_TIME"), "3600")
)));

/// 群信息缓存时间，单位秒，默认 3600 秒。
pub const GROUP_CACHE_TIME: Duration = Duration::from_secs(unwrap_ctx!(parse_u64(
    option::unwrap_or!(option_env!("AWR_GROUP_CACHE_TIME"), "3600")
)));

/// 群成员列表缓存时间，单位秒，默认 3600 秒。
pub const GROUP_MEMBER_LIST_CACHE_TIME: Duration = Duration::from_secs(unwrap_ctx!(parse_u64(
    option::unwrap_or!(option_env!("AWR_GROUP_MEMBER_LIST_CACHE_TIME"), "3600")
)));

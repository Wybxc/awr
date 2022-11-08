<div align="center">

# AWR

![Build Status](https://img.shields.io/github/workflow/status/Wybxc/awr/CI)
[![GitHub Issues](https://img.shields.io/github/issues/Wybxc/awr)](https://github.com/Wybxc/awr/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/Wybxc/awr)](https://github.com/Wybxc/awr/pulls)

![LICENCE](https://img.shields.io/github/license/Wybxc/awr)
[![Docs Status](https://img.shields.io/github/workflow/status/Wybxc/awr/API%20Document?label=docs)](https://awr-latest.netlify.app)
[![GitHub](https://img.shields.io/github/last-commit/Wybxc/awr?label=GitHub)](https://github.com/Wybxc/awr)

</div>

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

## 已完成功能/开发计划

<details>
<summary>点击展开</summary>

### 登录

- [x] 账号密码登录
- [x] 二维码登录
- [x] 验证码提交
- [x] 设备锁验证
- [x] 错误信息解析

### 消息类型

- [x] 文本
- [x] 表情
- [x] At
- [ ] 回复
- [ ] 匿名
- [ ] 骰子
- [ ] 石头剪刀布
- [ ] 图片
- [ ] 语音
- [ ] 长消息(仅支持群聊发送)
- [ ] 合并转发(仅支持群聊发送)
- [ ] 链接分享
- [ ] 小程序(暂只支持RAW)
- [ ] 短视频
- [ ] 群文件(上传与接收信息)

### 事件

- [ ] 群消息
- [ ] 好友消息
- [ ] 新好友请求
- [ ] 收到其他用户进群请求
- [ ] 新好友
- [ ] 群禁言
- [ ] 好友消息撤回
- [ ] 群消息撤回
- [ ] 收到邀请进群请求
- [ ] 群名称变更
- [ ] 好友删除
- [ ] 群成员权限变更
- [ ] 新成员进群/退群
- [ ] 登录号加群
- [ ] 临时会话消息
- [ ] 群解散
- [ ] 登录号退群(包含T出)
- [ ] 客户端离线
- [ ] 群提示 (戳一戳/运气王等)

### 主动操作

> 为防止滥用，将不支持主动邀请新成员进群

- [ ] 修改昵称
- [ ] 发送群消息
- [x] 获取群列表
- [x] 获取群成员列表
- [x] 获取好友列表/分组
- [ ] 获取好友个性签名
- [ ] 添加/删除/重命名好友分组
- [ ] 群成员禁言/解除禁言
- [ ] 踢出群成员
- [ ] 戳一戳群友
- [x] 戳一戳好友
- [ ] 设置群管理员
- [ ] 设置群公告
- [ ] 设置群名称
- [ ] 全员禁言
- [ ] 获取群@全体剩余次数
- [ ] 翻译
- [ ] 修改群成员头衔
- [ ] 设置群精华消息
- [x] 发送好友消息
- [ ] 发送临时会话消息
- [ ] 修改群成员Card
- [ ] 撤回群消息
- [x] 撤回好友消息
- [ ] 处理被邀请加群请求
- [ ] 处理加群请求
- [ ] 处理好友请求
- [ ] 删除好友
- [ ] 获取陌生人信息
- [ ] 设置在线状态
- [ ] 修改个人资料
- [ ] 修改个性签名
- [ ] 获取群荣誉 (龙王/群聊火焰等)
- [ ] 获取群文件下载链接
- [ ] ~~群成员邀请~~

</details>

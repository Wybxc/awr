import os
import awr
import asyncio

account = os.environ.get("QQ_ACCOUNT")
if not account:
    print("QQ_ACCOUNT not set")
    exit(1)

async def main():
    client = await awr.Dynamic().login(int(account), "./bots")
    
    for friend in await client.get_friends():
        print(friend)

    await client.alive()
    
try:
    asyncio.run(main())
except KeyboardInterrupt:
    import sys
    sys.exit(0)

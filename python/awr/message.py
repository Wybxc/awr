from typing import TypedDict

from typing_extensions import Literal


class Text(TypedDict):
    type: Literal["text"]
    text: str

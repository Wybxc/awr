from typing import TypedDict

from typing_extensions import Literal


class Text(TypedDict):
    type: Literal["text"]
    text: str


def text(text: str) -> Text:
    return {"type": "text", "text": text}


class At(TypedDict):
    type: Literal["at"]
    target: int


def at(target: int) -> At:
    return {"type": "at", "target": target}


class Face(TypedDict):
    type: Literal["face"]
    id: int | None
    name: str | None


def face(id_or_name: int | str) -> Face:
    if isinstance(id_or_name, int):
        return {"type": "face", "id": id_or_name, "name": None}
    else:
        return {"type": "face", "id": None, "name": id_or_name}


Element = str | Text | At | Face

from pydantic import BaseModel, HttpUrl
from typing import List, Optional


class Tile(BaseModel):
    id: str
    title: str
    subtitle: Optional[str] = None
    image_url: HttpUrl
    badge: Optional[str] = None


class Carousel(BaseModel):
    id: str
    title: str
    tiles: List[Tile]


class Page(BaseModel):
    title: str
    carousels: List[Carousel]

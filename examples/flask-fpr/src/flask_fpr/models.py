from flask import render_template
from markupsafe import Markup
from pydantic import BaseModel


class Tile(BaseModel):
    id: str
    title: str
    subtitle: str | None = None
    image_url: str
    badge: str | None = None


class Carousel(BaseModel):
    id: str
    title: str
    tiles: list[Tile]


class Page(BaseModel):
    carousels: list[Carousel]


def render_tile(tile: Tile) -> Markup:
    """Render a single tile using tile.html."""
    html = render_template("tile.html", tile=tile)
    return Markup(html)


def render_carousel(carousel: Carousel) -> Markup:
    """Render a carousel using carousel.html."""
    html = render_template("carousel.html", carousel=carousel)
    return Markup(html)


def render_page(page: Page) -> Markup:
    """Render a page using page.html."""
    html = render_template("page.html", page=page)
    return Markup(html)

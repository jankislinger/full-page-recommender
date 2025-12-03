import pytest
from flask import Flask

from flask_fpr.models import (
    Carousel,
    Page,
    Tile,
    render_carousel,
    render_page,
    render_tile,
)


def test_page(dummy_app: Flask):
    tiles = [dummy_tile(i) for i in range(4)]
    carousel = Carousel(id="carousel-foo", title="Carousel Title", tiles=tiles)
    page = Page(carousels=[carousel])

    with dummy_app.app_context():
        html = render_page(page)

    assert isinstance(html, str)
    assert page.carousels[0].title in html
    assert page.carousels[0].tiles[0].title in html


def test_carousel(dummy_app: Flask):
    tiles = [dummy_tile(i) for i in range(4)]
    carousel = Carousel(id="carousel-foo", title="Carousel Title", tiles=tiles)

    with dummy_app.app_context():
        html = render_carousel(carousel)

    assert isinstance(html, str)
    assert carousel.id in html
    assert carousel.title in html
    assert carousel.tiles[0].title in html


def test_tile(dummy_app: Flask):
    tile = Tile(id="foo", title="Movie Title", image_url="/foo.png", badge="New")

    with dummy_app.app_context():
        html = render_tile(tile)

    assert isinstance(html, str)
    assert tile.id in html
    assert tile.title in html
    assert tile.badge in html
    assert tile.image_url in html


def dummy_tile(i: int) -> Tile:
    return Tile(
        id=f"tile-{i:03d}",
        title=f"Movie {i}",
        image_url=f"/img_{i:03d}.png",
        badge="New",
    )


@pytest.fixture
def dummy_app() -> Flask:
    app = Flask(__name__, template_folder="../templates")

    app.add_template_filter(render_page, name="render_page")
    app.add_template_filter(render_carousel, name="render_carousel")
    app.add_template_filter(render_tile, name="render_tile")

    return app

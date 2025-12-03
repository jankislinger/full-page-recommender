from __future__ import annotations

from flask import Flask, render_template

from flask_fpr.models import (
    Carousel,
    Page,
    Tile,
    render_carousel,
    render_page,
    render_tile,
)

app = Flask(__name__)


# -----------------------------
# Render helpers (Jinja filters)
# -----------------------------


# Register filters so you can do {{ tile|render_tile }} etc.
app.add_template_filter(render_tile, name="render_tile")
app.add_template_filter(render_carousel, name="render_carousel")
app.add_template_filter(render_page, name="render_page")


# -----------------------------
# Demo data
# -----------------------------


def build_demo_page() -> Page:
    # In real app, this would come from DB / recommender / API
    hero_tile = Tile(
        id="hero-1",
        title="Featured Show: Night City Stories",
        subtitle="New episodes weekly",
        image_url="https://images.pexels.com/photos/799137/pexels-photo-799137.jpeg",
        badge="Exclusive",
    )

    continue_tiles = [
        Tile(
            id="cont-1",
            title="Space Rangers",
            subtitle="S1 · E4",
            image_url="https://images.pexels.com/photos/799443/pexels-photo-799443.jpeg",
        ),
        Tile(
            id="cont-2",
            title="Cooking Chaos",
            subtitle="S2 · E9",
            image_url="https://images.pexels.com/photos/765835/pexels-photo-765835.jpeg",
        ),
    ]

    trending_tiles = [
        Tile(
            id="trend-1",
            title="Cyber Heist",
            subtitle="Movie · 2h 10m",
            image_url="https://images.pexels.com/photos/799443/pexels-photo-799443.jpeg",
            badge="Top 10",
            is_hd=True,
        ),
        Tile(
            id="trend-2",
            title="Love in Prague",
            subtitle="Movie · 1h 45m",
            image_url="https://images.pexels.com/photos/799443/pexels-photo-799443.jpeg",
            badge="Popular",
        ),
        Tile(
            id="trend-3",
            title="Goal Rush",
            subtitle="Sports · Highlights",
            image_url="https://images.pexels.com/photos/799443/pexels-photo-799443.jpeg",
            is_hd=True,
        ),
    ]

    long_tiles = [
        Tile(
            id="trend-1",
            title="Cyber Heist",
            subtitle="Movie · 2h 10m",
            image_url="https://images.pexels.com/photos/799443/pexels-photo-799443.jpeg",
            badge=f"Top {i}",
            is_hd=True,
        )
        for i in range(24)
    ]

    carousels = [
        Carousel(id="continue-watching", title="Continue Watching", tiles=continue_tiles),
        Carousel(id="trending-now", title="Trending Now", tiles=trending_tiles),
        Carousel(id="long-tiles", title="Scrollable", tiles=long_tiles),
    ]

    return Page(title="MyStream", hero=hero_tile, carousels=carousels)


# -----------------------------
# Routes
# -----------------------------


@app.route("/", methods=["GET"])
def index():
    page = build_demo_page()
    return render_template("page.html", page=page)


if __name__ == "__main__":
    app.run(debug=True)

from __future__ import annotations

from collections.abc import Iterator

from flask import Flask, render_template, url_for

from flask_fpr.ease_engine import EaseEngine
from flask_fpr.engine import NUM_TILES, build_page
from flask_fpr.models import (
    Carousel,
    Page,
    render_carousel,
    render_page,
    render_tile,
)

app = Flask(__name__)
# _engine = DummyEngine()
engine = EaseEngine()


# -----------------------------
# Routes
# -----------------------------


@app.route("/user/<string:user_id>", methods=["GET"])
@app.route("/", methods=["GET"], defaults={"user_id": 200948})
def index(user_id: str):
    page = build_page(engine, user_id)
    return render_template("page.html", page=page)


@app.route("/select-user", methods=["GET"])
def users():
    import polars as pl

    sampled_users = engine.interactions["user_id"].sample(10, shuffle=True)

    data: pl.DataFrame = (
        engine.interactions.lazy()
        .filter(pl.col("user_id").is_in(sampled_users))
        .join(engine.items.lazy(), on="item_id")
        .group_by("user_id")
        .agg(pl.col("index").sample(fraction=1.0, shuffle=True))
        .with_columns(pl.col("index").list.head(3 * NUM_TILES))
        .collect()
    )
    page = users_history_page(data.iter_rows(named=False))
    return render_template("page.html", page=page)


# -----------------------------
# Render helpers (Jinja filters)
# -----------------------------


app.add_template_filter(render_tile, name="render_tile")
app.add_template_filter(render_carousel, name="render_carousel")
app.add_template_filter(render_page, name="render_page")


# -----------------------------
# Custom page rendering
# -----------------------------


def users_history_page(inputs: Iterator[tuple[int, list[int]]]) -> Page:
    carousels = []

    for user_id, item_indices in inputs:
        tiles = [engine.get_tile(i) for i in item_indices]
        c = Carousel(
            id=f"user-history-{user_id}",
            title=f"User {user_id}",
            tiles=tiles,
            link_text="Select user",
            link_url=url_for("index", user_id=user_id),
        )
        carousels.append(c)

    return Page(carousels=carousels)


if __name__ == "__main__":
    app.run(debug=True)

# Full Page Recommender

This algorithm serves personalized pages with a list of collections and
their corresponding items instead of a single set of items. It highlights
the most relevant collections at the top and includes soft de-duplication to
showcase the entire catalog, encouraging users to explore beyond their usual
choices.

A collection is a group of scored items with a label for display.
Collections can be sorted to keep the item order fixed, which is useful for
static lists like _"Top 10 Books This Week."_

An item's score within a collection reflects its relevance to the user and
the collection. For example, [_"Zombieland"_][zombieland-wiki] may score
higher in the _"Comedy Movies"_ collection than in _"Post-Apocalyptic Movies"_
since post-apocalyptic viewers may not be interested in it.

Item scores can originate from a single recommendation algorithm, but this
is not mandatory. For instance, one might use a model based on item metadata
for collections with cold items, like _"Just Released,"_ while employing a
rating-based model for other collections. The FPR algorithm does not provide
a model, but I plan to add [EASE][ease-arxiv] in the future. Additionally, I
aim to implement a solution to automatically generate collections and their
labels using Node2Vec embedding and nested clustering.

De-duplication uses item temperatures. When an item is recommended in a
collection, its temperature increases, reducing its score in other
collections for future rows. This allows the item to appear in multiple rows
but keeps them spaced out.

## Examples

Get the repo locally:

```shell
git clone git@github.com:jankislinger/full-page-recommendations.git
cd full-page-recommendations
```

Run example in Python:

```shell
poetry shell
poetry install --no-root
maturin develop
python main.py
```

Run example blazingly fast in Python:

```shell
maturin develop --release
python main.py
```

[ease-arxiv]: https://arxiv.org/abs/1905.03375
[zombieland-wiki]: https://en.wikipedia.org/wiki/Zombieland

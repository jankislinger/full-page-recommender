function scrollCarousel(id, direction, evt) {
  // If called from a button, respect disabled state
  if (evt && evt.currentTarget && evt.currentTarget.disabled) {
    return;
  }

  const scroller = document.getElementById("carousel-" + id);
  if (!scroller) return;

  const firstTile = scroller.querySelector("div");
  if (!firstTile) return;

  const tileWidth = firstTile.offsetWidth;
  const gap = parseInt(getComputedStyle(scroller).columnGap || 16, 10);
  const totalTileWidth = tileWidth + gap;

  const visibleTiles = Math.floor(scroller.clientWidth / totalTileWidth) || 1;
  const scrollAmount = visibleTiles * totalTileWidth * direction;

  scroller.scrollBy({ left: scrollAmount, behavior: "smooth" });
}

function setArrowDisabled(btn, disabled) {
  if (!btn) return;
  btn.disabled = disabled;
}

function updateCarouselButtons(id) {
  const scroller = document.getElementById("carousel-" + id);
  const leftBtn  = document.getElementById("carousel-left-" + id);
  const rightBtn = document.getElementById("carousel-right-" + id);

  if (!scroller || !leftBtn || !rightBtn) return;

  const maxScroll = Math.max(scroller.scrollWidth - scroller.clientWidth, 0);

  // Nothing to scroll → keep arrows but disable both
  if (maxScroll === 0) {
    setArrowDisabled(leftBtn, true);
    setArrowDisabled(rightBtn, true);
    return;
  }

  const atStart = scroller.scrollLeft <= 1;
  const atEnd   = scroller.scrollLeft >= maxScroll - 1;

  setArrowDisabled(leftBtn, atStart);
  setArrowDisabled(rightBtn, atEnd);
}

function initCarousels() {
  document.querySelectorAll(".carousel-scroller").forEach(scroller => {
    const id = scroller.dataset.carouselId;
    if (!id) return;

    // Initial state
    updateCarouselButtons(id);

    // Keep arrows in sync while scrolling
    scroller.addEventListener("scroll", () => updateCarouselButtons(id));
  });
}

window.addEventListener("load", initCarousels);
window.addEventListener("resize", initCarousels);

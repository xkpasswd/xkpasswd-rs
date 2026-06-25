/**
 * Popover positioning utilities.
 *
 * Pure functions — no DOM access — so they can be unit-tested in isolation.
 */

/**
 * Clamp a popover's preferred left coordinate so the menu stays within the
 * viewport with a uniform gutter.  Prefers the requested left, shifts left to
 * fit when it would overflow the right edge, and pins to the left gutter when
 * the menu is too wide to fit at all.
 *
 * @param preferredLeft - The ideal `left` value (px), typically `rect.left`.
 * @param menuWidth     - Rendered width of the menu (px), e.g. `offsetWidth`.
 * @param viewportWidth - Viewport width (px), typically `window.innerWidth`.
 * @param margin        - Minimum gap (px) between menu edges and viewport edges. Default 8.
 * @returns A `left` value (px) guaranteed to keep the menu within the viewport.
 */
export const clampPopoverLeft = (
  preferredLeft: number,
  menuWidth: number,
  viewportWidth: number,
  margin = 8,
): number => {
  const maxLeft = viewportWidth - margin - menuWidth;
  // When the menu is wider than the usable space, pin to the left gutter so
  // at least the left portion (including any scrollbar) is reachable.
  if (maxLeft < margin) return margin;
  return Math.min(Math.max(preferredLeft, margin), maxLeft);
};

import { describe, it, expect } from 'vitest';
import { clampPopoverLeft } from './positioning';

// ── clampPopoverLeft ──────────────────────────────────────────────────────────

describe('clampPopoverLeft', () => {
  // ── Preferred within bounds ────────────────────────────────────────────────

  it('returns preferredLeft unchanged when the menu fits without clipping', () => {
    // menu spans 130..450, well within 1400px viewport
    expect(clampPopoverLeft(130, 320, 1400)).toBe(130);
  });

  it('returns preferredLeft unchanged when menu exactly fits at gutter', () => {
    // menu spans 8..328, touching the left gutter exactly
    expect(clampPopoverLeft(8, 320, 1400)).toBe(8);
  });

  it('returns preferredLeft when menu fits exactly at the right gutter', () => {
    // menu spans 1072..1392, right edge = 1400 - 8 = 1392 ✓
    expect(clampPopoverLeft(1072, 320, 1400)).toBe(1072);
  });

  // ── Overflows right edge ──────────────────────────────────────────────────

  it('shifts left when menu would overflow the right edge', () => {
    // menu at 1200: 1200+320=1520 > 1400-8=1392 → clamped to 1400-8-320=1072
    expect(clampPopoverLeft(1200, 320, 1400)).toBe(1072);
  });

  it('shifts left even when preferredLeft is only slightly past maxLeft', () => {
    // maxLeft = 1400-8-320 = 1072; preferred = 1073
    expect(clampPopoverLeft(1073, 320, 1400)).toBe(1072);
  });

  // ── Overflows left edge (preferred < margin) ──────────────────────────────

  it('pins to margin when preferredLeft is negative', () => {
    expect(clampPopoverLeft(-40, 272, 390)).toBe(8);
  });

  it('pins to margin when preferredLeft is less than margin', () => {
    expect(clampPopoverLeft(3, 272, 390)).toBe(8);
  });

  // ── Menu wider than viewport (minus gutters) ──────────────────────────────

  it('pins to margin when menu is wider than viewport minus gutters', () => {
    // 800px menu in 390px viewport: maxLeft = 390-8-800 < 8 → pin to 8
    expect(clampPopoverLeft(0, 800, 390)).toBe(8);
  });

  it('pins to margin even when preferredLeft > margin in a too-narrow viewport', () => {
    expect(clampPopoverLeft(50, 800, 390)).toBe(8);
  });

  // ── Realistic mobile: bug fix verification ────────────────────────────────

  it('does NOT produce a negative left on mobile (the original bug scenario)', () => {
    // 272px menu, preferredLeft=30, 390px viewport → spans 30..302, fits fine
    const result = clampPopoverLeft(30, 272, 390);
    expect(result).toBe(30);
    expect(result).toBeGreaterThanOrEqual(8); // never off-screen-left
  });

  it('clamps a menu that starts too far right on mobile', () => {
    // preferredLeft=200, 272px menu, 390px viewport → maxLeft=390-8-272=110
    expect(clampPopoverLeft(200, 272, 390)).toBe(110);
  });

  // ── Custom margin ─────────────────────────────────────────────────────────

  it('respects a custom margin=0 (edge-to-edge)', () => {
    // maxLeft = 1400-0-320 = 1080; preferred=1200 → 1080
    expect(clampPopoverLeft(1200, 320, 1400, 0)).toBe(1080);
  });

  it('respects a custom margin=16', () => {
    // maxLeft = 1400-16-320 = 1064; preferred=1200 → 1064
    expect(clampPopoverLeft(1200, 320, 1400, 16)).toBe(1064);
  });

  it('pins to custom margin when preferred is left of it', () => {
    // margin=16; preferred=5 < 16 → 16
    expect(clampPopoverLeft(5, 272, 390, 16)).toBe(16);
  });
});

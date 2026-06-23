/**
 * Header → macOS-style titlebar (§5.2).
 *
 * Layout (left → right):
 *  .dots   — three 12 px traffic-light circles (aria-hidden)
 *  .path   — ~/xkpasswd-rs[@<commitHash>] path label; linked to repo
 *  .brand  — pushed right: inlined glyph SVG + "xkpasswd" wordmark
 *
 * The path link wraps both the project name and the commit hash (when
 * VITE_GIT_SHA is set at build time). Hovering turns both green.
 * If VITE_GIT_SHA is unset, only ~/xkpasswd-rs is shown (still linked).
 *
 * The glyph is inlined (not <img>) so the green caret rect can receive the
 * `animate-blink` Tailwind class. `motion-reduce:animate-none` respects
 * prefers-reduced-motion.
 */
import packageInfo from 'package.json';
import './styles.css';

const Header = () => {
  const commitHash = import.meta.env.VITE_GIT_SHA;
  const repoUrl = packageInfo.repository.url.replace(/\.git$/i, '');

  return (
    <header className="titlebar">
      {/* Traffic-light dots — decorative, not interactive */}
      <span className="dots" aria-hidden="true">
        <span className="dot bg-red" />
        <span className="dot bg-yellow" />
        <span className="dot bg-green" />
      </span>

      {/* ~/xkpasswd-rs[@<hash>] path label — linked to repo */}
      <span className="path">
        {'~/'}
        <a className="path-link" href={repoUrl}>
          <b>{'xkpasswd-rs'}</b>
          {commitHash && <span className="path-hash">{'@'}{commitHash}</span>}
        </a>
      </span>

      {/* Brand: glyph (inlined) + wordmark — pushed to right edge */}
      <span className="brand">
        {/*
         * Inlined logo-glyph.svg so the caret <rect> is a real DOM element
         * that can receive animate-blink. aria-hidden because the wordmark
         * text ("xkpasswd") already conveys the name.
         */}
        <svg
          width="40"
          height="16"
          viewBox="13 22 40 20"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
          aria-hidden="true"
        >
          <circle cx="18" cy="32" r="3" fill="#9aedfe" />
          <circle cx="27" cy="32" r="3" fill="#ff6ac1" />
          <circle cx="36" cy="32" r="3" fill="#f3f99d" />
          {/* Blinking caret — uses the `blink` keyframe defined in tailwind.config.cjs */}
          <rect
            x="43"
            y="24"
            width="7"
            height="16"
            rx="1.5"
            fill="#5af78e"
            className="animate-blink motion-reduce:animate-none"
          />
        </svg>
        <span className="wordmark">{'xkpasswd'}</span>
      </span>
    </header>
  );
};

export default Header;

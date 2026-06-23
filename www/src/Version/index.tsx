/**
 * Version → footer `#`-comment (§5.6).
 *
 * Rendered only when VITE_GIT_SHA is set at build time.
 * Format: `# Built with xkpasswd-rs@<sha>`
 *   - Capital "B" in "Built" (intentional, per spec)
 *   - xkpasswd-rs links to the repo; @<sha> links to the specific commit
 *   - `faint` colour, top hairline border
 */
import packageInfo from 'package.json';
import './styles.css';

const Version = () => {
  const commitHash = import.meta.env.VITE_GIT_SHA;

  if (!commitHash) {
    return null;
  }

  const repoUrl = packageInfo.repository.url.replace(/\.git$/i, '');

  return (
    <p className="foot">
      {'# Built with '}
      <a className="foot-link" href={repoUrl}>
        {'xkpasswd-rs'}
      </a>
      {'@'}
      <a className="foot-link" href={`${repoUrl}/commit/${commitHash}`}>
        {commitHash}
      </a>
    </p>
  );
};

export default Version;

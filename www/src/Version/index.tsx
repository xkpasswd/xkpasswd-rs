import packageInfo from '../../package.json';
import './styles.css';

const Version = () => {
  const commitHash = import.meta.env.VITE_GIT_SHA;

  if (!commitHash) {
    return null;
  }

  const repoUrl = packageInfo.repository.url.replace(/\.git$/i, '');

  return (
    <div className="section version-container">
      <span>
        {'Built with '}
        <a href={repoUrl} className="version-link repo">
          {'xkpasswd-rs'}
        </a>
        <a className="version-link" href={`${repoUrl}/commit/${commitHash}`}>
          {commitHash}
        </a>
      </span>
    </div>
  );
};

export default Version;

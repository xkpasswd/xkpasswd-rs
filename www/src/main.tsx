import { render } from 'preact';
import App from './app';
import './wasm';
import '@fontsource/source-code-pro/400.css';
import '@fontsource/source-code-pro/500.css';
import '@fontsource/source-code-pro/600.css';
import '@fontsource/source-code-pro/700.css';
import './main.css';

render(<App />, document.getElementById('app') as HTMLElement);

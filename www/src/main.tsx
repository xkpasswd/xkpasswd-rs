import { render } from 'preact';
import App from './app';
import './wasm';
import './main.css';

render(<App />, document.getElementById('app') as HTMLElement);

import { render } from 'preact';
import App from './app';
import './main.css';
import initWasm from '../xkpasswd/xkpasswd';

await initWasm();
render(<App />, document.getElementById('app') as HTMLElement);

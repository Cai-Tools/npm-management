/* @refresh reload */
import { render } from 'solid-js/web'
import '@nordstjerna/style';
import '@nordstjerna/ui/style.css';

import App from './App'

const root = document.getElementById('root')

render(() => <App />, root!)

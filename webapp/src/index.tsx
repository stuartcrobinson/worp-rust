import * as React from 'react'
import * as ReactDOM from 'react-dom'
import * as serviceWorker from './serviceWorker'

import {ReduxRoot} from './ReduxRoot'

const rootEl = document.getElementById('root')
ReactDOM.render(
  <React.StrictMode>
    <ReduxRoot />
  </React.StrictMode>,
  rootEl,
)

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister()

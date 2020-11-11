import * as React from 'react'

import App from './App'
import {PersistGate} from 'redux-persist/integration/react'
import {Provider} from 'react-redux'
import Typography from '@material-ui/core/Typography'
import configureStore from './configureStore'

const {persistor, store} = configureStore()

export function ReduxRoot() {
  return (
    <Provider store={store}>
      <PersistGate
        loading={<Typography>Loading...</Typography>}
        persistor={persistor}
      >
        <App />
      </PersistGate>
    </Provider>
  )
}

import * as React from 'react'

import CssBaseline from '@material-ui/core/CssBaseline'
import ThemeProvider from '@material-ui/styles/ThemeProvider'
import createMuiTheme from '@material-ui/core/styles/createMuiTheme'

// A theme with custom primary and secondary color.
// It's optional.
const theme = createMuiTheme({
  palette: {
    primary: {
      light: '#e5e5e5',
      main: '#727272',
      dark: '#363839',
      contrastText: '#fff',
    },
    secondary: {
      light: '#ff5e50',
      main: '#e41e26',
      dark: '#a90000',
      contrastText: '#fff',
    },
  },
})

export function withRoot(Component: any) {
  function WithRoot(props: object) {
    // MuiThemeProvider makes the theme available down the React tree
    // thanks to React context.
    return (
      <ThemeProvider theme={theme}>
        {/* Reboot kickstart an elegant, consistent, and simple baseline to build upon. */}
        <CssBaseline />
        <Component {...props} />
      </ThemeProvider>
    )
  }

  return WithRoot
}

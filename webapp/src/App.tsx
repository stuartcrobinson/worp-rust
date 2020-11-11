import * as React from 'react'

import {Route, Router} from 'react-router-dom'

// prettier-ignore
import AppBar from '@material-ui/core/AppBar'
import Badge from '@material-ui/core/Badge'
import Divider from '@material-ui/core/Divider'
import DrawerMui from '@material-ui/core/Drawer'
import FormatListNumberedIcon from '@material-ui/icons/FormatListNumbered'
import Hidden from '@material-ui/core/Hidden'
import HomeIcon from '@material-ui/icons/Home'
import {HomePage} from './pages/HomePage'
import IconButton from '@material-ui/core/IconButton'
import List from '@material-ui/core/List'
import ListItem from '@material-ui/core/ListItem'
import ListItemIcon from '@material-ui/core/ListItemIcon'
import ListItemText from '@material-ui/core/ListItemText'
import MenuIcon from '@material-ui/icons/Menu'
import {QueryPage} from './pages/QueryPage'
import {RootState} from './reducers'
import SearchIcon from '@material-ui/icons/Search'
import {Theme} from '@material-ui/core/styles/createMuiTheme'
import {Todo} from './model/todo'
import {TodoPage} from './pages/TodoPage'
import Toolbar from '@material-ui/core/Toolbar'
import Typography from '@material-ui/core/Typography'
import {history} from './configureStore'
import makeStyles from '@material-ui/styles/makeStyles'
import useMediaQuery from '@material-ui/core/useMediaQuery'
import {useSelector} from 'react-redux'
import {withRoot} from './withRoot'

function Routes() {
  const classes = useStyles()

  return (
    <div className={classes.content}>
      <Route exact={true} path="/" component={HomePage} />
      <Route exact={true} path="/home" component={HomePage} />
      <Route exact={true} path="/query" component={QueryPage} />
      <Route exact={true} path="/todo" component={TodoPage} />
    </div>
  )
}

function Drawer(props: {todoList: Todo[]}) {
  const classes = useStyles()

  return (
    <div>
      <div className={classes.drawerHeader} />
      <Divider />
      <List>
        <ListItem button onClick={() => history.push('/')}>
          <ListItemIcon>
            <HomeIcon />
          </ListItemIcon>
          <ListItemText primary="Home" />
        </ListItem>
      </List>
      <Divider />
      <List>
        <ListItem button onClick={() => history.push('/query')}>
          <ListItemIcon>
            <SearchIcon />
          </ListItemIcon>
          <ListItemText primary="Query" />
        </ListItem>
      </List>
      <Divider />
      <List>
        <ListItem button onClick={() => history.push('/todo')}>
          <ListItemIcon>
            <TodoIcon todoList={props.todoList} />
          </ListItemIcon>
          <ListItemText primary="Todo" />
        </ListItem>
      </List>
    </div>
  )
}

function App() {
  const classes = useStyles()
  const [mobileOpen, setMobileOpen] = React.useState(true)
  const todoList = useSelector((state: RootState) => state.todoList)
  const isMobile = useMediaQuery((theme: Theme) =>
    theme.breakpoints.down('sm')
  )

  const handleDrawerToggle = () => {
    setMobileOpen(!mobileOpen)
  }

  return (
    <Router history={history}>
      <div className={classes.root}>
        <div className={classes.appFrame}>
          <AppBar className={classes.appBar}>
            <Toolbar>
              <IconButton
                color="inherit"
                aria-label="open drawer"
                onClick={handleDrawerToggle}
                className={classes.navIconHide}
              >
                <MenuIcon />
              </IconButton>
              <Typography
                variant="h6"
                color="inherit"
                noWrap={isMobile}
              >
                Worp Dashboard
              </Typography>
            </Toolbar>
          </AppBar>
          <Hidden mdUp>
            <DrawerMui
              variant="temporary"
              anchor={'left'}
              open={mobileOpen}
              classes={{
                paper: classes.drawerPaper,
              }}
              onClose={handleDrawerToggle}
              ModalProps={{
                keepMounted: true, // Better open performance on mobile.
              }}
            >
              <Drawer todoList={todoList} />
            </DrawerMui>
          </Hidden>
          <Hidden smDown>
            <DrawerMui
              variant="permanent"
              open
              classes={{
                paper: classes.drawerPaper,
              }}
            >
              <Drawer todoList={todoList} />
            </DrawerMui>
          </Hidden>
          <Routes />
        </div>
      </div>
    </Router>
  )
}

function TodoIcon(props: {todoList: Todo[]}) {
  let uncompletedTodos = props.todoList.filter(t => t.completed === false)

  if (uncompletedTodos.length > 0) {
    return (
      <Badge color="secondary" badgeContent={uncompletedTodos.length}>
        <FormatListNumberedIcon />
      </Badge>
    )
  } else {
    return <FormatListNumberedIcon />
  }
}

const drawerWidth = 240
const useStyles = makeStyles((theme: Theme) => ({
  root: {
    width: '100%',
    height: '100%',
    zIndex: 1,
    overflow: 'hidden',
  },
  appFrame: {
    position: 'relative',
    display: 'flex',
    width: '100%',
    height: '100%',
  },
  appBar: {
    zIndex: theme.zIndex.drawer + 1,
    position: 'absolute',
  },
  navIconHide: {
    [theme.breakpoints.up('md')]: {
      display: 'none',
    },
  },
  drawerHeader: {...theme.mixins.toolbar},
  drawerPaper: {
    width: 250,
    backgroundColor: theme.palette.background.default,
    [theme.breakpoints.up('md')]: {
      width: drawerWidth,
      position: 'relative',
      height: '100%',
    },
  },
  content: {
    backgroundColor: theme.palette.background.default,
    width: '100%',
    height: 'calc(100% - 56px)',
    marginTop: 56,
    [theme.breakpoints.up('sm')]: {
      height: 'calc(100% - 64px)',
      marginTop: 64,
    },
  },
}))

export default withRoot(App)

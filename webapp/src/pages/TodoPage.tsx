import * as React from 'react'

import Button from '@material-ui/core/Button'
import Grid from '@material-ui/core/Grid'
import {Theme} from '@material-ui/core/styles/createMuiTheme'
import {TodoDialog} from '../components/TodoDialog'
import {TodoTable} from '../components/TodoTable'
import Typography from '@material-ui/core/Typography'
import makeStyles from '@material-ui/styles/makeStyles'

export function TodoPage() {
  const classes = useStyles()
  const [open, setOpen] = React.useState(false)

  const handleClose = () => {
    setOpen(false)
  }

  const handleAddTodo = () => {
    setOpen(true)
  }

  return (
    <Grid container className={classes.root}>
      <TodoDialog open={open} onClose={handleClose} />
      <Grid item xs={6}>
        <Typography variant="h4" gutterBottom>
          Todo List
        </Typography>
      </Grid>
      <Grid item xs={6}>
        <div className={classes.buttonContainer}>
          <Button
            className={classes.button}
            variant="contained"
            color="secondary"
            onClick={handleAddTodo}
          >
            Add Todo
          </Button>
        </div>
      </Grid>
      <Grid item xs={12}>
        <TodoTable />
      </Grid>
    </Grid>
  )
}

const useStyles = makeStyles((theme: Theme) => ({
  root: {
    padding: 20,
    [theme.breakpoints.down('md')]: {
      paddingTop: 50,
      paddingLeft: 15,
      paddingRight: 15,
    },
  },

  buttonContainer: {
    width: '100%',
    display: 'flex',
    justifyContent: 'flex-end',
  },

  button: {
    marginBottom: 15,
  },
}))

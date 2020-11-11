import * as React from 'react'
import * as TodoActions from '../actions/todo'

// prettier-ignore
import Button from '@material-ui/core/Button'
import Dialog from '@material-ui/core/Dialog'
import DialogActions from '@material-ui/core/DialogActions'
import DialogTitle from '@material-ui/core/DialogTitle'
import TextField from '@material-ui/core/TextField'
import makeStyles from '@material-ui/styles/makeStyles'
import {useActions} from '../actions'

interface Props {
  open: boolean
  onClose: () => void
}

export function TodoDialog(props: Props) {
  const {open, onClose} = props
  const classes = useStyles()
  const [newTodoText, setNewTodoText] = React.useState('')
  const todoActions = useActions(TodoActions)

  const handleClose = () => {
    todoActions.addTodo({
      id: Math.random(),
      completed: false,
      text: newTodoText,
    })
    onClose()

    // reset todo text if user reopens the dialog
    setNewTodoText('')
  }

  const handleChange = (event: any) => {
    setNewTodoText(event.target.value)
  }

  return (
    <Dialog open={open} onClose={handleClose}>
      <DialogTitle>Add a new TODO</DialogTitle>
      <TextField
        id="multiline-flexible"
        multiline
        value={newTodoText}
        onChange={handleChange}
        className={classes.textField}
      />
      <DialogActions>
        <Button color="primary" onClick={handleClose}>
          OK
        </Button>
      </DialogActions>
    </Dialog>
  )
}

const useStyles = makeStyles({
  textField: {
    width: '80%',
    margin: 20,
  },
})

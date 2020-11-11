import * as React from 'react'
import * as TodoActions from '../actions/todo'

// prettier-ignore
import Checkbox from '@material-ui/core/Checkbox'
import DeleteIcon from '@material-ui/icons/Delete'
import IconButton from '@material-ui/core/IconButton'
import Paper from '@material-ui/core/Paper'
import {RootState} from '../reducers'
import Table from '@material-ui/core/Table'
import TableBody from '@material-ui/core/TableBody'
import TableCell from '@material-ui/core/TableCell'
import TableHead from '@material-ui/core/TableHead'
import TableRow from '@material-ui/core/TableRow'
import {Todo} from '../model/todo'
import makeStyles from '@material-ui/styles/makeStyles'
import {useActions} from '../actions'
import {useSelector} from 'react-redux'

export function TodoTable() {
  const classes = useStyles()
  const todoList = useSelector((state: RootState) => state.todoList)
  const todoActions = useActions(TodoActions)

  const onRowClick = (todo: Todo) => {
    if (todo.completed) {
      todoActions.uncompleteTodo(todo.id)
    } else {
      todoActions.completeTodo(todo.id)
    }
  }

  return (
    <Paper className={classes.paper}>
      <Table className={classes.table}>
        <TableHead>
          <TableRow>
            <TableCell padding="default">Completed</TableCell>
            <TableCell padding="default">Text</TableCell>
            <TableCell padding="default">Delete</TableCell>
          </TableRow>
        </TableHead>
        <TableBody>
          {todoList.map((n: Todo) => {
            return (
              <TableRow
                key={n.id}
                hover
                onClick={event => onRowClick(n)}
              >
                <TableCell padding="none">
                  <Checkbox checked={n.completed} />
                </TableCell>
                <TableCell padding="none">{n.text}</TableCell>
                <TableCell padding="none">
                  <IconButton
                    aria-label="Delete"
                    color="default"
                    onClick={() =>
                      todoActions.deleteTodo(n.id)
                    }
                  >
                    <DeleteIcon />
                  </IconButton>
                </TableCell>
              </TableRow>
            )
          })}
        </TableBody>
      </Table>
    </Paper>
  )
}

const useStyles = makeStyles({
  paper: {
    width: '100%',
    minWidth: 260,
    display: 'inline-block',
  },
  table: {
    width: '100%',
  },
})

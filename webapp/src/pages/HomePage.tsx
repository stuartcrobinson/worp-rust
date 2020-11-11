import * as React from 'react'

import Button from '@material-ui/core/Button'
import {HomeBox} from '../components/HomeBox'
import {RootState} from '../reducers'
import Typography from '@material-ui/core/Typography'
import makeStyles from '@material-ui/styles/makeStyles'
import {useSelector} from 'react-redux'

export function HomePage() {
  const classes = useStyles()
  const [boxColor, setBoxColor] = React.useState('red')
  const todoList = useSelector((state: RootState) => state.todoList)

  const onButtonClick = () =>
    setBoxColor(boxColor === 'red' ? 'blue' : 'red')

  return (
    <div className={classes.root}>
      <Typography variant="h4" gutterBottom>
        You have {todoList.length} TODOs in your list!
      </Typography>
      <div className={classes.centerContainer}>
        <HomeBox size={300} color={boxColor} />
        <Button
          className={classes.button}
          onClick={onButtonClick}
          variant="outlined"
          color="primary"
        >
          Change Color
        </Button>
      </div>
    </div>
  )
}

const useStyles = makeStyles({
  root: {
    height: '100%',
    textAlign: 'center',
    paddingTop: 20,
    paddingLeft: 15,
    paddingRight: 15,
  },

  centerContainer: {
    flex: 1,
    height: '90%',
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    flexDirection: 'column',
  },

  button: {
    marginTop: 20,
  },
})

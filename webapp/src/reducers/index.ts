import * as todoReducer from './todo'

import {History} from 'history'
import {Todo} from '../model/todo'
import {combineReducers} from 'redux'

export interface RootState {
  todoList: Todo[]
}

export default (history: History) =>
  combineReducers({
    ...todoReducer,
  })

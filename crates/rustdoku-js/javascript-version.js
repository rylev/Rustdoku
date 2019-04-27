function generateJs(n) {
  const b = full()
  remove(b, n)
  return b
}
function full() {
  const board = Array(81).fill(undefined)
  solve(board)
  return board
}

function clone(board) {
  return board.slice(0)
}

function remove(board, n) {
  for (let i = 0; i < n; i++) {
    let tries = 0
    while (true) {
      const index = Math.floor(Math.random() * 81)
      const original = board[index]
      if (original !== undefined) {
        const clone1 = board.slice(0)
        const clone2 = board.slice(0)
        clone1[index] = undefined
        clone2[index] = undefined
        const condition = [index, original]
        const [withCondition, steps1] = solve(clone1, condition)
        const [withoutCondition, steps2] = solve(clone2)
        if (!withCondition && withoutCondition) {
          board[index] = undefined
          break
        } else {
          tries++
          if (tries >= 10000) {
            throw toString(clone1)
          }
        }
      }
    }

  }

}
function solve(board, condition) {
  return check(board, 0, 0, condition)
}

function check(board, index, steps, condition) {
  if (index > 80) { return [true, steps] }

  const original = board[index]
  const possibleValues = getPossibleValues(board, index)

  while (true) {
    if (possibleValues.every(x => x === false)) { break }
    const testIndex = Math.floor(Math.random(1) * 9)
    const isPossible = possibleValues[testIndex]

    if (!isPossible) {
      continue
    }

    const value = testIndex + 1

    const is_forbidden = condition && condition[0] === index && condition[1] === value

    if (!is_forbidden) {
      board[index] = value;
      const [succeeded, newSteps] = check(board, index + 1, steps + 1, condition)
      steps = newSteps
      if (succeeded) {
        return [true, steps]
      }
    }

    possibleValues[testIndex] = false
  }

  board[index] = original
  return [false, steps]
}

function getPossibleValues(board, index) {
  const value = board[index]
  if (value) {
    const possibilities = Array(9).fill(false)
    possibilities[value - 1] = true
    return possibilities
  }

  const rowIndex = Math.floor(index / 9)
  const columnIndex = index % 9
  const squareIndex = (Math.floor(rowIndex / 3) * 3) + Math.floor(columnIndex / 3)

  const row = getRow(board, rowIndex)
  const column = getColumn(board, columnIndex)
  const square = getSquare(board, squareIndex)
  const possibilities = Array(9).fill(true)

  for (rowValue of row) {
    if (rowValue !== undefined) {
      possibilities[rowValue - 1] = false
    }
  }

  for (columnValue of column) {
    if (columnValue !== undefined) {
      possibilities[columnValue - 1] = false
    }
  }

  for (squareValue of square) {
    if (squareValue !== undefined) {
      possibilities[squareValue - 1] = false
    }
  }

  return possibilities
}

function getRow(board, rowIndex) {
  let start = rowIndex * 9
  let end = start + 9

  return board.slice(start, end)
}

function getColumn(board, columnIndex) {
  const values = Array(9)

  for (let i = 0; i < 9; i++) {
    const j = (i * 9) + columnIndex
    values[i] = board[j]
  }

  return values
}

function getSquare(board, squareIndex) {
  const verticalOffset = Math.floor(squareIndex / 3)
  const horizontalOffset = Math.floor(squareIndex % 3)

  const offset = (verticalOffset * 27) + (horizontalOffset * 3)

  const values = Array(9).fill(undefined)

  for (let level = 0; level < 3; level++) {
    const from = offset + (level * 9)
    const to = from + 3
    let i = 0
    for (val of board.slice(from, to)) {
      values[i + (level * 3)] = val
      i++
    }
  }

  return values
}

function toString(board) {
  let output = ""
  let i = 0;
  for (value of board) {
    if (value) {
      output += value
    } else {

      output += "*"
    }

    if (!((i + 1) % 9 == 0)) {
      output += " "
    } else {
      output += "\n"
    }
    i++
  }

  return output

}


function newBoard() {
  const board = Array(81).fill(undefined)

  // Hard
  board[1] = 2
  board[2] = 5
  board[7] = 6
  board[9] = 1
  board[15] = 7
  board[16] = 9
  board[18] = 4
  board[23] = 1
  board[29] = 9
  board[33] = 6
  board[34] = 8
  board[41] = 9
  board[43] = 5
  board[46] = 1
  board[48] = 7
  board[50] = 3
  board[54] = 8
  board[59] = 5
  board[66] = 8
  board[67] = 7
  board[69] = 5
  board[71] = 2
  board[74] = 7
  board[75] = 2

  //Easy
  // board[2] = 2
  // board[11] = 5
  // board[12] = 8
  // board[14] = 2
  // board[15] = 9
  // board[19] = 8
  // board[21] = 4
  // board[22] = 6
  // board[25] = 3
  // board[27] = 9
  // board[31] = 7
  // board[33] = 2
  // board[35] = 6
  // board[39] = 9
  // board[41] = 1
  // board[55] = 6
  // board[57] = 1
  // board[63] = 1
  // board[64] = 7
  // board[67] = 5
  // board[70] = 4
  // board[71] = 8
  // board[72] = 2
  // board[75] = 3
  // board[76] = 4
  // board[77] = 6
  return board
}